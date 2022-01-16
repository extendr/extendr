use crate::*;
use libR_sys::*;

use super::device_descriptor::*;

// This contains the content of the callback functions, which will be called
// from a template callback function. This is needed since
#[repr(C)]
#[derive(Default)]
pub(crate) struct DeviceCallbacks {
    pub(crate) activate: Option<fn(arg1: DevDesc)>,
    pub(crate) circle: Option<fn(x: f64, y: f64, r: f64, gc: R_GE_gcontext, dd: DevDesc)>,
}

impl DeviceCallbacks {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn activate_wrapper(&self) -> Option<unsafe extern "C" fn(pDevDesc)> {
        // Return None if no callback function is registered.
        self.activate?;

        unsafe extern "C" fn activate_wrapper(arg1: pDevDesc) {
            let dev_desc = *arg1;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let activate_inner = (*data).callbacks.activate.unwrap();

            activate_inner(dev_desc);
        }
        Some(activate_wrapper)
    }

    pub fn circle_wrapper(
        &self,
    ) -> Option<unsafe extern "C" fn(f64, f64, f64, pGEcontext, pDevDesc)> {
        // Return None if no callback function is registered.
        self.circle?;

        unsafe extern "C" fn circle_wrapper(x: f64, y: f64, r: f64, gc: pGEcontext, dd: pDevDesc) {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let circle_inner = (*data).callbacks.circle.unwrap();

            let gcontext = *gc;
            circle_inner(x, y, r, gcontext, dev_desc);
        }

        Some(circle_wrapper)
    }
}
