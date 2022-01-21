use extendr_api::{graphics::*, prelude::*};

struct MyDevice<'a> {
    welcome_message: &'a str,
}

impl<'a> DeviceDriver for MyDevice<'a> {
    fn activate(&mut self, _dd: DevDesc) {
        let welcome_message = self.welcome_message;
        rprintln!("message from device: {welcome_message}");
    }

    fn close(&mut self, _dd: DevDesc) {
        rprintln!("good bye...");
    }
}
