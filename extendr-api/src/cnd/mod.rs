use crate::robj::Rinternals;
use crate::wrapper::environment::Environment;
use crate::{append, append_lang, make_lang, Robj};

/// Format an error message with rlang-style bullets.
///
/// The header gets a `! ` prefix, additional elements get `• ` prefix.
/// A leading `\n` is included so the message appears on a new line after
/// `Error in foo():` or `Error:`.
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
///
/// No `!` prefix — just the header and `• ` bullets.
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

/// Extract a call label from an environment for use in error prefixes.
/// Returns a string like "`my_function()`" or `None` if unavailable.
pub fn call_label(env: &Environment) -> Option<String> {
    use crate::robj::Eval;
    // Get sys.call() in the environment
    let call = crate::lang!("sys.call").eval_with_env(env).ok()?;
    if call.is_null() {
        return None;
    }
    // Deparse call[[1]] to get just the function name
    let name = crate::lang!("deparse", crate::lang!("[[", call, 1))
        .eval_with_env(env)
        .ok()?;
    let name_str = name.as_str()?;
    Some(format!("`{name_str}()`"))
}

/// Signal a warning with an rlang-style formatted message.
///
/// # Examples
///
/// ```rust,ignore
/// warn!("something went wrong");
/// warn!("something went wrong", &["detail 1", "detail 2"]);
/// warn!("something went wrong", &["detail 1", "detail 2"], call);
/// ```
#[macro_export]
macro_rules! warn {
    ($msg:expr) => {{
        let formatted = $crate::cnd::format_warn_message($msg, &[]);
        let c_msg = ::std::ffi::CString::new(formatted).unwrap();
        unsafe {
            $crate::Rf_warningcall($crate::R_NilValue, c"%s".as_ptr(), c_msg.as_ptr());
        }
    }};
    ($msg:expr, $body:expr) => {{
        let formatted = $crate::cnd::format_warn_message($msg, $body);
        let c_msg = ::std::ffi::CString::new(formatted).unwrap();
        unsafe {
            $crate::Rf_warningcall($crate::R_NilValue, c"%s".as_ptr(), c_msg.as_ptr());
        }
    }};
    ($msg:expr, $body:expr, $call:expr) => {{
        let formatted = $crate::cnd::format_warn_message($msg, $body);
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
///
/// # Examples
///
/// ```rust,ignore
/// abort!("something went wrong");
/// abort!("something went wrong", &["detail 1", "detail 2"]);
/// abort!("something went wrong", &["detail 1", "detail 2"], Environment::caller());
/// ```
#[macro_export]
macro_rules! abort {
    ($msg:expr) => {{
        let formatted = $crate::cnd::format_cnd_message($msg, &[]);
        let c_msg = ::std::ffi::CString::new(formatted).unwrap();
        unsafe {
            $crate::Rf_errorcall($crate::R_NilValue, c"%s".as_ptr(), c_msg.as_ptr());
        }
    }};

    ($msg:expr, call = $call:expr) => {{
        let formatted = $crate::cnd::format_cnd_message($msg, &[]);
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
        let formatted = $crate::cnd::format_cnd_message($msg, $body);
        let c_msg = ::std::ffi::CString::new(formatted).unwrap();
        unsafe {
            $crate::Rf_errorcall($crate::R_NilValue, c"%s".as_ptr(), c_msg.as_ptr());
        }
    }};

    ($msg:expr, $body:expr, $call:expr) => {{
        let formatted = $crate::cnd::format_cnd_message($msg, $body);
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

    #[test]
    fn test_format_cnd_message_no_body() {
        let msg = super::format_cnd_message("something went wrong", &[]);
        assert_eq!(msg, "\n! something went wrong");
    }

    #[test]
    fn test_format_cnd_message_with_body() {
        let msg = super::format_cnd_message("something went wrong", &["detail 1", "detail 2"]);
        assert_eq!(
            msg,
            "\n! something went wrong\n\u{2022} detail 1\n\u{2022} detail 2"
        );
    }
}
