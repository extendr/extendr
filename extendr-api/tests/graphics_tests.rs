use extendr_api::graphics::{rgb, Context, Device, Unit};
use extendr_api::prelude::*;

#[test]
fn graphics_test() {
    let use_postscript = true;
    let dir = std::env::temp_dir();
    let path = dir.join("test.ps");
    let path_str = path.to_string_lossy().to_string();
    test! {
        if use_postscript {
            R!("postscript({{path_str}})")?;
        }
        let dev = Device::current();
        let mut gc = Context::from_device(&dev, Unit::Inches);

        // Start a new page.
        gc.fill(rgb(0xc0, 0xc0, 0xc0));
        dev.new_page(&gc);
        // dev.setClip(0., 0., 10., 10.);

        // Graphics commands.
        gc.color(rgb(0x40, 0x40, 0x40));
        gc.line_width(0.05);

        // Draw a line.
        dev.line((1.0, 1.0), (2.0, 2.0), &gc);

        // Draw a circle using `polygon()`.
        let scale = std::f64::consts::PI*2.0/10.0;
        gc.fill(rgb(0xc0, 0xff, 0xc0));
        dev.polygon(
            (0..10).map(|i| (
                ((i as f64) * scale).cos() + 4.0,
                ((i as f64) * scale).sin() + 2.0
            )), &gc);

        // Draw a circle using `circle()`.
        gc.fill(rgb(0x80, 0xff, 0x80));
        dev.circle((1.0, 3.0), 0.5, &gc);

        if use_postscript {
            R!("dev.off()")?;
        } else {
            std::thread::sleep(std::time::Duration::from_millis(2000));
        }
    }

    let ps = std::fs::read_to_string(path).expect("PS file not written.");
    if let Some(split) = ps.split_once("%%EndProlog") {
        let epilogue = split.1;
        println!("epilogue:\n{}", epilogue);

        // Graphics commands.
        // Note windows version uses \r\n, hence the newline at the start.
        // assert!(epilogue.contains("\no"));
        // assert!(epilogue.contains("\ncp p3"));
        // assert!(epilogue.contains("\nc p3"));
    } else {
        println!("ps:\n{}", ps);
        assert!(("epilogue not found", false).1);
    }
}
