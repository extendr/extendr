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
}
