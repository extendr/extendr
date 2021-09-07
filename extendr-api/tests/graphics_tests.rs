use extendr_api::graphics::{Context, DevDesc, Mode};
use extendr_api::prelude::*;

#[test]
fn graphics_test() {
    // TODO: use tmpfile.
    let path = "/tmp/test.ps";
    test! {
        R!("postscript({{path}})")?;
        let dev = DevDesc::current();
        let gc = Context::new();
        dev.mode(Mode::On);
        dev.line(0.0, 0.0, 100.0, 100.0, &gc);
        R!("dev.off()")?;
    }

    let ps = std::fs::read_to_string(path).unwrap();
    // println!("ps={:?}", ps);
    assert!(ps.contains("100.00 100.00 l\n"));
}
