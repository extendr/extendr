use extendr_api::prelude::*;

#[extendr]
fn dbls_named(x: Doubles) -> Doubles {
    let names = x
        .iter()
        .map(|xi| xi.inner().to_string())
        .collect::<Vec<_>>();
    x.set_attrib("names", names).unwrap()
}

#[extendr]
fn strings_named(x: Strings) -> Strings {
    let names = x
        .iter()
        .map(|xi| xi.as_str().to_string())
        .collect::<Vec<_>>();
    x.set_attrib("names", names).unwrap()
}

#[extendr]
fn list_named(x: List, nms: Strings) -> List {
    x.set_attrib("names", nms).unwrap()
}

extendr_module! {
    mod attributes;
    fn dbls_named;
    fn strings_named;
    fn list_named;
}
