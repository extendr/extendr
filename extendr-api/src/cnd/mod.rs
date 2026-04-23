/// Format a condition message with rlang-style bullets.
///
/// The first element gets a `! ` prefix, additional elements get `• ` prefix.
/// Body lines are appended as-is.
pub fn format_cnd_message(header: &str, body: &[&str]) -> String {
    let mut msg = format!("\n! {header}");
    for line in body {
        msg.push('\n');
        msg.push_str(&format!("\u{2022} {line}"));
    }
    msg
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
        let formatted = $crate::cnd::format_cnd_message($msg, &[]);
        let c_msg = ::std::ffi::CString::new(formatted).unwrap();
        unsafe {
            $crate::Rf_warningcall($crate::R_NilValue, c"%s".as_ptr(), c_msg.as_ptr());
        }
    }};
    ($msg:expr, $body:expr) => {{
        let formatted = $crate::cnd::format_cnd_message($msg, $body);
        let c_msg = ::std::ffi::CString::new(formatted).unwrap();
        unsafe {
            $crate::Rf_warningcall($crate::R_NilValue, c"%s".as_ptr(), c_msg.as_ptr());
        }
    }};
    ($msg:expr, $body:expr, $call:expr) => {{
        let formatted = $crate::cnd::format_cnd_message($msg, $body);
        let c_msg = ::std::ffi::CString::new(formatted).unwrap();
        unsafe {
            $crate::Rf_warningcall($call.get(), c"%s".as_ptr(), c_msg.as_ptr());
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
/// abort!("something went wrong", &["detail 1", "detail 2"], call);
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
        unsafe {
            $crate::Rf_errorcall($call.get(), c"%s".as_ptr(), c_msg.as_ptr());
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_format_cnd_message_no_body() {
        let msg = super::format_cnd_message("something went wrong", &[]);
        assert_eq!(msg, "! something went wrong");
    }

    #[test]
    fn test_format_cnd_message_with_body() {
        let msg = super::format_cnd_message("something went wrong", &["detail 1", "detail 2"]);
        assert_eq!(
            msg,
            "! something went wrong\n\u{2022} detail 1\n\u{2022}
 detail 2"
        );
    }
}
