use extendr_api::graphics::{Context, DevDesc, Mode};
use extendr_api::prelude::*;

#[test]
fn graphics_test() {
    let dir = std::env::temp_dir();
    let path = dir.join("test.ps");
    let path_str = path.to_string_lossy().to_string();
    test! {
        R!("postscript({{path_str}})")?;
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
