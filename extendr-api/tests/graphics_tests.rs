use extendr_api::graphics::color::*;
use extendr_api::graphics::{Context, Device, Unit};
use extendr_api::prelude::*;

#[test]
fn graphics_test() {
    // Set this to false to render with your local graphics device.
    let use_postscript = false;
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
        // Use X11 "Alice Blue" https://en.wikipedia.org/wiki/X11_color_names
        gc.fill(antiquewhite());
        dev.new_page(&gc);

        dev.mode_on();

        // Graphics commands.
        gc.color(darkkhaki());
        gc.line_width(0.05);

        // Draw a line.
        dev.line((1.0, 1.0), (2.0, 2.0), &gc);

        // Draw a circle using `polygon()`.
        let scale = std::f64::consts::PI*2.0/10.0;
        gc.fill(deepskyblue());
        dev.polygon(
            (0..10).map(|i| (
                ((i as f64) * scale).cos() + 4.0,
                ((i as f64) * scale).sin() + 2.0
            )), &gc);

        // Draw a circle using `circle()`.
        gc.fill(rgb(0x20, 0x20, 0xc0));
        dev.circle((1.0, 1.0), 0.5, &gc);

        dev.mode_off();

        if use_postscript {
            // Flush the file and kill the device.
            R!("dev.off()")?;
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
        } else {
            std::thread::sleep(std::time::Duration::from_millis(2000));
            assert!(false); // did we check in the flag as "false?"
        }
    }
}
