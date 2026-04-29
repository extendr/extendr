//! # Condition objects
//!
//! In R, a condition is an S3 list with:
//! - `message`: a character vector describing the condition
//! - `call`: the call that triggered the condition, or `NULL`
//! - a class attribute including `"condition"` and optionally `"error"`,
//!   `"warning"`, or `"message"`
//!
//! rlang-style conditions may include additional fields (`trace`, `parent`) which
//! are not represented in [`Condition`] and will be dropped on round-trip.
//!
//! ## Types
//!
//! This module provides four types:
//!
//! - [`ConditionKind`]: an enum discriminating the base R condition type
//!   (`condition`, `message`, `warning`, `error`).
//!
//! - [`Condition`]: a Rust-native representation. Construct one with [`ConditionBuilder::default()`]. This is the primary type we encourage you to work with for its ergonomics due to its use of std types.
//!
//! - [`RCondition`]: a thin wrapper around a [`List`] that already exists in R
//!   memory as a proper condition object. Use this when receiving or returning
//!   condition objects at the R boundary.
//!
//! - [`ConditionBuilder`]: a builder for [`Condition`]. Create a builder with [`ConditionBuilder::default()`] to start, chain setters, and call `.build()` to produce a [`Condition`].
//!
//! ## Conversions
//!
//! | From \ To   | `List`  | `RCondition` | `Robj`  | `Condition` |
//! |-------------|---------|--------------|---------|-------------|
//! | `Condition` | `From`  | `From`       | `From`  | —           |
//! | `List`      | —       | `From`       | blanket | `TryFrom`   |
//! | `&List`     | —       | `From`       | —       | `TryFrom`   |
//! | `RCondition`| `From`  | —            | `From`  | `TryFrom`   |
//! | `Robj`      | —       | `TryFrom`    | —       | `TryFrom`   |
//! | `&Robj`     | —       | `TryFrom`    | —       | `TryFrom`   |
use crate::{
    robj::Rinternals, Attributes, Error, IntoRobj, Language, List, Operators, Robj, Strings,
};

/// Discriminates the kind of R condition being constructed.
#[derive(Copy, Debug, Clone, PartialEq)]
pub enum ConditionKind {
    Condition,
    Message,
    Warning,
    Error,
}

/// Rust-native representation of an R condition.
///
/// Holds Rust types and is converted into an R condition list via
/// `From<Condition> for Robj` or `From<Condition> for RCondition`.
#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    pub message: Vec<String>,
    pub kind: ConditionKind,
    pub class: Option<Vec<String>>,
    pub call: Option<Language>,
}

impl From<Condition> for List {
    fn from(value: Condition) -> Self {
        let msg = Strings::from_values(value.message).into_robj();

        let call_robj = value.call.map(|v| v.into()).unwrap_or(Robj::from(()));

        let mut cnd = List::from_pairs([("message", msg), ("call", call_robj)]);

        let base_classes: &[&str] = match value.kind {
            ConditionKind::Condition => &["condition"],
            ConditionKind::Message => &["message", "condition"],
            ConditionKind::Warning => &["warning", "condition"],
            ConditionKind::Error => &["error", "condition"],
        };

        let mut class = value.class.unwrap_or_default();
        class.extend(base_classes.iter().map(|s| s.to_string()));
        cnd.set_class(&class)
            .expect("set_class on a list should never fail");
        cnd
    }
}

impl From<Condition> for RCondition {
    fn from(value: Condition) -> Self {
        Self(List::from(value))
    }
}

impl From<Condition> for Robj {
    fn from(value: Condition) -> Self {
        Robj::from(List::from(value))
    }
}

impl TryFrom<&List> for Condition {
    type Error = Error;

    fn try_from(list: &List) -> std::result::Result<Self, Self::Error> {
        let message: Vec<String> = Strings::try_from(list.dollar("message")?)?.into();

        let cls: Vec<String> = list
            .class()
            .map(|inner| inner.map(|s| s.to_string()).collect())
            .unwrap_or_default();

        if !cls.iter().any(|c| c == "condition") {
            return Err(Error::Other(
                "object does not inherit from `condition`".into(),
            ));
        }

        let base_kinds = ["error", "warning", "message"];
        let matched: Vec<&str> = base_kinds
            .iter()
            .copied()
            .filter(|k| cls.iter().any(|c| c == k))
            .collect();

        if matched.len() > 1 {
            return Err(Error::Other(format!(
                "ambiguous condition: inherits from multiple base kinds: {}",
                matched.join(", ")
            )));
        }

        let kind = match matched.first().copied() {
            Some("error") => ConditionKind::Error,
            Some("warning") => ConditionKind::Warning,
            Some("message") => ConditionKind::Message,
            _ => ConditionKind::Condition,
        };

        let base_classes: &[&str] = match kind {
            ConditionKind::Condition => &["condition"],
            ConditionKind::Message => &["message", "condition"],
            ConditionKind::Warning => &["warning", "condition"],
            ConditionKind::Error => &["error", "condition"],
        };
        let user_class: Vec<String> = cls
            .iter()
            .filter(|c| !base_classes.contains(&c.as_str()))
            .cloned()
            .collect();
        let class = if user_class.is_empty() {
            None
        } else {
            Some(user_class)
        };

        let call = list.dollar("call").ok().and_then(|v| {
            if v.is_null() {
                None
            } else {
                Language::try_from(&v).ok()
            }
        });

        Ok(Condition {
            message,
            kind,
            class,
            call,
        })
    }
}

impl TryFrom<List> for Condition {
    type Error = Error;

    fn try_from(value: List) -> std::result::Result<Self, Self::Error> {
        Condition::try_from(&value)
    }
}

impl TryFrom<&Robj> for Condition {
    type Error = Error;

    fn try_from(value: &Robj) -> std::result::Result<Self, Self::Error> {
        Condition::try_from(List::try_from(value)?)
    }
}

impl TryFrom<Robj> for Condition {
    type Error = Error;

    fn try_from(value: Robj) -> std::result::Result<Self, Self::Error> {
        Condition::try_from(&value)
    }
}

/// A wrapper around an R condition object (`Robj`).
///
/// This represents an actual R condition list as it exists in R memory.
pub struct RCondition(pub List);

impl From<List> for RCondition {
    fn from(value: List) -> Self {
        RCondition(value)
    }
}

impl From<&List> for RCondition {
    fn from(value: &List) -> Self {
        RCondition(value.clone())
    }
}

impl From<RCondition> for List {
    fn from(value: RCondition) -> Self {
        value.0
    }
}

impl From<RCondition> for Robj {
    fn from(value: RCondition) -> Self {
        Robj::from(value.0)
    }
}

impl TryFrom<RCondition> for Condition {
    type Error = Error;

    fn try_from(value: RCondition) -> std::result::Result<Self, Self::Error> {
        Condition::try_from(value.0)
    }
}

impl TryFrom<&Robj> for RCondition {
    type Error = Error;

    fn try_from(value: &Robj) -> std::result::Result<Self, Self::Error> {
        Ok(RCondition(List::try_from(value)?))
    }
}

impl TryFrom<Robj> for RCondition {
    type Error = Error;

    fn try_from(value: Robj) -> std::result::Result<Self, Self::Error> {
        RCondition::try_from(&value)
    }
}

/// Builder for constructing a [`Condition`].
pub struct ConditionBuilder {
    message: Vec<String>,
    kind: ConditionKind,
    class: Option<Vec<String>>,
    call: Option<Language>,
}

impl ConditionBuilder {
    pub fn set_message(mut self, message: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.message = message.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn set_kind(mut self, kind: ConditionKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn set_class(mut self, class: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.class = Some(class.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn set_call(mut self, call: Language) -> Self {
        self.call = Some(call);
        self
    }

    pub fn build(self) -> Condition {
        Condition {
            message: self.message,
            kind: self.kind,
            class: self.class,
            call: self.call,
        }
    }
}

impl Default for ConditionBuilder {
    fn default() -> Self {
        Self {
            message: Vec::new(),
            kind: ConditionKind::Condition,
            class: None,
            call: None,
        }
    }
}

/// Format an error message with rlang-style bullets.
pub fn format_cnd_message(header: &str, body: &[&str]) -> String {
    use std::io::IsTerminal;
    let use_color = std::io::stderr().is_terminal();
    let bang = if use_color { "\x1b[33m!\x1b[0m" } else { "!" };
    let bullet = if use_color {
        "\x1b[36m\u{2022}\x1b[0m"
    } else {
        "\u{2022}"
    };

    let mut msg = format!("\n{bang} {header}");
    for line in body {
        msg.push('\n');
        msg.push_str(&format!("{bullet} {line}"));
    }
    msg
}

/// Format a warning message with rlang-style bullets.
pub fn format_warn_message(header: &str, body: &[&str]) -> String {
    use std::io::IsTerminal;
    let use_color = std::io::stderr().is_terminal();
    let bullet = if use_color {
        "\x1b[36m\u{2022}\x1b[0m"
    } else {
        "\u{2022}"
    };

    let mut msg = header.to_string();
    for line in body {
        msg.push('\n');
        msg.push_str(&format!("{bullet} {line}"));
    }
    msg
}

/// Signal a warning with an rlang-style formatted message.
#[macro_export]
macro_rules! warn {
    ($msg:expr) => {{
        let formatted = $crate::conditions::format_warn_message($msg, &[]);
        let c_msg = ::std::ffi::CString::new(formatted).unwrap();
        unsafe {
            $crate::Rf_warningcall($crate::R_NilValue, c"%s".as_ptr(), c_msg.as_ptr());
        }
    }};
    ($msg:expr, $body:expr) => {{
        let formatted = $crate::conditions::format_warn_message($msg, $body);
        let c_msg = ::std::ffi::CString::new(formatted).unwrap();
        unsafe {
            $crate::Rf_warningcall($crate::R_NilValue, c"%s".as_ptr(), c_msg.as_ptr());
        }
    }};
    ($msg:expr, $body:expr, $call:expr) => {{
        let formatted = $crate::conditions::format_warn_message($msg, $body);
        let c_msg = ::std::ffi::CString::new(formatted).unwrap();
        let call_robj = $call.call();
        unsafe {
            let call_sexp = call_robj
                .as_ref()
                .map(|c| c.get())
                .unwrap_or($crate::R_NilValue);
            $crate::Rf_warningcall(call_sexp, c"%s".as_ptr(), c_msg.as_ptr());
        }
    }};
}

/// Signal an error with an rlang-style formatted message.
#[macro_export]
macro_rules! abort {
    ($msg:expr) => {{
        let formatted = $crate::conditions::format_cnd_message($msg, &[]);
        let c_msg = ::std::ffi::CString::new(formatted).unwrap();
        unsafe {
            $crate::Rf_errorcall($crate::R_NilValue, c"%s".as_ptr(), c_msg.as_ptr());
        }
    }};

    ($msg:expr, call = $call:expr) => {{
        let formatted = $crate::conditions::format_cnd_message($msg, &[]);
        let c_msg = ::std::ffi::CString::new(formatted).unwrap();
        let call_robj = $call.call();
        unsafe {
            let call_sexp = call_robj
                .as_ref()
                .map(|c| c.get())
                .unwrap_or($crate::R_NilValue);
            $crate::Rf_errorcall(call_sexp, c"%s".as_ptr(), c_msg.as_ptr());
        }
    }};

    ($msg:expr, $body:expr) => {{
        let formatted = $crate::conditions::format_cnd_message($msg, $body);
        let c_msg = ::std::ffi::CString::new(formatted).unwrap();
        unsafe {
            $crate::Rf_errorcall($crate::R_NilValue, c"%s".as_ptr(), c_msg.as_ptr());
        }
    }};

    ($msg:expr, $body:expr, $call:expr) => {{
        let formatted = $crate::conditions::format_cnd_message($msg, $body);
        let c_msg = ::std::ffi::CString::new(formatted).unwrap();
        let call_robj = $call.call();
        unsafe {
            let call_sexp = call_robj
                .as_ref()
                .map(|c| c.get())
                .unwrap_or($crate::R_NilValue);
            $crate::Rf_errorcall(call_sexp, c"%s".as_ptr(), c_msg.as_ptr());
        }
    }};
}

#[cfg(test)]
mod tests {
    use extendr_engine::with_r;

    use crate::{
        conditions::{Condition, ConditionBuilder, ConditionKind, RCondition},
        Attributes, List, Result, Robj,
    };

    #[test]
    fn test_format_cnd_message_no_body() {
        let msg = super::format_cnd_message("something went wrong", &[]);
        assert!(msg.contains("something went wrong"));
    }

    #[test]
    fn test_format_cnd_message_with_body() {
        let msg = super::format_cnd_message("something went wrong", &["detail 1", "detail 2"]);
        assert!(msg.contains("something went wrong"));
        assert!(msg.contains("detail 1"));
        assert!(msg.contains("detail 2"));
    }

    #[test]
    fn roundtrip_with_class() -> Result<()> {
        with_r(|| {
            let cnd = ConditionBuilder::default()
                .set_kind(ConditionKind::Message)
                .set_message(["this is a custom message"])
                .set_class(["class1", "class2"])
                .build();
            let c2 = Condition::try_from(RCondition::from(cnd.clone()))?;
            assert_eq!(cnd, c2);
            Ok(())
        })
    }

    #[test]
    fn roundtrip_no_class() -> Result<()> {
        with_r(|| {
            let cnd = ConditionBuilder::default()
                .set_kind(ConditionKind::Warning)
                .set_message(["watch out"])
                .build();
            let c2 = Condition::try_from(RCondition::from(cnd.clone()))?;
            assert_eq!(cnd, c2);
            Ok(())
        })
    }

    #[test]
    fn roundtrip_via_robj() -> Result<()> {
        with_r(|| {
            let cnd = ConditionBuilder::default()
                .set_kind(ConditionKind::Error)
                .set_message(["something failed"])
                .set_class(["my_error"])
                .build();
            let robj = Robj::from(cnd.clone());
            let c2 = Condition::try_from(robj)?;
            assert_eq!(cnd, c2);
            Ok(())
        })
    }

    #[test]
    fn roundtrip_all_kinds() -> Result<()> {
        with_r(|| {
            for (kind, expected_base) in [
                (ConditionKind::Condition, "condition"),
                (ConditionKind::Message, "message"),
                (ConditionKind::Warning, "warning"),
                (ConditionKind::Error, "error"),
            ] {
                let cnd = ConditionBuilder::default()
                    .set_kind(kind)
                    .set_message(["msg"])
                    .build();
                let list = List::from(cnd);
                let cls: Vec<_> = list.class().unwrap().collect();
                assert!(cls.contains(&expected_base), "missing {expected_base}");
                assert!(cls.contains(&"condition"), "missing condition");
            }
            Ok(())
        })
    }

    #[test]
    fn err_not_a_condition() -> Result<()> {
        with_r(|| {
            let list = List::from_pairs([("message", Robj::from("oops"))]);
            let result = Condition::try_from(&list);
            assert!(result.is_err());
            Ok(())
        })
    }

    #[test]
    fn err_not_a_list() -> Result<()> {
        with_r(|| {
            let robj = Robj::from("just a string");
            let result = Condition::try_from(&robj);
            assert!(result.is_err());
            Ok(())
        })
    }

    #[test]
    fn err_ambiguous_kind() -> Result<()> {
        with_r(|| {
            let mut list =
                List::from_pairs([("message", Robj::from("oops")), ("call", Robj::from(()))]);
            list.set_class(&["error", "warning", "condition"]).unwrap();
            let result = Condition::try_from(&list);
            assert!(result.is_err());
            Ok(())
        })
    }

    #[test]
    fn roundtrip_rcondition_to_condition() -> Result<()> {
        with_r(|| {
            let cnd = ConditionBuilder::default()
                .set_kind(ConditionKind::Error)
                .set_message(["bad thing"])
                .set_class(["my_err"])
                .build();
            let rcnd = RCondition::from(cnd.clone());
            let c2 = Condition::try_from(rcnd)?;
            assert_eq!(cnd, c2);
            Ok(())
        })
    }
}
