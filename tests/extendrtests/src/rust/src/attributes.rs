use extendr_api::prelude::*;

#[extendr]
fn dbls_named(mut x: Doubles) -> Doubles {
    x.set_attrib(
        "names",
        x.iter().map(|xi| xi.0.to_string()).collect::<Vec<_>>(),
    )
    .unwrap();

    x
}

#[extendr]
fn strings_named(mut x: Strings) -> Strings {
    x.set_attrib(
        "names",
        x.iter().map(|xi| xi.to_string()).collect::<Vec<_>>(),
    )
    .unwrap();
    x
}

#[extendr]
fn list_named(mut x: List, nms: Strings) -> List {
    let _ = x.set_attrib("names", nms);
    x
}

extendr_module! {
    mod attributes;
    fn dbls_named;
    fn strings_named;
    fn list_named;
}
