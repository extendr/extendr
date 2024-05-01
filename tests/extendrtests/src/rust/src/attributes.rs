use extendr_api::prelude::*;

/// Adds an attribute to a vector of doubles
/// 
/// @param x Vector of doubles 
/// 
/// @export
#[extendr]
fn dbls_named(x: Doubles) -> Doubles {
    let mut x = x;
    x.set_attrib(
        "names",
        x.iter()
            .map(|xi| xi.inner().to_string())
            .collect::<Vec<_>>(),
    )
    .unwrap();

    x
}

#[extendr]
fn strings_named(x: Strings) -> Strings {
    let mut x = x;
    x.set_attrib(
        "names",
        x.iter()
            .map(|xi| xi.as_str().to_string())
            .collect::<Vec<_>>(),
    )
    .unwrap();
    x
}

#[extendr]
fn list_named(x: List, nms: Strings) -> List {
    let mut x = x;
    let _ = x.set_attrib("names", nms);
    x
}

extendr_module! {
    mod attributes;
    fn dbls_named;
    fn strings_named;
    fn list_named;
}
