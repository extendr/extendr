use crate::*;
use libR_sys::*;

use super::{
    color::{self, Color},
    LineType,
};

// R internals says:
//
// There should be a ‘pointsize’ argument which defaults to 12, and it should
// give the pointsize in big points (1/72 inch). How exactly this is interpreted
// is font-specific, but it should use a font which works with lines packed 1/6
// inch apart, and looks good with lines 1/5 inch apart (that is with 2pt
// leading).
const POINTSIZE: f64 = 12.0;

const PT: f64 = 1.0 / 72.0;
const PT_PER_INCH: f64 = 72.0;

// R internals says:
//
// where ‘fnsize’ is the ‘size’ of the standard font (cex=1) on the device, in
// device units.
//
// and it seems the Postscript device chooses `pointsize` as this.
const FONTSIZE: f64 = POINTSIZE;

// R internal says:
//
// The default size of a device should be 7 inches square.
const WIDTH_INCH: f64 = 7.0;
const HEIGH_INCH: f64 = 7.0;

pub enum CanHAdjOption {
    NotSupported = 0,
    FixedAdjustment = 1,
    VariableAdjustment = 2,
}

pub enum GraphicDeviceCapabilityTransparency {
    Unset = 0,
    No = 1,
    Yes = 2,
}

pub enum GraphicDeviceCapabilityTransparentBg {
    Unset = 0,
    No = 1,
    Fully = 2,
    Semi = 3,
}

pub enum GraphicDeviceCapabilityRaster {
    Unset = 0,
    No = 1,
    Yes = 2,
    ExceptForMissingValues = 3,
}

pub enum GraphicDeviceCapabilityCapture {
    Unset = 0,
    No = 1,
    Yes = 2,
}

pub enum GraphicDeviceCapabilityLocator {
    Unset = 0,
    No = 1,
    Yes = 2,
}

/// A builder of [libR_sys::_DevDesc].
///
/// Compared to the original [libR_sys::_DevDesc], `DeviceDescriptor` omits
/// these fields that seems not very useful:
///
/// - `clipLeft`, `clipRight`, `clipBottom`, `clipTop`: In most of the cases,
///   this should match the device size at first.
/// - `gamma`, `canChangeGamma`: These fields are now ignored because gamma
///   support has been removed.
/// - `deviceSpecific`: This can be provided later when we actually create a
///   [Device].
/// - `canGenMouseDown`, `canGenMouseMove`, `canGenMouseUp`, `canGenKeybd`,
///   `canGenIdle`: These fields are currently not used by R and preserved only
///   for backward-compatibility.
/// - `gettingEvent`: This is set true when getGraphicsEvent is actively looking
///   for events. Reading the description on ["6.1.6 Graphics events" of R
///   Internals](https://cran.r-project.org/doc/manuals/r-devel/R-ints.html#Graphics-events),
///   it seems this flag is not what is controlled by a graphic device.
pub struct DeviceDescriptor {
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,

    xCharOffset: f64,
    yCharOffset: f64,
    yLineBias: f64,

    /// Inches per raster unit, i.e. point. A point is usually 1/72, but another
    /// value can be chosen here to scale the device. The first element is
    /// width, the second is height.
    ipr: [f64; 2],

    /// Font size. The first element is width, the second is height.
    cra: [f64; 2],

    /// Whether the device can clip text. We set FALSE by default, but should be
    /// turned on.
    canClip: bool,

    canHAdj: CanHAdjOption,

    /// The initial values for pointsize.
    startps: f64,

    /// The initial values for colour.
    startcol: Color,

    /// The initial values for fill.
    startfill: Color,

    /// The initial values for linetype.
    startlty: LineType,

    // Note that I couldn't follow how this `startfont` is used, but it seems this "font"
    // means "font face", considering `GPar`'s `font` is set to `fontface` of
    // `pGEcontext` (c.f.
    // https://github.com/wch/r-source/blob/9f284035b7e503aebe4a804579e9e80a541311bb/src/library/graphics/src/graphics.c#L2568).
    /// The initial values for font face.
    startfont: i32,

    displayListOn: bool,

    activate: Option<unsafe extern "C" fn(arg1: pDevDesc)>,
    circle: Option<unsafe extern "C" fn(x: f64, y: f64, r: f64, gc: pGEcontext, dd: pDevDesc)>,
    clip: Option<unsafe extern "C" fn(x0: f64, x1: f64, y0: f64, y1: f64, dd: pDevDesc)>,
    close: Option<unsafe extern "C" fn(dd: pDevDesc)>,
    deactivate: Option<unsafe extern "C" fn(arg1: pDevDesc)>,
    locator: Option<unsafe extern "C" fn(x: *mut f64, y: *mut f64, dd: pDevDesc) -> Rboolean>,
    line: Option<
        unsafe extern "C" fn(x1: f64, y1: f64, x2: f64, y2: f64, gc: pGEcontext, dd: pDevDesc),
    >,
    metricInfo: Option<
        unsafe extern "C" fn(
            c: std::os::raw::c_int,
            gc: pGEcontext,
            ascent: *mut f64,
            descent: *mut f64,
            width: *mut f64,
            dd: pDevDesc,
        ),
    >,
    mode: Option<unsafe extern "C" fn(mode: std::os::raw::c_int, dd: pDevDesc)>,
    newPage: Option<unsafe extern "C" fn(gc: pGEcontext, dd: pDevDesc)>,
    polygon: Option<
        unsafe extern "C" fn(
            n: std::os::raw::c_int,
            x: *mut f64,
            y: *mut f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    polyline: Option<
        unsafe extern "C" fn(
            n: std::os::raw::c_int,
            x: *mut f64,
            y: *mut f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    rect: Option<
        unsafe extern "C" fn(x0: f64, y0: f64, x1: f64, y1: f64, gc: pGEcontext, dd: pDevDesc),
    >,
    path: Option<
        unsafe extern "C" fn(
            x: *mut f64,
            y: *mut f64,
            npoly: std::os::raw::c_int,
            nper: *mut std::os::raw::c_int,
            winding: Rboolean,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    raster: Option<
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
    >,
    cap: Option<unsafe extern "C" fn(dd: pDevDesc) -> SEXP>,
    size: Option<
        unsafe extern "C" fn(
            left: *mut f64,
            right: *mut f64,
            bottom: *mut f64,
            top: *mut f64,
            dd: pDevDesc,
        ),
    >,
    strWidth: Option<
        unsafe extern "C" fn(str: *const std::os::raw::c_char, gc: pGEcontext, dd: pDevDesc) -> f64,
    >,
    text: Option<
        unsafe extern "C" fn(
            x: f64,
            y: f64,
            str: *const std::os::raw::c_char,
            rot: f64,
            hadj: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    onExit: Option<unsafe extern "C" fn(dd: pDevDesc)>,
    getEvent: Option<unsafe extern "C" fn(arg1: SEXP, arg2: *const std::os::raw::c_char) -> SEXP>,
    newFrameConfirm: Option<unsafe extern "C" fn(dd: pDevDesc) -> Rboolean>,

    // UTF-8 support
    hasTextUTF8: bool,
    textUTF8: Option<
        unsafe extern "C" fn(
            x: f64,
            y: f64,
            str: *const std::os::raw::c_char,
            rot: f64,
            hadj: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    strWidthUTF8: Option<
        unsafe extern "C" fn(str: *const std::os::raw::c_char, gc: pGEcontext, dd: pDevDesc) -> f64,
    >,
    wantSymbolUTF8: bool,

    // R internals says:
    //
    //     Some devices can produce high-quality rotated text, but those based on
    //     bitmaps often cannot. Those which can should set useRotatedTextInContour
    //     to be true from graphics API version 4.
    //
    // It seems this is used only by plot3d, so FALSE should be appropriate in
    // most of the cases.
    useRotatedTextInContour: bool,

    /// If the graphic device is to handle user interaction, set these. For more
    /// details can be found on R Internals:
    ///  
    /// https://cran.r-project.org/doc/manuals/r-devel/R-ints.html#Graphics-events
    eventEnv: Environment,
    eventHelper: Option<unsafe extern "C" fn(dd: pDevDesc, code: std::os::raw::c_int)>,

    /// The header file says:
    ///
    /// Allows graphics devices to have multiple levels of suspension: when this
    /// reaches zero output is flushed.
    holdflush: Option<
        unsafe extern "C" fn(dd: pDevDesc, level: std::os::raw::c_int) -> std::os::raw::c_int,
    >,

    /// Device capabilities. In all cases, 0 means NA (unset), and 1 means no.
    /// It seems 2 or larger numbers typically represents "yes."
    haveTransparency: GraphicDeviceCapabilityTransparency,
    haveTransparentBg: GraphicDeviceCapabilityTransparentBg,
    haveRaster: GraphicDeviceCapabilityRaster,
    haveCapture: GraphicDeviceCapabilityCapture,
    haveLocator: GraphicDeviceCapabilityLocator,

    /// Patterns and gradients (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/definitions/definitions.html#internals)
    #[cfg(use_r_ge_version_14)]
    setPattern: Option<unsafe extern "C" fn(pattern: SEXP, dd: pDevDesc) -> SEXP>,

    /// Patterns and gradients (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/definitions/definitions.html#internals)
    #[cfg(use_r_ge_version_14)]
    releasePattern: Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,

    /// Clipping paths (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/definitions/definitions.html#internals)
    #[cfg(use_r_ge_version_14)]
    setClipPath: Option<unsafe extern "C" fn(path: SEXP, ref_: SEXP, dd: pDevDesc) -> SEXP>,

    /// Clipping paths (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/definitions/definitions.html#internals)
    #[cfg(use_r_ge_version_14)]
    releaseClipPath: Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,

    /// Masks (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/definitions/definitions.html#internals)
    #[cfg(use_r_ge_version_14)]
    setMask: Option<unsafe extern "C" fn(path: SEXP, ref_: SEXP, dd: pDevDesc) -> SEXP>,

    /// Masks (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/definitions/definitions.html#internals)
    #[cfg(use_r_ge_version_14)]
    releaseMask: Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,

    /// The version of the graphic device API. Surprisingly, we can set the
    /// device version other than the actual graphic device version (probably to
    /// avoid the "Graphics API version mismatch" error).
    #[cfg(use_r_ge_version_14)]
    deviceVersion: u32,

    /// If TRUE, the graphic engine does no clipping and the device is supposed
    /// to handle all of them.
    #[cfg(use_r_ge_version_14)]
    deviceClip: bool,

    /// Group compositing operations and affine transformation (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/groups/groups.html)
    #[cfg(use_r_ge_version_15)]
    defineGroup: Option<
        unsafe extern "C" fn(
            source: SEXP,
            op: ::std::os::raw::c_int,
            destination: SEXP,
            dd: pDevDesc,
        ) -> SEXP,
    >,

    /// Group compositing operations and affine transformation (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/groups/groups.html)
    #[cfg(use_r_ge_version_15)]
    useGroup: Option<unsafe extern "C" fn(ref_: SEXP, trans: SEXP, dd: pDevDesc)>,

    /// Group compositing operations and affine transformation (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/groups/groups.html)
    #[cfg(use_r_ge_version_15)]
    releaseGroup: Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,

    /// Stroking and filling paths (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/paths/paths.html)
    #[cfg(use_r_ge_version_15)]
    stroke: Option<unsafe extern "C" fn(path: SEXP, gc: pGEcontext, dd: pDevDesc)>,

    /// Stroking and filling paths (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/paths/paths.html)
    #[cfg(use_r_ge_version_15)]
    fill: Option<
        unsafe extern "C" fn(path: SEXP, rule: ::std::os::raw::c_int, gc: pGEcontext, dd: pDevDesc),
    >,

    /// Stroking and filling paths (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/paths/paths.html)
    #[cfg(use_r_ge_version_15)]
    fillStroke: Option<
        unsafe extern "C" fn(path: SEXP, rule: ::std::os::raw::c_int, gc: pGEcontext, dd: pDevDesc),
    >,

    /// R Internals says:
    ///
    /// In addition, the capabilities callback allows the device driver to
    /// provide more detailed information, especially related to callbacks in
    /// the engine/device API version 13 or higher. The capabilities callback is
    /// called with a list of integer vectors that represent the best guess that
    /// the graphics engine can make, based on the flags in the DevDesc
    /// structure and the ‘deviceVersion’.
    #[cfg(use_r_ge_version_15)]
    capabilities: ::std::option::Option<unsafe extern "C" fn(cap: SEXP) -> SEXP>,
}

impl DeviceDescriptor {
    pub fn new() -> Self {
        Self {
            left: 0.0,
            right: WIDTH_INCH * PT_PER_INCH,
            bottom: HEIGH_INCH * PT_PER_INCH,
            top: 0.0,

            // Not sure where these numbers came from, but it seems this is a
            // common practice, considering the postscript device and svglite
            // device do so.
            xCharOffset: 0.4900,
            yCharOffset: 0.3333,
            yLineBias: 0.2,

            ipr: [PT, PT],

            // Font size. Not sure why these 0.9 and 1.2 are chosen, but R
            // internals says "It is suggested that a good choice is"
            cra: [0.9 * FONTSIZE, 1.2 * FONTSIZE],

            canClip: false,

            canHAdj: CanHAdjOption::NotSupported,

            startps: POINTSIZE,
            startcol: Color::hex(0x000000),
            startfill: Color::hex(0x000000),
            startlty: LineType::Solid,

            // As `GInit()` sets `1`, use the same value here.
            startfont: 1,

            // The header file says "toggle for initial display list status."
            // When we want to maintain a plot history, this should be turned on
            // so that `GEinitDisplayList` is invoked.
            displayListOn: false,

            activate: None,
            circle: None,
            clip: None,
            close: None,
            deactivate: None,
            locator: None,
            line: None,
            metricInfo: None,
            mode: None,
            newPage: None,
            polygon: None,
            polyline: None,
            rect: None,
            path: None,
            raster: None,
            cap: None,
            size: None,
            strWidth: None,
            text: None,
            onExit: None,
            getEvent: None,
            newFrameConfirm: None,

            // UTF-8 support
            hasTextUTF8: false,
            textUTF8: None,
            strWidthUTF8: None,
            wantSymbolUTF8: false,

            // R internals says:
            //
            // Some devices can produce high-quality rotated text, but those
            // based on bitmaps often cannot. Those which can should set
            // useRotatedTextInContour to be true from graphics API version 4.
            //
            // It seems this is used only by plot3d, so FALSE should be
            // appropriate in most of the cases.
            useRotatedTextInContour: false,

            eventEnv: empty_env(),
            eventHelper: None,

            holdflush: None,

            haveTransparency: GraphicDeviceCapabilityTransparency::No,
            haveTransparentBg: GraphicDeviceCapabilityTransparentBg::No,
            haveRaster: GraphicDeviceCapabilityRaster::No,
            haveCapture: GraphicDeviceCapabilityCapture::No,
            haveLocator: GraphicDeviceCapabilityLocator::No,

            #[cfg(use_r_ge_version_14)]
            setPattern: None,
            #[cfg(use_r_ge_version_14)]
            releasePattern: None,

            #[cfg(use_r_ge_version_14)]
            setClipPath: None,
            #[cfg(use_r_ge_version_14)]
            releaseClipPath: None,

            #[cfg(use_r_ge_version_14)]
            setMask: None,
            #[cfg(use_r_ge_version_14)]
            releaseMask: None,

            #[cfg(use_r_ge_version_14)]
            deviceVersion: R_GE_definitions as _,

            #[cfg(use_r_ge_version_14)]
            deviceClip: false,

            #[cfg(use_r_ge_version_15)]
            defineGroup: None,
            #[cfg(use_r_ge_version_15)]
            useGroup: None,
            #[cfg(use_r_ge_version_15)]
            releaseGroup: None,

            #[cfg(use_r_ge_version_15)]
            stroke: None,
            #[cfg(use_r_ge_version_15)]
            fill: None,
            #[cfg(use_r_ge_version_15)]
            fillStroke: None,

            #[cfg(use_r_ge_version_15)]
            capabilities: None,
        }
    }
}

// TODO: create a builder for DevDesc
pub fn default_device_descriptor() -> DevDesc {
    DevDesc {
        left: 0.0,
        right: WIDTH_INCH * PT_PER_INCH,
        bottom: HEIGH_INCH * PT_PER_INCH,
        top: 0.0,

        // This should be the same as the size of the device unless otherwise specified.
        clipLeft: 0.0,
        clipRight: WIDTH_INCH * PT_PER_INCH,
        clipBottom: HEIGH_INCH * PT_PER_INCH,
        clipTop: 0.0,

        // Not sure where these numbers came from, but it seems this is a common
        // practice, considering the postscript device and svglite device do so.
        xCharOffset: 0.4900,
        yCharOffset: 0.3333,
        yLineBias: 0.2,

        // Inches per raster unit, i.e. point.
        ipr: [PT, PT],

        // Font size. Not sure why these 0.9 and 1.2 are chosen, but R internals
        // says "It is suggested that a good choice is"
        cra: [0.9 * FONTSIZE, 1.2 * FONTSIZE],

        // Gamma-related parameters are all ignored. R-internals indicates so:
        //
        // canChangeGamma – Rboolean: can the display gamma be adjusted? This is now
        // ignored, as gamma support has been removed.
        //
        // and actually it seems this parameter is never used.
        gamma: 1.0,

        // Whether the device can clip text. We set FALSE by default, but should be
        // turned on.
        canClip: 0,

        // As described above, gamma is not supported.
        canChangeGamma: 0,

        // Whether the device can handle horizontal alignment of text. We set 0 by
        // default, but should be turned on. Note that this takes int, not Rboolean.
        // R internals says:
        //
        // can the device do horizontal adjustment of text via the text callback,
        // and if so, how precisely? 0 = no adjustment, 1 = {0, 0.5, 1} (left,
        // centre, right justification) or 2 = continuously variable (in [0,1])
        // between left and right justification.
        canHAdj: 0,

        // The initial values for pointsize (ps), colour (col), fill (fill), line
        // type (lty), font face (font), gamma (gamma).
        startps: POINTSIZE,
        startcol: 0x000000,
        startfill: 0x000000,
        startlty: LTY_SOLID as _,
        startfont: 1,
        startgamma: 1.0,

        // A raw pointer to the data specific to the device.
        deviceSpecific: std::ptr::null::<std::ffi::c_void>() as *mut std::ffi::c_void,

        // The header file says "toggle for initial display list status." When we want
        // to maintain a plot history, this should be turned on so that
        // `GEinitDisplayList` is invoked.
        displayListOn: 0,

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
        activate: None,
        circle: None,
        clip: None,
        close: None,
        deactivate: None,
        locator: None,
        line: None,
        metricInfo: None,
        mode: None,
        newPage: None,
        polygon: None,
        polyline: None,
        rect: None,
        path: None,
        raster: None,
        cap: None,
        size: None,
        strWidth: None,
        text: None,
        onExit: None,
        getEvent: None,
        newFrameConfirm: None,

        // UTF-8 support
        hasTextUTF8: 1,
        textUTF8: None,
        strWidthUTF8: None,
        wantSymbolUTF8: 1,

        useRotatedTextInContour: 0,

        eventEnv: unsafe { R_EmptyEnv },
        eventHelper: None,

        holdflush: None,

        haveTransparency: 0,
        haveTransparentBg: 0,
        haveRaster: 0,
        haveCapture: 0,
        haveLocator: 0,

        #[cfg(use_r_ge_version_14)]
        setPattern: None,
        #[cfg(use_r_ge_version_14)]
        releasePattern: None,

        #[cfg(use_r_ge_version_14)]
        setClipPath: None,
        #[cfg(use_r_ge_version_14)]
        releaseClipPath: None,

        #[cfg(use_r_ge_version_14)]
        setMask: None,
        #[cfg(use_r_ge_version_14)]
        releaseMask: None,

        #[cfg(use_r_ge_version_14)]
        deviceVersion: R_GE_definitions as _,

        #[cfg(use_r_ge_version_14)]
        deviceClip: 0,

        #[cfg(use_r_ge_version_15)]
        defineGroup: None,
        #[cfg(use_r_ge_version_15)]
        useGroup: None,
        #[cfg(use_r_ge_version_15)]
        releaseGroup: None,

        #[cfg(use_r_ge_version_15)]
        stroke: None,
        #[cfg(use_r_ge_version_15)]
        fill: None,
        #[cfg(use_r_ge_version_15)]
        fillStroke: None,

        #[cfg(use_r_ge_version_15)]
        capabilities: None,

        reserved: [0i8; 64],
    }
}
