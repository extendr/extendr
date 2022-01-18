use core::slice;

use crate::*;
use libR_sys::*;

use super::device_descriptor::*;

// This contains the content of the callback functions, which will be called
// from a template callback function. This is needed since
#[repr(C)]
#[derive(Default)]
#[allow(clippy::type_complexity, non_snake_case)]
pub(crate) struct DeviceCallbacks {
    pub(crate) activate: Option<fn(arg1: DevDesc)>,
    pub(crate) circle: Option<fn(x: f64, y: f64, r: f64, gc: R_GE_gcontext, dd: DevDesc)>,
    pub(crate) clip: Option<fn(x0: f64, x1: f64, y0: f64, y1: f64, dd: DevDesc)>,
    pub(crate) close: Option<fn(dd: DevDesc)>,
    pub(crate) deactivate: Option<fn(arg1: DevDesc)>,
    pub(crate) line: Option<fn(x1: f64, y1: f64, x2: f64, y2: f64, gc: R_GE_gcontext, dd: DevDesc)>,
    pub(crate) metricInfo: Option<
        fn(
            c: i32,
            gc: R_GE_gcontext,
            ascent: *mut f64,
            descent: *mut f64,
            width: *mut f64,
            dd: DevDesc,
        ),
    >,
    pub(crate) mode: Option<fn(mode: i32, dd: DevDesc)>,
    pub(crate) newPage: Option<fn(gc: R_GE_gcontext, dd: DevDesc)>,
    pub(crate) polygon: Option<fn(x: &[f64], y: &[f64], gc: R_GE_gcontext, dd: DevDesc)>,
    pub(crate) polyline: Option<fn(x: &[f64], y: &[f64], gc: R_GE_gcontext, dd: DevDesc)>,
    pub(crate) rect: Option<fn(x0: f64, y0: f64, x1: f64, y1: f64, gc: R_GE_gcontext, dd: DevDesc)>,
    pub(crate) path: Option<
        fn(x: &[f64], y: &[f64], nper: &[i32], winding: Rboolean, gc: R_GE_gcontext, dd: DevDesc),
    >,
    pub(crate) raster: Option<
        fn(
            raster: &[u32],
            w: usize,
            h: usize,
            x: f64,
            y: f64,
            width: f64,
            height: f64,
            rot: f64,
            interpolate: Rboolean,
            gc: R_GE_gcontext,
            dd: DevDesc,
        ),
    >,
    pub(crate) cap: Option<fn(dd: DevDesc) -> SEXP>,
    pub(crate) size: Option<fn(dd: DevDesc) -> (f64, f64, f64, f64)>,
    pub(crate) strWidth: Option<fn(str: &str, gc: R_GE_gcontext, dd: DevDesc) -> f64>,
    pub(crate) text:
        Option<fn(x: f64, y: f64, str: &str, rot: f64, hadj: f64, gc: R_GE_gcontext, dd: DevDesc)>,
    pub(crate) onExit: Option<fn(dd: DevDesc)>,
    pub(crate) newFrameConfirm: Option<fn(dd: DevDesc) -> bool>,
}

#[allow(clippy::type_complexity, non_snake_case)]
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

    pub fn deactivate_wrapper(&self) -> Option<unsafe extern "C" fn(arg1: pDevDesc)> {
        // Return None if no callback function is registered.
        self.deactivate?;

        unsafe extern "C" fn deactivate_wrapper(arg1: pDevDesc) {
            let dev_desc = *arg1;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let deactivate_inner = (*data).callbacks.deactivate.unwrap();

            deactivate_inner(dev_desc);
        }

        Some(deactivate_wrapper)
    }

    pub fn line_wrapper(
        &self,
    ) -> Option<
        unsafe extern "C" fn(x1: f64, y1: f64, x2: f64, y2: f64, gc: pGEcontext, dd: pDevDesc),
    > {
        // Return None if no callback function is registered.
        self.line?;

        unsafe extern "C" fn line_wrapper(
            x1: f64,
            y1: f64,
            x2: f64,
            y2: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let line_inner = (*data).callbacks.line.unwrap();

            let gcontext = *gc;

            line_inner(x1, y1, x2, y2, gcontext, dev_desc);
        }

        Some(line_wrapper)
    }

    pub fn metricInfo_wrapper(
        &self,
    ) -> Option<
        unsafe extern "C" fn(
            c: std::os::raw::c_int,
            gc: pGEcontext,
            ascent: *mut f64,
            descent: *mut f64,
            width: *mut f64,
            dd: pDevDesc,
        ),
    > {
        // Return None if no callback function is registered.
        self.metricInfo?;

        unsafe extern "C" fn metricInfo_wrapper(
            c: std::os::raw::c_int,
            gc: pGEcontext,
            ascent: *mut f64,
            descent: *mut f64,
            width: *mut f64,
            dd: pDevDesc,
        ) {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let metricInfo_inner = (*data).callbacks.metricInfo.unwrap();

            let gcontext = *gc;

            metricInfo_inner(c as _, gcontext, ascent, descent, width, dev_desc);
        }

        Some(metricInfo_wrapper)
    }

    pub fn mode_wrapper(
        &self,
    ) -> Option<unsafe extern "C" fn(mode: std::os::raw::c_int, dd: pDevDesc)> {
        // Return None if no callback function is registered.
        self.mode?;

        unsafe extern "C" fn mode_wrapper(mode: std::os::raw::c_int, dd: pDevDesc) {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let mode_inner = (*data).callbacks.mode.unwrap();

            mode_inner(mode as _, dev_desc);
        }

        Some(mode_wrapper)
    }

    pub fn newPage_wrapper(&self) -> Option<unsafe extern "C" fn(gc: pGEcontext, dd: pDevDesc)> {
        // Return None if no callback function is registered.
        self.newPage?;

        unsafe extern "C" fn newPage_wrapper(gc: pGEcontext, dd: pDevDesc) {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let newPage_inner = (*data).callbacks.newPage.unwrap();

            let gcontext = *gc;

            newPage_inner(gcontext, dev_desc);
        }

        Some(newPage_wrapper)
    }

    pub fn polygon_wrapper(
        &self,
    ) -> Option<
        unsafe extern "C" fn(
            n: std::os::raw::c_int,
            x: *mut f64,
            y: *mut f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    > {
        // Return None if no callback function is registered.
        self.polygon?;

        unsafe extern "C" fn polygon_wrapper(
            n: std::os::raw::c_int,
            x: *mut f64,
            y: *mut f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let polygon_inner = (*data).callbacks.polygon.unwrap();

            let gcontext = *gc;

            let x = slice::from_raw_parts(x, n as _);
            let y = slice::from_raw_parts(y, n as _);

            polygon_inner(x, y, gcontext, dev_desc);
        }

        Some(polygon_wrapper)
    }

    pub fn polyline_wrapper(
        &self,
    ) -> Option<
        unsafe extern "C" fn(
            n: std::os::raw::c_int,
            x: *mut f64,
            y: *mut f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    > {
        // Return None if no callback function is registered.
        self.polyline?;

        unsafe extern "C" fn polyline_wrapper(
            n: std::os::raw::c_int,
            x: *mut f64,
            y: *mut f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let polyline_inner = (*data).callbacks.polyline.unwrap();

            let gcontext = *gc;

            let x = slice::from_raw_parts(x, n as _);
            let y = slice::from_raw_parts(y, n as _);

            polyline_inner(x, y, gcontext, dev_desc);
        }

        Some(polyline_wrapper)
    }

    pub fn rect_wrapper(
        &self,
    ) -> Option<
        unsafe extern "C" fn(x0: f64, x1: f64, y0: f64, y1: f64, gc: pGEcontext, dd: pDevDesc),
    > {
        // Return None if no callback function is registered.
        self.rect?;

        unsafe extern "C" fn rect_wrapper(
            x0: f64,
            x1: f64,
            y0: f64,
            y1: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let rect_inner = (*data).callbacks.rect.unwrap();

            let gcontext = *gc;

            rect_inner(x0, x1, y0, y1, gcontext, dev_desc);
        }

        Some(rect_wrapper)
    }

    pub fn path_wrapper(
        &self,
    ) -> Option<
        unsafe extern "C" fn(
            x: *mut f64,
            y: *mut f64,
            npoly: std::os::raw::c_int,
            nper: *mut std::os::raw::c_int,
            winding: Rboolean,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    > {
        // Return None if no callback function is registered.
        self.path?;

        unsafe extern "C" fn path_wrapper(
            x: *mut f64,
            y: *mut f64,
            npoly: std::os::raw::c_int,
            nper: *mut std::os::raw::c_int,
            winding: Rboolean,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let path_inner = (*data).callbacks.path.unwrap();

            let gcontext = *gc;

            let nper = slice::from_raw_parts(nper, npoly as _);
            // TODO: This isn't very efficient as we need to iterate over nper at least twice.
            let n = nper.iter().sum::<i32>() as usize;
            let x = slice::from_raw_parts(x, n);
            let y = slice::from_raw_parts(y, n);

            path_inner(x, y, nper, winding, gcontext, dev_desc);
        }

        Some(path_wrapper)
    }

    pub fn raster_wrapper(
        &self,
    ) -> Option<
        unsafe extern "C" fn(
            raster: *mut std::os::raw::c_uint,
            w: std::os::raw::c_int,
            h: std::os::raw::c_int,
            x: f64,
            y: f64,
            width: f64,
            height: f64,
            rot: f64,
            interpolate: Rboolean,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    > {
        // Return None if no callback function is registered.
        self.raster?;

        unsafe extern "C" fn raster_wrapper(
            raster: *mut std::os::raw::c_uint,
            w: std::os::raw::c_int,
            h: std::os::raw::c_int,
            x: f64,
            y: f64,
            width: f64,
            height: f64,
            rot: f64,
            interpolate: Rboolean,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let raster_inner = (*data).callbacks.raster.unwrap();

            let gcontext = *gc;

            let raster = slice::from_raw_parts(raster, (w * h) as _);

            raster_inner(
                raster,
                w as _,
                h as _,
                x,
                y,
                width,
                height,
                rot,
                interpolate,
                gcontext,
                dev_desc,
            );
        }

        Some(raster_wrapper)
    }

    pub fn cap_wrapper(&self) -> Option<unsafe extern "C" fn(dd: pDevDesc) -> SEXP> {
        // Return None if no callback function is registered.
        self.cap?;

        unsafe extern "C" fn cap_wrapper(dd: pDevDesc) -> SEXP {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let cap_inner = (*data).callbacks.cap.unwrap();

            // TODO: convert the output more nicely
            cap_inner(dev_desc)
        }

        Some(cap_wrapper)
    }

    pub fn size_wrapper(
        &self,
    ) -> Option<
        unsafe extern "C" fn(
            left: *mut f64,
            right: *mut f64,
            bottom: *mut f64,
            top: *mut f64,
            dd: pDevDesc,
        ),
    > {
        // Return None if no callback function is registered.
        self.size?;

        unsafe extern "C" fn size_wrapper(
            left: *mut f64,
            right: *mut f64,
            bottom: *mut f64,
            top: *mut f64,
            dd: pDevDesc,
        ) {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let size_inner = (*data).callbacks.size.unwrap();

            let sizes = size_inner(dev_desc);
            *left = sizes.0;
            *right = sizes.1;
            *bottom = sizes.2;
            *top = sizes.3;
        }

        Some(size_wrapper)
    }

    pub fn strWidth_wrapper(
        &self,
    ) -> Option<
        unsafe extern "C" fn(str: *const std::os::raw::c_char, gc: pGEcontext, dd: pDevDesc) -> f64,
    > {
        // Return None if no callback function is registered.
        self.strWidth?;

        unsafe extern "C" fn strWidth_wrapper(
            str: *const std::os::raw::c_char,
            gc: pGEcontext,
            dd: pDevDesc,
        ) -> f64 {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let strWidth_inner = (*data).callbacks.strWidth.unwrap();

            let cstr = std::ffi::CStr::from_ptr(str);

            let gcontext = *gc;

            // TODO: Should we do something when the str is not available?
            strWidth_inner(cstr.to_str().unwrap(), gcontext, dev_desc)
        }

        Some(strWidth_wrapper)
    }

    pub fn text_wrapper(
        &self,
    ) -> Option<
        unsafe extern "C" fn(
            x: f64,
            y: f64,
            str: *const std::os::raw::c_char,
            rot: f64,
            hadj: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    > {
        // Return None if no callback function is registered.
        self.text?;

        unsafe extern "C" fn text_wrapper(
            x: f64,
            y: f64,
            str: *const std::os::raw::c_char,
            rot: f64,
            hadj: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let text_inner = (*data).callbacks.text.unwrap();

            let cstr = std::ffi::CStr::from_ptr(str);

            let gcontext = *gc;

            // TODO: Should we do something when the str is not available?
            text_inner(x, y, cstr.to_str().unwrap(), rot, hadj, gcontext, dev_desc);
        }

        Some(text_wrapper)
    }

    pub fn onExit_wrapper(&self) -> Option<unsafe extern "C" fn(dd: pDevDesc)> {
        // Return None if no callback function is registered.
        self.onExit?;

        unsafe extern "C" fn onExit_wrapper(dd: pDevDesc) {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let onExit_inner = (*data).callbacks.onExit.unwrap();

            onExit_inner(dev_desc);
        }
        Some(onExit_wrapper)
    }

    pub fn newFrameConfirm_wrapper(
        &self,
    ) -> Option<unsafe extern "C" fn(dd: pDevDesc) -> Rboolean> {
        // Return None if no callback function is registered.
        self.newFrameConfirm?;

        unsafe extern "C" fn newFrameConfirm_wrapper(dd: pDevDesc) -> Rboolean {
            let dev_desc = *dd;
            let data = dev_desc.deviceSpecific as *const DeviceSpecificData;
            let newFrameConfirm_inner = (*data).callbacks.newFrameConfirm.unwrap();

            newFrameConfirm_inner(dev_desc).try_into().unwrap()
        }

        Some(newFrameConfirm_wrapper)
    }
}
