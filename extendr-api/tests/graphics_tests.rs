use extendr_api::prelude::*;
use extendr_api::graphics::{Context, DevDesc};

#[test]
fn graphics_test() {
    test! {
        let dev = DevDesc::current();
        let gc = Context::new();
        dev.setClip(0.0, 0.0, 100.0, 100.0);
        dev.line(0.0, 0.0, 100.0, 100.0, &gc);
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}
