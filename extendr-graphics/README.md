# extendr-graphics

An interface to R's graphics-related features.

## Example

``` rust
use extendr_api::prelude::*;
use extendr_graphics::{DeviceDescriptor, DeviceDriver, DevDesc};

struct MyDevice<'a> {
    welcome_message: &'a str,
}

impl<'a> DeviceDriver for MyDevice<'a> {
    fn activate(&mut self, _dd: DevDesc) {
        let welcome_message = self.welcome_message;
        rprintln!("message from device: {welcome_message}");
    }
}

/// Create a new device.
///
/// @export
#[extendr]
fn my_device(welcome_message: String) {
    let device_driver = MyDevice {
        welcome_message: welcome_message.as_str(),
    };
    
    let device_descriptor = DeviceDescriptor::new();
    let device = device_driver.create_device::<MyDevice>(device_descriptor, "my device");
}
```

This can be called from R.

``` r
my_device("I'm so active!!!")
#> message from device: I'm so active!!!
```

## Resources for developers

Graphic device is documented in the R-internals. The header file also contains
the useful information. The code of the graphics package is also useful to see
what values are used by default (i.e. `GInit`).

- https://cran.r-project.org/doc/manuals/r-devel/R-ints.html
- https://github.com/wch/r-source/blob/trunk/src/include/R_ext/GraphicsDevice.h
- https://github.com/wch/r-source/blob/trunk/src/library/graphics/src/graphics.c

While the documents are good, sometimes we need to refer to the real
implementaions to find hints.

- postscript device: https://github.com/wch/r-source/blob/trunk/src/library/grDevices/src/devPS.c
- svglite package: https://github.com/r-lib/svglite/blob/main/src/devSVG.cpp
- devout package: https://github.com/coolbutuseless/devout/blob/master/src/rdevice.cpp

For newer features, the blog posts by Paul Murrell might be helpful:

- https://developer.r-project.org/Blog/public/2020/07/15/new-features-in-the-r-graphics-engine/index.html
- https://developer.r-project.org/Blog/public/2021/12/06/groups-and-paths-and-masks-in-r-graphics/index.html
- https://developer.r-project.org/Blog/public/2021/12/14/updating-graphics-devices-for-r-4.2.0/index.html
