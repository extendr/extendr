use extendr_api::prelude::*;
use extendr_ffi::ParseStatus;

#[test]
fn parse_reports_error_for_invalid_syntax() {
    test! {
        let err = parse("c(10,,42,20)").unwrap_err();
        let msg = err.to_string();
        match err {
            Error::ParseError {
                status: ParseStatus::PARSE_ERROR,
                ..
            } => assert!(msg.contains("syntax error")),
            other => panic!("Unexpected error variant: {other:?}"),
        }
    }
}

#[test]
fn parse_reports_error_for_incomplete_input() {
    test! {
        let err = parse("c(10").unwrap_err();
        let msg = err.to_string();
        match err {
            Error::ParseError {
                status: ParseStatus::PARSE_INCOMPLETE,
                ..
            } => assert!(msg.contains("incomplete statement")),
            other => panic!("Unexpected error variant: {other:?}"),
        }
    }
}

#[test]
fn eval_string_returns_eval_error_for_missing_argument() {
    test! {
        let result = eval_string("c(10,,42,20)");
        assert!(matches!(result, Err(Error::EvalError(_))));
    }
}
