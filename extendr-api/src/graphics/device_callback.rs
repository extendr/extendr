use crate::*;
use libR_sys::*;

use super::device_descriptor::*;

// This contains the content of the callback functions, which will be called
// from a template callback function. This is needed since
#[repr(C)]
#[derive(Default)]
#[allow(clippy::type_complexity)]
pub(crate) struct DeviceCallbacks {
    pub(crate) activate: Option<fn(arg1: DevDesc)>,
    pub(crate) circle: Option<fn(x: f64, y: f64, r: f64, gc: R_GE_gcontext, dd: DevDesc)>,
    pub(crate) clip: Option<fn(x0: f64, x1: f64, y0: f64, y1: f64, dd: DevDesc)>,
    pub(crate) close: Option<fn(dd: DevDesc)>,
}

impl DeviceCallbacks {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn activate_wrapper(&self) -> Option<unsafe extern "C" fn(arg1: pDevDesc)> {
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
    ) -> Option<unsafe extern "C" fn(x: f64, y: f64, r: f64, gc: pGEcontext, dd: pDevDesc)> {
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

    pub fn clip_wrapper(
        &self,
    ) -> Option<unsafe extern "C" fn(x0: f64, x1: f64, y0: f64, y1: f64, dd: pDevDesc)> {
        // Return None if no callback function is registered.
        self.clip?;

        unsafe extern "C" fn clip_wrapper(x0: f64, x1: f64, y0: f64, y1: f64, dd: pDevDesc) {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let clip_inner = (*data).callbacks.clip.unwrap();

            clip_inner(x0, x1, y0, y1, dev_desc);
        }

        Some(clip_wrapper)
    }

    // Note: close is special. This function is responsible for tearing down the
    // DeviceSpecificData itself, which is always needed even when no close
    // callback is supplied.
    pub fn close_wrapper(&self) -> Option<unsafe extern "C" fn(dd: pDevDesc)> {
        unsafe extern "C" fn close_wrapper(dd: pDevDesc) {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *mut DeviceSpecificData;
            if let Some(close_inner) = (*data).callbacks.close {
                close_inner(dev_desc);
            }

            // Convert back to a Rust struct to drop the resources on Rust's side.
            Box::from_raw(dev_desc.deviceSpecific);
        }

        Some(close_wrapper)
    }
}
