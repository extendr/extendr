use extendr_api::*;

#[extendr]
fn data() -> &'static str {
    "data"
}

// Macro to generate exports
extendr_module! {
    mod data;
    fn data;
}
