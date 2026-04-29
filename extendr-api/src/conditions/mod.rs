use crate::{
    AsStrIter, Attributes, Environment, Error, IntoRobj, Language, List, Operators, Result, Robj,
    Rstr, StrIter, Strings,
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
            ConditionKind::Condition => &["simpleCondition", "condition"],
            ConditionKind::Message => &["simpleMessage", "message", "condition"],
            ConditionKind::Warning => &["simpleWarning", "warning", "condition"],
            ConditionKind::Error => &["simpleError", "error", "condition"],
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

impl TryFrom<&Robj> for Condition {
    type Error = Error;

    fn try_from(value: &Robj) -> std::result::Result<Self, Self::Error> {
        let list = List::try_from(value)?;

        let message = Strings::try_from(list.dollar("message")?)?.into();

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

        // Strip the base classes to recover user-defined prefix classes
        let base_classes: &[&str] = match kind {
            ConditionKind::Condition => &["simpleCondition", "condition"],
            ConditionKind::Message => &["simpleMessage", "message", "condition"],
            ConditionKind::Warning => &["simpleWarning", "warning", "condition"],
            ConditionKind::Error => &["simpleError", "error", "condition"],
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

        let call = list
            .dollar("call")
            .ok()
            .and_then(|v| Language::try_from(&v).ok());

        Ok(Condition {
            message,
            kind,
            class,
            call,
        })
    }
}

/// A wrapper around an R condition object (`Robj`).
///
/// This represents an actual R condition list as it exists in R memory.
pub struct RCondition(pub List);

/// Builder for constructing a [`Condition`].
pub struct ConditionBuilder {
    message: Vec<String>,
    kind: ConditionKind,
    class: Option<Vec<String>>,
    call: Option<Language>,
}

impl ConditionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

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
    use crate::conditions::{ConditionBuilder, RCondition};

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
    fn test_cnd() {
        let cnd = ConditionBuilder::new()
            .set_kind(super::ConditionKind::Message)
            .set_message(["this is a custom message"])
            .set_class(["class1", "class2"])
            .build();

        let rcnd = RCondition::from(cnd.clone());
    }
}
