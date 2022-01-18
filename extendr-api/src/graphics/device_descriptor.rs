use crate::*;
use libR_sys::*;

use super::{color::Color, device_driver::DeviceDriver, FontFace, LineType};

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
/// these fields that seem not very useful:
///
/// - `clipLeft`, `clipRight`, `clipBottom`, and `clipTop`: In most of the
///   cases, this should match the device size at first.
/// - `xCharOffset`, `yCharOffset`, and `yLineBias`: Because I get [the
///   hatred](https://github.com/wch/r-source/blob/9f284035b7e503aebe4a804579e9e80a541311bb/src/include/R_ext/GraphicsDevice.h#L101-L103).
///   They are rarely used.
/// - `gamma`, and `canChangeGamma`: These fields are now ignored because gamma
///   support has been removed.
/// - `deviceSpecific`: This can be provided later when we actually create a
///   [Device].
/// - `canGenMouseDown`, `canGenMouseMove`, `canGenMouseUp`, `canGenKeybd`, and
///   `canGenIdle`: These fields are currently not used by R and preserved only
///   for backward-compatibility.
/// - `gettingEvent`, `getEvent`: This is set true when getGraphicsEvent is
///   actively looking for events. Reading the description on ["6.1.6 Graphics
///   events" of R
///   Internals](https://cran.r-project.org/doc/manuals/r-devel/R-ints.html#Graphics-events),
///   it seems this flag is not what is controlled by a graphic device.
#[allow(non_snake_case)]
pub struct DeviceDescriptor {
    pub left: f64,
    pub right: f64,
    pub bottom: f64,
    pub top: f64,

    pub ipr: [f64; 2],

    pub cra: [f64; 2],

    pub canClip: bool,

    pub canHAdj: CanHAdjOption,

    pub startps: f64,
    pub startcol: Color,
    pub startfill: Color,
    pub startlty: LineType,
    pub startfont: FontFace,

    pub displayListOn: bool,

    // UTF-8 support
    pub hasTextUTF8: bool,
    pub textUTF8: Option<
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
    pub strWidthUTF8: Option<
        unsafe extern "C" fn(str: *const std::os::raw::c_char, gc: pGEcontext, dd: pDevDesc) -> f64,
    >,
    pub wantSymbolUTF8: bool,

    // R internals says:
    //
    //     Some devices can produce high-quality rotated text, but those based on
    //     bitmaps often cannot. Those which can should set useRotatedTextInContour
    //     to be true from graphics API version 4.
    //
    // It seems this is used only by plot3d, so FALSE should be appropriate in
    // most of the cases.
    pub useRotatedTextInContour: bool,

    /// If the graphic device is to handle user interaction, set these. For more
    /// details can be found on R Internals:
    ///  
    /// https://cran.r-project.org/doc/manuals/r-devel/R-ints.html#Graphics-events
    pub eventEnv: Environment,
    pub eventHelper: Option<unsafe extern "C" fn(dd: pDevDesc, code: std::os::raw::c_int)>,

    /// The header file says:
    ///
    /// Allows graphics devices to have multiple levels of suspension: when this
    /// reaches zero output is flushed.
    pub holdflush: Option<
        unsafe extern "C" fn(dd: pDevDesc, level: std::os::raw::c_int) -> std::os::raw::c_int,
    >,

    /// Device capabilities. In all cases, 0 means NA (unset), and 1 means no.
    /// It seems 2 or larger numbers typically represents "yes."
    pub haveTransparency: GraphicDeviceCapabilityTransparency,
    pub haveTransparentBg: GraphicDeviceCapabilityTransparentBg,
    pub haveRaster: GraphicDeviceCapabilityRaster,
    pub haveCapture: GraphicDeviceCapabilityCapture,
    pub haveLocator: GraphicDeviceCapabilityLocator,

    /// Patterns and gradients (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/definitions/definitions.html#internals)
    #[cfg(use_r_ge_version_14)]
    pub setPattern: Option<unsafe extern "C" fn(pattern: SEXP, dd: pDevDesc) -> SEXP>,

    /// Patterns and gradients (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/definitions/definitions.html#internals)
    #[cfg(use_r_ge_version_14)]
    pub releasePattern: Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,

    /// Clipping paths (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/definitions/definitions.html#internals)
    #[cfg(use_r_ge_version_14)]
    pub setClipPath: Option<unsafe extern "C" fn(path: SEXP, ref_: SEXP, dd: pDevDesc) -> SEXP>,

    /// Clipping paths (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/definitions/definitions.html#internals)
    #[cfg(use_r_ge_version_14)]
    pub releaseClipPath: Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,

    /// Masks (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/definitions/definitions.html#internals)
    #[cfg(use_r_ge_version_14)]
    pub setMask: Option<unsafe extern "C" fn(path: SEXP, ref_: SEXP, dd: pDevDesc) -> SEXP>,

    /// Masks (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/definitions/definitions.html#internals)
    #[cfg(use_r_ge_version_14)]
    pub releaseMask: Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,

    /// The version of the graphic device API. Surprisingly, we can set the
    /// device version other than the actual graphic device version (probably to
    /// avoid the "Graphics API version mismatch" error).
    #[cfg(use_r_ge_version_14)]
    pub deviceVersion: u32,

    /// If TRUE, the graphic engine does no clipping and the device is supposed
    /// to handle all of them.
    #[cfg(use_r_ge_version_14)]
    pub deviceClip: bool,

    /// Group compositing operations and affine transformation (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/groups/groups.html)
    #[cfg(use_r_ge_version_15)]
    pub defineGroup: Option<
        unsafe extern "C" fn(
            source: SEXP,
            op: ::std::os::raw::c_int,
            destination: SEXP,
            dd: pDevDesc,
        ) -> SEXP,
    >,

    /// Group compositing operations and affine transformation (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/groups/groups.html)
    #[cfg(use_r_ge_version_15)]
    pub useGroup: Option<unsafe extern "C" fn(ref_: SEXP, trans: SEXP, dd: pDevDesc)>,

    /// Group compositing operations and affine transformation (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/groups/groups.html)
    #[cfg(use_r_ge_version_15)]
    pub releaseGroup: Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,

    /// Stroking and filling paths (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/paths/paths.html)
    #[cfg(use_r_ge_version_15)]
    pub stroke: Option<unsafe extern "C" fn(path: SEXP, gc: pGEcontext, dd: pDevDesc)>,

    /// Stroking and filling paths (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/paths/paths.html)
    #[cfg(use_r_ge_version_15)]
    pub fill: Option<
        unsafe extern "C" fn(path: SEXP, rule: ::std::os::raw::c_int, gc: pGEcontext, dd: pDevDesc),
    >,

    /// Stroking and filling paths (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/paths/paths.html)
    #[cfg(use_r_ge_version_15)]
    pub fillStroke: Option<
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
    pub capabilities: ::std::option::Option<unsafe extern "C" fn(cap: SEXP) -> SEXP>,
}

#[allow(non_snake_case)]
impl DeviceDescriptor {
    pub fn new() -> Self {
        Self {
            // The R Internal says " The default size of a device should be 7
            // inches square."
            left: 0.0,
            right: WIDTH_INCH * PT_PER_INCH,
            bottom: HEIGH_INCH * PT_PER_INCH,
            top: 0.0,

            ipr: [PT, PT],

            // Font size. Not sure why these 0.9 and 1.2 are chosen, but R
            // internals says this is "a good choice."
            cra: [0.9 * FONTSIZE, 1.2 * FONTSIZE],

            canClip: false,

            canHAdj: CanHAdjOption::NotSupported,

            startps: POINTSIZE,
            startcol: Color::hex(0x000000),
            startfill: Color::hex(0xffffff),
            startlty: LineType::Solid,
            startfont: FontFace::PlainFont,

            // The header file says "toggle for initial display list status."
            // When we want to maintain a plot history, this should be turned on
            // so that `GEinitDisplayList` is invoked.
            displayListOn: false,

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

    /// Sets the device sizes (unit: point).
    ///
    /// If not specified, the following numbers (7 inches square, following [the
    /// R Internals' convetion]) will be used.
    ///
    /// * `left`: 0
    /// * `right`: 7 inches * points per inch = `7 * 72`
    /// * `bottom`: 7 inches * points per inch = `7 * 72`
    /// * `top`: 0
    ///
    /// [the R Internals' convetion]:
    ///     https://cran.r-project.org/doc/manuals/r-devel/R-ints.html#Conventions
    pub fn device_size(mut self, left: f64, right: f64, bottom: f64, top: f64) -> Self {
        self.left = left;
        self.right = right;
        self.bottom = bottom;
        self.top = top;
        self
    }

    /// Sets inches per raster unit (i.e. point).
    ///
    /// A point is usually 1/72 (the default value), but another value can be
    /// specified here to scale the device. The first element is width, the second
    /// is height.
    pub fn ipr(mut self, ipr: [f64; 2]) -> Self {
        self.ipr = ipr;
        self
    }

    /// Sets the font size (unit: point).
    ///
    /// The first element is width, the second is height. If not specified,
    /// `[0.9 * 12.0, 1.2 * 12.0]`, which is [suggested by the R Internals as "a
    /// good choice"] will be used (12 point is the usual default for graphics devices).
    ///
    /// [suggested by the R Internals as "a good choice"]:
    ///     https://cran.r-project.org/doc/manuals/r-devel/R-ints.html#Handling-text
    pub fn cra(mut self, cra: [f64; 2]) -> Self {
        self.cra = cra;
        self
    }

    // /// Sets the flag of whether the device can clip text.
    // ///
    // /// If not specified, `false` will be used. In that case, the graphic engine
    // /// will clip instead by omitting any "any text that does not appear to be
    // /// wholly inside the clipping region," according to [the R Internals],
    // /// which is not very ideal.
    // ///
    // /// [the R Internals]:
    // ///     https://cran.r-project.org/doc/manuals/r-devel/R-ints.html#Handling-text
    // pub fn canClip(mut self, canClip: bool) -> Self {
    //     self.canClip = canClip;
    //     self
    // }

    /// Sets the flag of whether the device can handle holizontal adjustment.
    ///
    /// If not specified [CanHAdjOption::NotSupported] will be used.
    pub fn canHAdj(mut self, canHAdj: CanHAdjOption) -> Self {
        self.canHAdj = canHAdj;
        self
    }

    /// Sets the initial value of pointsize.
    ///
    /// If not specified, 12, which is the usual default for graphics devices,
    /// will be used.
    pub fn startps(mut self, startps: f64) -> Self {
        self.startps = startps;
        self
    }

    /// Sets the initial value of colour.
    ///
    /// If not specified, black (`0x000000`) will be used.
    pub fn startcol(mut self, startcol: Color) -> Self {
        self.startcol = startcol;
        self
    }
    /// Sets the initial value of fill.
    ///
    /// If not specified, white (`0xffffff`) will be used.
    pub fn startfill(mut self, startfill: Color) -> Self {
        self.startfill = startfill;
        self
    }

    /// Sets the initial value of line type.
    ///
    /// If not specified, [LineType::Solid] will be used.
    pub fn startlty(mut self, startlty: LineType) -> Self {
        self.startlty = startlty;
        self
    }

    /// Sets the initial value of font face.
    ///
    /// If not specified, [FontFace::PlainFont] will be used.
    pub fn startfont(mut self, startfont: FontFace) -> Self {
        self.startfont = startfont;
        self
    }

    /// Sets the flag of whether the device maintain a plot history.
    ///
    /// If not specified, `false` will be used.
    pub fn displayListOn(mut self, displayListOn: bool) -> Self {
        self.displayListOn = displayListOn;
        self
    }
}

impl Default for DeviceDescriptor {
    fn default() -> Self {
        Self::new()
    }
}
