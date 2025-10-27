use extendr_api::prelude::*;

#[extendr]
fn error_simple() -> Result<i32> {
    Err(Error::Other("This is a simple error message".to_string()))
}

#[extendr]
fn error_parse_int(s: &str) -> Result<i32> {
    let parsed: i32 = s.parse().map_err(|e| Error::Other(format!("{e}")))?;
    Ok(parsed)
}

#[extendr]
fn error_success() -> Result<i32> {
    Ok(42)
}

#[extendr]
fn error_division(numerator: f64, denominator: f64) -> Result<f64> {
    if denominator == 0.0 {
        Err(Error::Other("Division by zero is not allowed".to_string()))
    } else {
        Ok(numerator / denominator)
    }
}

// this function illustrates that we can chain errors together and not get a panic
#[extendr]
fn error_chain(s: &str) -> Result<f64> {
    let num: i32 = s
        .parse()
        .map_err(|e| Error::Other(format!("Parse error: {}", e)))?;
    if num < 0 {
        Err(Error::Other("Negative numbers not allowed".to_string()))
    } else {
        Ok(num as f64)
    }
}

#[extendr]
fn error_long_message() -> Result<()> {
    Err(Error::Other(
        "This is a longer error message that describes in detail what went wrong. \
         It includes multiple sentences and provides context about the failure. \
         This helps test how longer error messages are displayed to users."
            .to_string(),
    ))
}

extendr_module! {
    mod errors;
    fn error_simple;
    fn error_parse_int;
    fn error_success;
    fn error_division;
    fn error_chain;
    fn error_long_message;
}
