use extendr_api::graphics::{Context, Device, rgb};
use extendr_api::prelude::*;

#[test]
fn graphics_test() {
    let dir = std::env::temp_dir();
    let path = dir.join("test.ps");
    let path_str = path.to_string_lossy().to_string();
    test! {
        R!("postscript({{path_str}})")?;
        let dev = Device::current();
        let mut gc = Context::new();

        // Graphics commands.
        gc.color(rgb(0xff, 0x00, 0x00));
        gc.line_width(10.0);
        dev.line(0.0, 0.0, 100.0, 100.0, &gc);
        R!("dev.off()")?;
    }

    let ps = std::fs::read_to_string(path).expect("PS file not written.");
    if let Some(split) = ps.split_once("%%EndProlog\n") {
        let epilogue = split.1;
        println!("epilogue:\n{}", epilogue);
    
        // Graphics commands.
        assert!(ps.contains("1 0 0 srgb\n"));
        assert!(ps.contains("100.00 100.00 l\n"));
    
    } else {
        println!("ps:\n{}", ps);
        assert!(false);
    }
}
