#[cfg(feature = "graphics")]
mod graphics_tests {
    use std::fmt::Write;

    use extendr_api::graphics::color::predefined::{antiquewhite, black, darkkhaki, deepskyblue};
    use extendr_api::graphics::color::Color;
    use extendr_api::graphics::{
        Context, DevDesc, Device, DeviceDescriptor, DeviceDriver, FontFace, R_GE_gcontext, Raster,
        TextMetric, Unit,
    };
    use extendr_api::prelude::*;

    #[test]
    fn graphics_test() {
        // Set this to false to render with your local graphics device.
        let use_postscript = true;
        let dir = std::env::temp_dir();
        let path = dir.join("test.ps");
        let path_str = path.to_string_lossy().to_string();
        test! {
            if use_postscript {
                R!("postscript({{path_str}})")?;
            }
            let dev = Device::current()?;
            let mut gc = Context::from_device(&dev, Unit::Inches);

            // Start a new page.
            // Use CSS "antiquewhite" https://www.w3.org/TR/2018/REC-css-color-3-20180619/
            gc.fill(antiquewhite());
            dev.new_page(&gc);

            dev.mode_on()?;

            // Graphics commands.
            gc.color(darkkhaki());
            gc.line_width(0.1);

            // Draw a line.
            dev.line((2.0, 2.0), (3.0, 3.0), &gc);

            // Draw a circle using `polygon()`.
            let scale = std::f64::consts::PI*2.0/10.0;
            gc.fill(deepskyblue());
            dev.polygon(
                (0..10).map(|i| (
                    ((i as f64) * scale).cos() + 4.0,
                    ((i as f64) * scale).sin() + 2.0
                )), &gc);

            // Draw a circle using `circle()`.
            gc.fill(Color::rgb(0x20, 0x20, 0xc0));
            dev.circle((1.0, 1.0), 0.5, &gc);

            gc.color(black());
            gc.point_size(36.0);
            gc.font_face(FontFace::Plain);
            gc.font_family("Helvetica");

            // Draw Hello -- World with the two dashes almost touching.
            let w = dev.text_width("Hello -", &gc);
            dev.text((1.0, 3.0), "Hello -", (0.0, 0.0), 0.0, &gc);
            dev.text((1.0 + w, 3.0), "- World", (0.0, 0.0), 0.0, &gc);

            gc.line_width(0.01);
            for i in 0..10 {
                dev.symbol((1.0 + i as f64 * 0.3, 4.0), i, 0.25, &gc);
            }

            println!("{:?}", dev.char_metric('a', &gc));
            println!("{:?}", dev.char_metric('g', &gc));
            println!("{:?}", dev.char_metric('ê', &gc));

            println!("{:?}", dev.text_metric("a", &gc));
            println!("{:?}", dev.text_metric("g", &gc));
            println!("{:?}", dev.text_metric("ê", &gc));

            dev.mode_off()?;

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

    // Taking the mutable references so that we can peek at the values from
    // outside, which is (only?) useful for testing.
    struct TestDevice<'a> {
        last_mode: &'a mut i32,
        value: &'a mut f64,
        canvas: &'a mut String,
        closed: &'a mut bool,
    }

    impl<'a> DeviceDriver for TestDevice<'a> {
        fn activate(&mut self, _: DevDesc) {
            *self.value = 100.0;
        }

        fn mode(&mut self, mode: i32, _: DevDesc) {
            *self.last_mode = mode;
            *self.value += 1.0;
        }

        fn close(&mut self, _: DevDesc) {
            *self.closed = true;
        }

        fn new_page(&mut self, _: R_GE_gcontext, _: DevDesc) {
            self.canvas.clear();
        }

        fn clip(&mut self, from: (f64, f64), to: (f64, f64), _dd: DevDesc) {
            let (f_x, f_y) = from;
            let (t_x, t_y) = to;
            writeln!(
                *self.canvas,
                "clip from=({f_x:.1}, {f_y:.1}) to=({t_x:.1}, {t_y:.1})"
            )
            .unwrap();
        }

        fn circle(&mut self, center: (f64, f64), r: f64, _: R_GE_gcontext, _: DevDesc) {
            let (x, y) = center;
            writeln!(*self.canvas, "circle center=({x:.1}, {y:.1}) r={r:.1}").unwrap();
        }

        fn line(&mut self, from: (f64, f64), to: (f64, f64), _: R_GE_gcontext, _: DevDesc) {
            let (f_x, f_y) = from;
            let (t_x, t_y) = to;
            writeln!(
                *self.canvas,
                "line from=({f_x:.1}, {f_y:.1}) to=({t_x:.1}, {t_y:.1})"
            )
            .unwrap();
        }

        fn rect(&mut self, from: (f64, f64), to: (f64, f64), _: R_GE_gcontext, _: DevDesc) {
            let (f_x, f_y) = from;
            let (t_x, t_y) = to;
            writeln!(
                *self.canvas,
                "rect from=({f_x:.1}, {f_y:.1}) to=({t_x:.1}, {t_y:.1})"
            )
            .unwrap();
        }

        fn polyline<T: IntoIterator<Item = (f64, f64)>>(
            &mut self,
            coords: T,
            _: R_GE_gcontext,
            _: DevDesc,
        ) {
            let coords = coords
                .into_iter()
                .map(|(x, y)| format!("({x:.1}, {y:.1})"))
                .collect::<Vec<String>>()
                .join(" ");
            writeln!(*self.canvas, "polyline coords=[{coords}]").unwrap();
        }

        fn polygon<T: IntoIterator<Item = (f64, f64)>>(
            &mut self,
            coords: T,
            _: R_GE_gcontext,
            _: DevDesc,
        ) {
            let coords = coords
                .into_iter()
                .map(|(x, y)| format!("({x:.1}, {y:.1})"))
                .collect::<Vec<String>>()
                .join(" ");
            writeln!(*self.canvas, "polygon coords=[{coords}]").unwrap();
        }

        fn path<T: IntoIterator<Item = impl IntoIterator<Item = (f64, f64)>>>(
            &mut self,
            coords: T,
            winding: bool,
            _: R_GE_gcontext,
            _: DevDesc,
        ) {
            let coords = coords
                .into_iter()
                .map(|i| {
                    let xy = i
                        .into_iter()
                        .map(|(x, y)| format!("({x:.1}, {y:.1})"))
                        .collect::<Vec<String>>()
                        .join(" ");
                    format!("({xy})")
                })
                .collect::<Vec<String>>()
                .join(", ");

            writeln!(*self.canvas, "path coords=[{coords}] winding={winding}").unwrap();
        }

        fn text(
            &mut self,
            pos: (f64, f64),
            str: &str,
            rot: f64,
            hadj: f64,
            _: R_GE_gcontext,
            _: DevDesc,
        ) {
            let (x, y) = pos;
            writeln!(
                *self.canvas,
                "text pos=({x:.1}, {y:.1}) str='{str}' rot={rot:.1} hadj={hadj:.1}"
            )
            .unwrap();
        }

        // returns the char code in width so that we can check it
        fn char_metric(&mut self, c: char, _: R_GE_gcontext, _: DevDesc) -> TextMetric {
            TextMetric {
                ascent: 0.0,
                descent: 0.0,
                width: c as i32 as _,
            }
        }

        fn raster<T: AsRef<[u32]>>(
            &mut self,
            raster: Raster<T>,
            pos: (f64, f64),
            size: (f64, f64),
            rot: f64,
            interpolate: bool,
            _: R_GE_gcontext,
            _: DevDesc,
        ) {
            let (x, y) = pos;
            let (width, height) = size;
            let Raster { pixels, width: w } = raster;
            let pixels_str = pixels
                .as_ref()
                .iter()
                .map(|&p| format!("{p}"))
                .collect::<Vec<String>>()
                .join("|");

            writeln!(
                *self.canvas,
                "raster {pixels_str} w={w} pos=({x:.1}, {y:.1}) size=({width:.1}, {height:.1}) rot={rot:.1} interpolate={interpolate}"
            )
            .unwrap();
        }
    }

    #[test]
    fn device_driver_test() {
        test! {
            let mut value = 0.0;
            let mut last_mode = 0;
            let mut closed = false;
            let mut canvas = String::new();

            let device_driver = TestDevice {
                last_mode: &mut last_mode,
                value: &mut value,
                canvas: &mut canvas,
                closed: &mut closed,
            };

            let device_descriptor = DeviceDescriptor::new();
            let device = device_driver.create_device::<TestDevice>(device_descriptor, "test device");

            let gc = Context::from_device(&device, Unit::Device);

            // if activate() is invoked, value should be 100.0
            assert_eq!(value, 100.0);
            assert!(!closed);

            device.mode_on().unwrap();

            // if mode() is invoked, value and last_mode should be updated
            assert_eq!(last_mode, 1);
            assert_eq!(value, 101.0);

            device.mode_off().unwrap();

            // if mode() is invoked, value and last_mode should be updated
            assert_eq!(last_mode, 0);
            assert_eq!(value, 102.0);

            // ASCII char
            let c1 = 'c';
            assert_eq!(device.char_metric(c1, &gc).width, c1 as i32 as f64);
            // non-ASCII char
            let c2 = 'ú';
            let c3 = '鬼';
            assert_eq!(device.char_metric(c2, &gc).width, c2 as i32 as f64);
            assert_eq!(device.char_metric(c3, &gc).width, c3 as i32 as f64);
            // text
            let t1 = "ab";
            assert_eq!(device.text_width(t1, &gc), ('a' as i32 + 'b' as i32) as f64);

            device.clip((1.1, 2.2), (3.3, 4.4), &gc);
            device.circle((1.1, 2.2), 3.3, &gc);
            device.line((1.1, 2.2), (3.3, 4.4), &gc);
            device.rect((1.1, 2.2), (3.3, 4.4), &gc);

            device.polyline([(0.0, 0.0), (0.0, 2.0)], &gc);
            device.polygon([(0.0, 0.0), (1.0, 2.0), (2.0, 0.0)], &gc);
            device.path([[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], [(0.3, 0.0), (0.3, 0.3), (0.7, 0.3), (0.3, 0.7)]], true, &gc);

            // x element of `center` is `hadj`, a horizontal adjustment
            // I'm yet to figure out how the `y` element is used. Let's leave it as 0 for now.
            device.text((1.1, 2.2), "foo", (0.5, 0.0), 5.5, &gc);

            let r = Raster {
                pixels: &[1, 2, 3, 4, 5, 6],
                width: 3,
            };
            device.raster(r, (1.1, 2.2), (3.3, 4.4), 5.5, false, &gc);

            assert_eq!(canvas, "clip from=(1.1, 2.2) to=(3.3, 4.4)\n\
                                    circle center=(1.1, 2.2) r=3.3\n\
                                    line from=(1.1, 2.2) to=(3.3, 4.4)\n\
                                    rect from=(1.1, 2.2) to=(3.3, 4.4)\n\
                                    polyline coords=[(0.0, 0.0) (0.0, 2.0)]\n\
                                    polygon coords=[(0.0, 0.0) (1.0, 2.0) (2.0, 0.0)]\n\
                                    path coords=[((0.0, 0.0) (1.0, 0.0) (1.0, 1.0) (0.0, 1.0)), ((0.3, 0.0) (0.3, 0.3) (0.7, 0.3) (0.3, 0.7))] winding=true\n\
                                    text pos=(1.1, 2.2) str='foo' rot=5.5 hadj=0.5\n\
                                    raster 1|2|3|4|5|6 w=3 pos=(1.1, 2.2) size=(3.3, 4.4) rot=5.5 interpolate=false\n\
                                    ");

            // Clearing canvas.
            device.new_page(&gc);
            assert_eq!(canvas, "");

            // check if the R doesn't crash on closing the device.
            R!("dev.off()")?;

            assert!(closed);
        }
    }
}
