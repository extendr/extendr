use crate::*;
use libR_sys::*;

use super::device_descriptor::*;

// This contains the content of the callback functions, which will be called
// from a template callback function. This is needed since
#[repr(C)]
#[derive(Default)]
pub(crate) struct DeviceCallbacks {
    pub(crate) activate: Option<fn(arg1: DevDesc)>,
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
            let activate_fun = (*data).callbacks.activate.unwrap();
            activate_fun(dev_desc);
        }
        Some(activate_wrapper)
    }
}
