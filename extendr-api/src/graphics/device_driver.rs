use core::slice;

use crate::*;
use libR_sys::*;

use super::{device_descriptor::*, Device};

// This contains the content of the callback functions, which will be called
// from a template callback function. This is needed since
#[allow(non_snake_case, unused_variables, clippy::too_many_arguments)]
pub trait DeviceDriver: std::marker::Sized {
    /// A callback function to setup the device when the device is activated.
    fn activate(dd: DevDesc) {}

    /// A callback function to draw a circle.
    fn circle(x: f64, y: f64, r: f64, gc: R_GE_gcontext, dd: DevDesc) {}

    /// A callback function to clip.
    fn clip(x0: f64, x1: f64, y0: f64, y1: f64, dd: DevDesc) {}

    /// Usually, the default implementation of `clip`, which does nothing, is
    /// used. When you want to skip clipping at all, this should be set `false`.
    const USE_CLIP: bool = true;

    /// A callback function to free device-specific resources when the
    /// device is killed.
    fn close(dd: DevDesc) {}

    /// A callback function to clean up when the device is deactivated.
    fn deactivate(dd: DevDesc) {}

    /// TODO'
    // /// A callback function that returns the location of the next mouse click.
    // ///
    // /// If the device doesn't accept mouse clicks, this should be left `None`.
    // fn locator(dd: DevDesc) -> (f64, f64) {}

    /// A callback function to draw a line.
    fn line(x1: f64, y1: f64, x2: f64, y2: f64, gc: R_GE_gcontext, dd: DevDesc) {}

    /// A callback function that return the metric info of a glyph.
    fn metricInfo(c: char, gc: R_GE_gcontext, dd: DevDesc) -> (f64, f64, f64) {
        (0.0, 0.0, 0.0)
    }

    /// A callback function called whenever the graphics engine starts
    /// drawing (mode=1) or stops drawing (mode=0).
    fn mode(mode: i32, dd: DevDesc) {}

    /// A callback function called whenever a new plot requires a new page.
    fn newPage(gc: R_GE_gcontext, dd: DevDesc) {}

    /// A callback function to draw a polygon.
    fn polygon(x: &[f64], y: &[f64], gc: R_GE_gcontext, dd: DevDesc) {}

    /// A callback function to draw a polyline.
    fn polyline(x: &[f64], y: &[f64], gc: R_GE_gcontext, dd: DevDesc) {}

    /// A callback function to draw a rect.
    fn rect(x0: f64, y0: f64, x1: f64, y1: f64, gc: R_GE_gcontext, dd: DevDesc) {}

    /// A callback function to draw paths.
    ///
    /// `nper` contains number of points in each polygon. `winding` represents
    /// the filling rule; `TRUE` means "nonzero", `FALSE` means "evenodd".
    fn path(x: &[f64], y: &[f64], nper: &[i32], winding: Rboolean, gc: R_GE_gcontext, dd: DevDesc) {
    }

    /// A callback function to draw a raster.
    ///
    /// `raster` is a ROW-wise array of color (ABGR). `w` and `h` represents the
    /// number of elements in the row and the column of the raster. `x` and `y`
    /// is the size of the raster in points. `rot` is the rotation in degrees,
    /// with positive rotation anticlockwise from the positive x-axis.
    /// `interpolate` is whether to apply the linear interpolation on the raster
    /// image.
    fn raster(
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
    ) {
    }

    /// Usually, the default implementation of `raster`, which does nothing, is
    /// used. When you want to skip clipping at all, this should be set `false`.
    const USE_RASTER: bool = true;

    /// A callback function that captures and returns the current canvas.
    ///
    /// This is only meaningful for raster devices.
    fn cap(dd: DevDesc) -> Robj {
        ().into()
    }

    /// Usually, the default implementation of `capture`, which does nothing, is
    /// used. When you want to skip clipping at all, this should be set `false`.
    const USE_CAPTURE: bool = true;

    /// A callback function that is called when the device gets resized.
    ///
    /// The callback should return `(left, right, bottom, top)`.
    fn size(dd: DevDesc) -> (f64, f64, f64, f64) {
        (0.0, 0.0, 0.0, 0.0)
    }

    /// A callback function that returns the width of the given string in
    /// the device units.
    fn strWidth(str: &str, gc: R_GE_gcontext, dd: DevDesc) -> f64 {
        0.0
    }

    /// A callback function to draw a text.
    ///
    /// `rot` is the rotation in degrees, with positive rotation anticlockwise
    /// from the positive x-axis.
    fn text(x: f64, y: f64, str: &str, rot: f64, hadj: f64, gc: R_GE_gcontext, dd: DevDesc) {}

    /// A callback function called when the user aborts some operation.
    fn onExit(dd: DevDesc) {}

    /// Sets a callback function to confirm a new frame.
    fn newFrameConfirm(dd: DevDesc) -> bool {
        true
    }

    /// Create a [Device].
    #[allow(dead_code)]
    fn create_device<T: DeviceDriver>(
        self,
        device_descriptor: DeviceDescriptor,
        device_name: &'static str,
    ) -> Device {
        #![allow(non_snake_case)]
        #![allow(unused_variables)]
        use std::os::raw::{c_char, c_int, c_uint};

        unsafe {
            single_threaded(|| {
                // Check the API version
                R_GE_checkVersionOrDie(R_GE_version as _);

                // Check if there are too many devices
                R_CheckDeviceAvailable();
            });
        }

        unsafe extern "C" fn device_driver_activate<T: DeviceDriver>(arg1: pDevDesc) {
            <T>::activate(*arg1);
        }

        unsafe extern "C" fn device_driver_circle<T: DeviceDriver>(
            x: f64,
            y: f64,
            r: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            <T>::circle(x, y, r, *gc, *dd);
        }

        unsafe extern "C" fn device_driver_clip<T: DeviceDriver>(
            x0: f64,
            x1: f64,
            y0: f64,
            y1: f64,
            dd: pDevDesc,
        ) {
            <T>::clip(x0, x1, y0, y1, *dd);
        }

        // Note: close is special. This function is responsible for tearing down the
        // DeviceSpecificData itself, which is always needed even when no close
        // callback is supplied.
        unsafe extern "C" fn device_driver_close<T: DeviceDriver>(dd: pDevDesc) {
            let dev_desc = *dd;

            <T>::close(dev_desc);

            // Convert back to a Rust struct to drop the resources on Rust's side.
            Box::from_raw(dev_desc.deviceSpecific as *mut T);
        }

        unsafe extern "C" fn device_driver_deactivate<T: DeviceDriver>(arg1: pDevDesc) {
            <T>::deactivate(*arg1);
        }

        unsafe extern "C" fn device_driver_line<T: DeviceDriver>(
            x1: f64,
            y1: f64,
            x2: f64,
            y2: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            <T>::line(x1, y1, x2, y2, *gc, *dd);
        }

        unsafe extern "C" fn device_driver_metricInfo<T: DeviceDriver>(
            c: c_int,
            gc: pGEcontext,
            ascent: *mut f64,
            descent: *mut f64,
            width: *mut f64,
            dd: pDevDesc,
        ) {
            if let Some(c) = std::char::from_u32(c as _) {
                let metric_info = <T>::metricInfo(c, *gc, *dd);
                *ascent = metric_info.0;
                *descent = metric_info.1;
                *width = metric_info.2;
            }
        }

        unsafe extern "C" fn device_driver_mode<T: DeviceDriver>(mode: c_int, dd: pDevDesc) {
            <T>::mode(mode as _, *dd);
        }

        unsafe extern "C" fn device_driver_newPage<T: DeviceDriver>(gc: pGEcontext, dd: pDevDesc) {
            <T>::newPage(*gc, *dd);
        }

        unsafe extern "C" fn device_driver_polygon<T: DeviceDriver>(
            n: c_int,
            x: *mut f64,
            y: *mut f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            let x = slice::from_raw_parts(x, n as _);
            let y = slice::from_raw_parts(y, n as _);

            <T>::polygon(x, y, *gc, *dd);
        }

        unsafe extern "C" fn device_driver_polyline<T: DeviceDriver>(
            n: c_int,
            x: *mut f64,
            y: *mut f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            let x = slice::from_raw_parts(x, n as _);
            let y = slice::from_raw_parts(y, n as _);

            <T>::polyline(x, y, *gc, *dd);
        }

        unsafe extern "C" fn device_driver_rect<T: DeviceDriver>(
            x0: f64,
            x1: f64,
            y0: f64,
            y1: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            <T>::rect(x0, x1, y0, y1, *gc, *dd);
        }

        unsafe extern "C" fn device_driver_path<T: DeviceDriver>(
            x: *mut f64,
            y: *mut f64,
            npoly: c_int,
            nper: *mut c_int,
            winding: Rboolean,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            let nper = slice::from_raw_parts(nper, npoly as _);
            // TODO: This isn't very efficient as we need to iterate over nper at least twice.
            let n = nper.iter().sum::<i32>() as usize;
            let x = slice::from_raw_parts(x, n);
            let y = slice::from_raw_parts(y, n);

            <T>::path(x, y, nper, winding, *gc, *dd);
        }

        unsafe extern "C" fn device_driver_raster<T: DeviceDriver>(
            raster: *mut c_uint,
            w: c_int,
            h: c_int,
            x: f64,
            y: f64,
            width: f64,
            height: f64,
            rot: f64,
            interpolate: Rboolean,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            let raster = slice::from_raw_parts(raster, (w * h) as _);

            <T>::raster(
                raster,
                w as _,
                h as _,
                x,
                y,
                width,
                height,
                rot,
                interpolate,
                *gc,
                *dd,
            );
        }

        unsafe extern "C" fn device_driver_cap<T: DeviceDriver>(dd: pDevDesc) -> SEXP {
            // TODO: convert the output more nicely
            <T>::cap(*dd).get()
        }

        unsafe extern "C" fn device_driver_size<T: DeviceDriver>(
            left: *mut f64,
            right: *mut f64,
            bottom: *mut f64,
            top: *mut f64,
            dd: pDevDesc,
        ) {
            let sizes = <T>::size(*dd);
            *left = sizes.0;
            *right = sizes.1;
            *bottom = sizes.2;
            *top = sizes.3;
        }

        unsafe extern "C" fn device_driver_strWidth<T: DeviceDriver>(
            str: *const c_char,
            gc: pGEcontext,
            dd: pDevDesc,
        ) -> f64 {
            let cstr = std::ffi::CStr::from_ptr(str);

            // TODO: Should we do something when the str is not available?
            if let Ok(cstr) = cstr.to_str() {
                <T>::strWidth(cstr, *gc, *dd)
            } else {
                0.0
            }
        }

        unsafe extern "C" fn device_driver_text<T: DeviceDriver>(
            x: f64,
            y: f64,
            str: *const c_char,
            rot: f64,
            hadj: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            let cstr = std::ffi::CStr::from_ptr(str);

            // TODO: Should we do something when the str is not available?
            if let Ok(cstr) = cstr.to_str() {
                <T>::text(x, y, cstr, rot, hadj, *gc, *dd);
            }
        }

        unsafe extern "C" fn device_driver_onExit<T: DeviceDriver>(dd: pDevDesc) {
            <T>::onExit(*dd);
        }

        unsafe extern "C" fn device_driver_newFrameConfirm<T: DeviceDriver>(
            dd: pDevDesc,
        ) -> Rboolean {
            if let Ok(confirm) = <T>::newFrameConfirm(*dd).try_into() {
                confirm
            } else {
                false.into()
            }
        }

        let deviceSpecific = Box::into_raw(Box::new(self)) as *mut std::os::raw::c_void;

        let dev_desc = Box::new(DevDesc {
            left: device_descriptor.left,
            right: device_descriptor.right,
            bottom: device_descriptor.bottom,
            top: device_descriptor.top,

            // This should be the same as the size of the device
            clipLeft: device_descriptor.left,
            clipRight: device_descriptor.right,
            clipBottom: device_descriptor.bottom,
            clipTop: device_descriptor.top,

            // Not sure where these numbers came from, but it seems this is a
            // common practice, considering the postscript device and svglite
            // device do so.
            xCharOffset: 0.4900,
            yCharOffset: 0.3333,
            yLineBias: 0.2,

            ipr: device_descriptor.ipr,
            cra: device_descriptor.cra,

            // Gamma-related parameters are all ignored. R-internals indicates so:
            //
            // canChangeGamma – Rboolean: can the display gamma be adjusted? This is now
            // ignored, as gamma support has been removed.
            //
            // and actually it seems this parameter is never used.
            gamma: 1.0,

            canClip: if device_descriptor.canClip { 1 } else { 0 },

            // As described above, gamma is not supported.
            canChangeGamma: 0,

            canHAdj: device_descriptor.canHAdj as _,

            startps: device_descriptor.startps,
            startcol: device_descriptor.startcol.to_i32(),
            startfill: device_descriptor.startfill.to_i32(),
            startlty: device_descriptor.startlty.to_i32(),
            startfont: device_descriptor.startfont.to_i32(),

            startgamma: 1.0,

            // A raw pointer to the data specific to the device.
            deviceSpecific,

            displayListOn: if device_descriptor.displayListOn {
                1
            } else {
                0
            },

            // These are currently not used, so just set FALSE.
            canGenMouseDown: 0,
            canGenMouseMove: 0,
            canGenMouseUp: 0,
            canGenKeybd: 0,
            canGenIdle: 0,

            // The header file says:
            //
            // This is set while getGraphicsEvent is actively looking for events.
            //
            // It seems no implementation sets this, so this is probably what is
            // modified on the engine's side.
            gettingEvent: 0,

            // These are the functions that handles actual operations.
            activate: Some(device_driver_activate::<T>),
            circle: Some(device_driver_circle::<T>),
            clip: if <T>::USE_CLIP {
                Some(device_driver_clip::<T>)
            } else {
                None
            },
            close: Some(device_driver_close::<T>),
            deactivate: Some(device_driver_deactivate::<T>),
            locator: None, // TODO
            line: Some(device_driver_line::<T>),
            metricInfo: Some(device_driver_metricInfo::<T>),
            mode: Some(device_driver_mode::<T>),
            newPage: Some(device_driver_newPage::<T>),
            polygon: Some(device_driver_polygon::<T>),
            polyline: Some(device_driver_polyline::<T>),
            rect: Some(device_driver_rect::<T>),
            path: Some(device_driver_path::<T>),
            raster: if <T>::USE_RASTER {
                Some(device_driver_raster::<T>)
            } else {
                None
            },
            cap: if <T>::USE_CAPTURE {
                Some(device_driver_cap::<T>)
            } else {
                None
            },
            size: Some(device_driver_size::<T>),
            strWidth: Some(device_driver_strWidth::<T>),
            text: Some(device_driver_text::<T>),
            onExit: Some(device_driver_onExit::<T>),
            getEvent: None, // This is no longer used and exists only for backward-compatibility of the structure.
            newFrameConfirm: Some(device_driver_newFrameConfirm::<T>),

            // UTF-8 support
            hasTextUTF8: if device_descriptor.hasTextUTF8 { 1 } else { 0 },
            textUTF8: device_descriptor.textUTF8,
            strWidthUTF8: device_descriptor.strWidthUTF8,
            wantSymbolUTF8: if device_descriptor.wantSymbolUTF8 {
                1
            } else {
                0
            },

            useRotatedTextInContour: if device_descriptor.useRotatedTextInContour {
                1
            } else {
                0
            },

            eventEnv: unsafe { device_descriptor.eventEnv.get() },
            eventHelper: device_descriptor.eventHelper,

            holdflush: device_descriptor.holdflush,

            haveTransparency: device_descriptor.haveTransparency as _,
            haveTransparentBg: device_descriptor.haveTransparentBg as _,
            haveRaster: device_descriptor.haveRaster as _,
            haveCapture: device_descriptor.haveCapture as _,
            haveLocator: device_descriptor.haveLocator as _,

            #[cfg(use_r_ge_version_14)]
            setPattern: device_descriptor.setPattern,
            #[cfg(use_r_ge_version_14)]
            releasePattern: device_descriptor.releasePattern,

            #[cfg(use_r_ge_version_14)]
            setClipPath: device_descriptor.setClipPath,
            #[cfg(use_r_ge_version_14)]
            releaseClipPath: device_descriptor.releaseClipPath,

            #[cfg(use_r_ge_version_14)]
            setMask: device_descriptor.setMask,
            #[cfg(use_r_ge_version_14)]
            releaseMask: device_descriptor.releaseMask,

            #[cfg(use_r_ge_version_14)]
            deviceVersion: device_descriptor.deviceVersion as _,

            #[cfg(use_r_ge_version_14)]
            deviceClip: if device_descriptor.deviceClip { 1 } else { 0 },

            #[cfg(use_r_ge_version_15)]
            defineGroup: device_descriptor.defineGroup,
            #[cfg(use_r_ge_version_15)]
            useGroup: device_descriptor.useGroup,
            #[cfg(use_r_ge_version_15)]
            releaseGroup: device_descriptor.releaseGroup,

            #[cfg(use_r_ge_version_15)]
            stroke: device_descriptor.stroke,
            #[cfg(use_r_ge_version_15)]
            fill: device_descriptor.fill,
            #[cfg(use_r_ge_version_15)]
            fillStroke: device_descriptor.fillStroke,

            #[cfg(use_r_ge_version_15)]
            capabilities: device_descriptor.capabilities,

            reserved: [0i8; 64],
        });

        let device_name = CString::new(device_name).unwrap();

        single_threaded(|| unsafe {
            let p_dev_desc = Box::into_raw(dev_desc);
            let device = GEcreateDevDesc(p_dev_desc);

            // NOTE: If we use GEaddDevice2f(), GEinitDisplayList() is not needed.
            GEaddDevice2(device, device_name.as_ptr() as *mut i8);
            GEinitDisplayList(device);

            Device { inner: device }
        })
    }
}
