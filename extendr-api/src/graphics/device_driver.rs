use core::slice;

use crate::*;
use libR_sys::*;

use super::{device_descriptor::*, Device};

/// The underlying C structure `DevDesc` has two fields related to clipping:
///
/// - `canClip`
/// - `deviceClip` (available on R >= 4.1)
///
/// `canClip` indicates whether the device has clipping functionality at all. If
/// not, the graphic engine kindly clips before sending the drawing operations
/// to the device. But, this isn't very ideal in some points. Especially, it's
/// bad that the engine will omit "any text that does not appear to be wholly
/// inside the clipping region," according to [the R Internals]. So, the device
/// should implement `clip()` and set `canClip` to `true` if possible.
///
/// Even when `canClip` is `true`, the engine does clip to protect the device
/// from large values by default. But, for efficiency, the device can take all
/// the responsibility of clipping. That is `deviceClip`, which was introduced in R 4.1. If this is set to
/// `true`, the engine will perform no clipping at all. For more details, please
/// refer to [the offical announcement blog post].
///
/// So, in short, a graphic device can choose either of the following:
///
/// - clipping without the help of the graphic engine (`Device`)
/// - clipping with the help of the graphic engine (`DeviceAndEngine`)
/// - no clipping at all (`Engine`)
///
/// [the R Internals]:
///     https://cran.r-project.org/doc/manuals/r-devel/R-ints.html#Handling-text
/// [the announcement blog post]:
///     https://developer.r-project.org/Blog/public/2020/06/08/improvements-to-clipping-in-the-r-graphics-engine/
pub enum ClippingStrategy {
    Device,
    DeviceAndEngine,
    Engine,
}

/// An implementation of the [Device] functionalities.
#[allow(non_snake_case, unused_variables, clippy::too_many_arguments)]
pub trait DeviceDriver: std::marker::Sized {
    /// Usually, the default implementation of `raster`, which does nothing, is
    /// used. When you want to skip clipping at all, this should be set `false`.
    const USE_RASTER: bool = true;

    /// Usually, the default implementation of `capture`, which does nothing, is
    /// used. When you want to skip clipping at all, this should be set `false`.
    const USE_CAPTURE: bool = true;

    /// Whether the device maintain a plot history. This corresponds to
    /// `displayListOn` in the underlying [DevDesc].
    const USE_PLOT_HISTORY: bool = false;

    /// To what extent the device takes the responsibility of clipping. See
    /// [ClippingStrategy] for the details.
    const CLIPPING_STRATEGY: ClippingStrategy = ClippingStrategy::DeviceAndEngine;

    /// Set this to `false` if the implemented `strWidth()` and `text()` only
    /// accept ASCII text.
    const ACCEPT_UTF8_TEXT: bool = true;

    /// A callback function to setup the device when the device is activated.
    fn activate(&mut self, dd: DevDesc) {}

    /// A callback function to draw a circle.
    fn circle(&mut self, x: f64, y: f64, r: f64, gc: R_GE_gcontext, dd: DevDesc) {}

    /// A callback function to clip.
    fn clip(&mut self, x0: f64, x1: f64, y0: f64, y1: f64, dd: DevDesc) {}

    /// A callback function to free device-specific resources when the
    /// device is killed.
    fn close(&mut self, dd: DevDesc) {}

    /// A callback function to clean up when the device is deactivated.
    fn deactivate(&mut self, dd: DevDesc) {}

    /// TODO'
    // /// A callback function that returns the location of the next mouse click.
    // ///
    // /// If the device doesn't accept mouse clicks, this should be left `None`.
    // fn locator(dd: DevDesc) -> (f64, f64) {}

    /// A callback function to draw a line.
    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, gc: R_GE_gcontext, dd: DevDesc) {}

    /// A callback function that return the metric info of a glyph.
    fn metricInfo(&mut self, c: char, gc: R_GE_gcontext, dd: DevDesc) -> (f64, f64, f64) {
        (0.0, 0.0, 0.0)
    }

    /// A callback function called whenever the graphics engine starts
    /// drawing (mode=1) or stops drawing (mode=0).
    fn mode(&mut self, mode: i32, dd: DevDesc) {}

    /// A callback function called whenever a new plot requires a new page.
    fn newPage(&mut self, gc: R_GE_gcontext, dd: DevDesc) {}

    /// A callback function to draw a polygon.
    fn polygon(&mut self, x: &[f64], y: &[f64], gc: R_GE_gcontext, dd: DevDesc) {}

    /// A callback function to draw a polyline.
    fn polyline(&mut self, x: &[f64], y: &[f64], gc: R_GE_gcontext, dd: DevDesc) {}

    /// A callback function to draw a rect.
    fn rect(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, gc: R_GE_gcontext, dd: DevDesc) {}

    /// A callback function to draw paths.
    ///
    /// `nper` contains number of points in each polygon. `winding` represents
    /// the filling rule; `TRUE` means "nonzero", `FALSE` means "evenodd".
    fn path(
        &mut self,
        x: &[f64],
        y: &[f64],
        nper: &[i32],
        winding: Rboolean,
        gc: R_GE_gcontext,
        dd: DevDesc,
    ) {
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
        &mut self,
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

    /// A callback function that captures and returns the current canvas.
    ///
    /// This is only meaningful for raster devices.
    fn cap(&mut self, dd: DevDesc) -> Robj {
        ().into()
    }

    /// A callback function that is called when the device gets resized.
    ///
    /// The callback should return `(left, right, bottom, top)`.
    fn size(&mut self, dd: DevDesc) -> (f64, f64, f64, f64) {
        (0.0, 0.0, 0.0, 0.0)
    }

    /// A callback function that returns the width of the given string in
    /// the device units.
    fn strWidth(&mut self, str: &str, gc: R_GE_gcontext, dd: DevDesc) -> f64 {
        0.0
    }

    /// A callback function to draw a text.
    ///
    /// `rot` is the rotation in degrees, with positive rotation anticlockwise
    /// from the positive x-axis.
    fn text(
        &mut self,
        x: f64,
        y: f64,
        str: &str,
        rot: f64,
        hadj: f64,
        gc: R_GE_gcontext,
        dd: DevDesc,
    ) {
    }

    /// A callback function called when the user aborts some operation.
    fn onExit(&mut self, dd: DevDesc) {}

    /// Sets a callback function to confirm a new frame.
    fn newFrameConfirm(&mut self, dd: DevDesc) -> bool {
        true
    }

    /// Create a [Device].
    ///
    /// ```
    /// use extendr_api::prelude::*;
    /// use extendr_api::graphics::device_driver::DeviceDriver;
    /// use extendr_api::graphics::device_descriptor::DeviceDescriptor;
    /// use extendr_api::graphics::DevDesc;
    ///
    /// test!{
    ///     struct MyGraphicDevice {
    ///         last_mode: i32
    ///     }
    ///
    ///     impl DeviceDriver for MyGraphicDevice {
    ///         fn mode(&mut self, mode: i32, _dd: DevDesc) {
    ///             self.last_mode = mode;
    ///         }
    ///     }
    ///
    ///     let device_driver = MyGraphicDevice { last_mode: 0 };
    ///     let device_descriptor = DeviceDescriptor::new();
    ///     let device = device_driver.create_device::<MyGraphicDevice>(device_descriptor, "my graphic device");
    ///
    ///     device.mode_on().unwrap();
    ///
    ///     // TODO: how can I peek at the value of last_mode?
    ///     // assert_eq!(device_driver.last_mode, 1_i32);
    /// }
    /// ```
    #[allow(dead_code)]
    fn create_device<T: DeviceDriver>(
        self,
        device_descriptor: DeviceDescriptor,
        device_name: &'static str,
    ) -> Device {
        #![allow(non_snake_case)]
        #![allow(unused_variables)]
        use std::os::raw::{c_char, c_int, c_uint};

        // The code here is a Rust interpretation of the C-version of example
        // code on the R Internals:
        //
        // https://cran.r-project.org/doc/manuals/r-devel/R-ints.html#Device-structures

        unsafe {
            single_threaded(|| {
                // Check the API version
                R_GE_checkVersionOrDie(R_GE_version as _);

                // Check if there are too many devices
                R_CheckDeviceAvailable();
            });
        }

        // Define wrapper functions. This is a bit boring, and frustrationg to
        // see `create_device()` bloats to such a massive function because of
        // this, but probably there's no other way to do this nicely...

        unsafe extern "C" fn device_driver_activate<T: DeviceDriver>(arg1: pDevDesc) {
            // Derefernce to the original struct without moving it. While this
            // is a dangerous operation, it should be safe as long as the data
            // lives only within this function.
            //
            // Note that, we bravely unwrap() here because deviceSpecific should
            // never be a null pointer, as we set it. If the pDevDesc got
            // currupted, it might happen, but we can do nothing in that weird
            // case anyway.
            let data = ((*arg1).deviceSpecific as *mut T).as_mut().unwrap();

            data.activate(*arg1);
        }

        unsafe extern "C" fn device_driver_circle<T: DeviceDriver>(
            x: f64,
            y: f64,
            r: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            let data = ((*dd).deviceSpecific as *mut T).as_mut().unwrap();
            data.circle(x, y, r, *gc, *dd);
        }

        unsafe extern "C" fn device_driver_clip<T: DeviceDriver>(
            x0: f64,
            x1: f64,
            y0: f64,
            y1: f64,
            dd: pDevDesc,
        ) {
            let data = ((*dd).deviceSpecific as *mut T).as_mut().unwrap();
            data.clip(x0, x1, y0, y1, *dd);
        }

        // Note: close is special. This function is responsible for tearing down
        // the DeviceDriver itself, which is always needed even when no close
        // callback is implemented.
        unsafe extern "C" fn device_driver_close<T: DeviceDriver>(dd: pDevDesc) {
            let dev_desc = *dd;
            let data_ptr = dev_desc.deviceSpecific as *mut T;
            // Convert back to a Rust struct to drop the resources on Rust's side.
            let mut data = Box::from_raw(data_ptr);

            data.close(dev_desc);
        }

        unsafe extern "C" fn device_driver_deactivate<T: DeviceDriver>(arg1: pDevDesc) {
            let mut data = ((*arg1).deviceSpecific as *mut T).read();
            data.deactivate(*arg1);
        }

        unsafe extern "C" fn device_driver_line<T: DeviceDriver>(
            x1: f64,
            y1: f64,
            x2: f64,
            y2: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            let data = ((*dd).deviceSpecific as *mut T).as_mut().unwrap();
            data.line(x1, y1, x2, y2, *gc, *dd);
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
                let data = ((*dd).deviceSpecific as *mut T).as_mut().unwrap();
                let metric_info = data.metricInfo(c, *gc, *dd);
                *ascent = metric_info.0;
                *descent = metric_info.1;
                *width = metric_info.2;
            }
        }

        unsafe extern "C" fn device_driver_mode<T: DeviceDriver>(mode: c_int, dd: pDevDesc) {
            let data = ((*dd).deviceSpecific as *mut T).as_mut().unwrap();
            data.mode(mode as _, *dd);
        }

        unsafe extern "C" fn device_driver_newPage<T: DeviceDriver>(gc: pGEcontext, dd: pDevDesc) {
            let data = ((*dd).deviceSpecific as *mut T).as_mut().unwrap();
            data.newPage(*gc, *dd);
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

            let data = ((*dd).deviceSpecific as *mut T).as_mut().unwrap();
            data.polygon(x, y, *gc, *dd);
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

            let data = ((*dd).deviceSpecific as *mut T).as_mut().unwrap();
            data.polyline(x, y, *gc, *dd);
        }

        unsafe extern "C" fn device_driver_rect<T: DeviceDriver>(
            x0: f64,
            x1: f64,
            y0: f64,
            y1: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ) {
            let data = ((*dd).deviceSpecific as *mut T).as_mut().unwrap();
            data.rect(x0, x1, y0, y1, *gc, *dd);
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

            let data = ((*dd).deviceSpecific as *mut T).as_mut().unwrap();
            data.path(x, y, nper, winding, *gc, *dd);
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
            let data = ((*dd).deviceSpecific as *mut T).as_mut().unwrap();
            let raster = slice::from_raw_parts(raster, (w * h) as _);

            data.raster(
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
            let data = ((*dd).deviceSpecific as *mut T).as_mut().unwrap();
            // TODO: convert the output more nicely
            data.cap(*dd).get()
        }

        unsafe extern "C" fn device_driver_size<T: DeviceDriver>(
            left: *mut f64,
            right: *mut f64,
            bottom: *mut f64,
            top: *mut f64,
            dd: pDevDesc,
        ) {
            let data = ((*dd).deviceSpecific as *mut T).as_mut().unwrap();
            let sizes = data.size(*dd);
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
            let data = ((*dd).deviceSpecific as *mut T).as_mut().unwrap();
            let cstr = std::ffi::CStr::from_ptr(str);

            // TODO: Should we do something when the str is not available?
            if let Ok(cstr) = cstr.to_str() {
                data.strWidth(cstr, *gc, *dd)
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
            let data = ((*dd).deviceSpecific as *mut T).as_mut().unwrap();
            let cstr = std::ffi::CStr::from_ptr(str);

            // TODO: Should we do something when the str is not available?
            if let Ok(cstr) = cstr.to_str() {
                data.text(x, y, cstr, rot, hadj, *gc, *dd);
            }
        }

        unsafe extern "C" fn device_driver_onExit<T: DeviceDriver>(dd: pDevDesc) {
            let data = ((*dd).deviceSpecific as *mut T).as_mut().unwrap();
            data.onExit(*dd);
        }

        unsafe extern "C" fn device_driver_newFrameConfirm<T: DeviceDriver>(
            dd: pDevDesc,
        ) -> Rboolean {
            let data = ((*dd).deviceSpecific as *mut T).as_mut().unwrap();
            if let Ok(confirm) = data.newFrameConfirm(*dd).try_into() {
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

            canClip: match <T>::CLIPPING_STRATEGY {
                ClippingStrategy::Engine => 0,
                _ => 1,
            },

            // As described above, gamma is not supported.
            canChangeGamma: 0,

            canHAdj: CanHAdjOption::VariableAdjustment as _,

            startps: device_descriptor.startps,
            startcol: device_descriptor.startcol.to_i32(),
            startfill: device_descriptor.startfill.to_i32(),
            startlty: device_descriptor.startlty.to_i32(),
            startfont: device_descriptor.startfont.to_i32(),

            startgamma: 1.0,

            // A raw pointer to the data specific to the device.
            deviceSpecific,

            displayListOn: if <T>::USE_PLOT_HISTORY { 1 } else { 0 },

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

            activate: Some(device_driver_activate::<T>),
            circle: Some(device_driver_circle::<T>),
            clip: match <T>::CLIPPING_STRATEGY {
                ClippingStrategy::Engine => None,
                _ => Some(device_driver_clip::<T>),
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

            // This is no longer used and exists only for backward-compatibility
            // of the structure.
            getEvent: None,

            newFrameConfirm: Some(device_driver_newFrameConfirm::<T>),

            // UTF-8 support
            hasTextUTF8: if <T>::ACCEPT_UTF8_TEXT { 1 } else { 0 },
            textUTF8: if <T>::ACCEPT_UTF8_TEXT {
                Some(device_driver_text::<T>)
            } else {
                None
            },
            strWidthUTF8: if <T>::ACCEPT_UTF8_TEXT {
                Some(device_driver_strWidth::<T>)
            } else {
                None
            },
            wantSymbolUTF8: if <T>::ACCEPT_UTF8_TEXT { 1 } else { 0 },

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

            // There might be some cases where we want to use `Unset` or
            // `ExceptForMissingValues`, but, for the sake of simplicity, we
            // only use yes or no. Let's revisit here when necessary.
            haveRaster: if <T>::USE_RASTER {
                GraphicDeviceCapabilityRaster::Yes as _
            } else {
                GraphicDeviceCapabilityRaster::No as _
            },

            haveCapture: if <T>::USE_CAPTURE {
                GraphicDeviceCapabilityCapture::Yes as _
            } else {
                GraphicDeviceCapabilityCapture::No as _
            },

            haveLocator: GraphicDeviceCapabilityLocator::Unset as _,

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
            deviceClip: match <T>::CLIPPING_STRATEGY {
                ClippingStrategy::Device => 1,
                _ => 0,
            },

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
