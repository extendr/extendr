//! Graphics Engine support
use super::*;

pub const R_GE_version: u32 = 16;
// Types
pub type DevDesc = _DevDesc;
pub type pDevDesc = *mut DevDesc;
pub type pGEcontext = *mut R_GE_gcontext;
pub type pGEDevDesc = *mut GEDevDesc;

// Constants
pub const LTY_BLANK: i32 = -1;
pub const LTY_SOLID: u32 = 0;
pub const LTY_DASHED: u32 = 68;
pub const LTY_DOTTED: u32 = 49;
pub const LTY_DOTDASH: u32 = 13361;
pub const LTY_LONGDASH: u32 = 55;
pub const LTY_TWODASH: u32 = 9762;
pub const R_GE_definitions: u32 = 13;

// Enums
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GEUnit {
    GE_DEVICE = 0,
    GE_NDC = 1,
    GE_INCHES = 2,
    GE_CM = 3,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum R_GE_lineend {
    GE_ROUND_CAP = 1,
    GE_BUTT_CAP = 2,
    GE_SQUARE_CAP = 3,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum R_GE_linejoin {
    GE_ROUND_JOIN = 1,
    GE_MITRE_JOIN = 2,
    GE_BEVEL_JOIN = 3,
}

// Structs
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_GE_gcontext {
    pub col: ::std::os::raw::c_int,
    pub fill: ::std::os::raw::c_int,
    pub gamma: f64,
    pub lwd: f64,
    pub lty: ::std::os::raw::c_int,
    pub lend: R_GE_lineend,
    pub ljoin: R_GE_linejoin,
    pub lmitre: f64,
    pub cex: f64,
    pub ps: f64,
    pub lineheight: f64,
    pub fontface: ::std::os::raw::c_int,
    pub fontfamily: [::std::os::raw::c_char; 201usize],
    pub patternFill: SEXP,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DevDesc {
    #[doc = "left raster coordinate"]
    pub left: f64,
    #[doc = "right raster coordinate"]
    pub right: f64,
    #[doc = "bottom raster coordinate"]
    pub bottom: f64,
    #[doc = "top raster coordinate"]
    pub top: f64,
    #[doc = "R only has the notion of a rectangular clipping region"]
    pub clipLeft: f64,
    pub clipRight: f64,
    pub clipBottom: f64,
    pub clipTop: f64,
    #[doc = "x character addressing offset - unused"]
    pub xCharOffset: f64,
    #[doc = "y character addressing offset"]
    pub yCharOffset: f64,
    #[doc = "1/2 interline space as frac of line height"]
    pub yLineBias: f64,
    #[doc = "Inches per raster; \\[0\\]=x, \\[1\\]=y"]
    pub ipr: [f64; 2usize],
    #[doc = "Character size in rasters; \\[0\\]=x, \\[1\\]=y"]
    pub cra: [f64; 2usize],
    #[doc = "(initial) Device Gamma Correction"]
    pub gamma: f64,
    #[doc = "Device-level clipping"]
    pub canClip: Rboolean,
    #[doc = "can the gamma factor be modified?"]
    pub canChangeGamma: Rboolean,
    #[doc = "Can do at least some horiz adjust of text\n0 = none, 1 = {0,0.5,1}, 2 = \\[0,1\\]"]
    pub canHAdj: ::std::os::raw::c_int,
    #[doc = "Device initial settings\n/\n/* These are things that the device must set up when it is created.\n The graphics system can modify them and track current values,"]
    pub startps: f64,
    #[doc = "sets par(\"fg\"), par(\"col\") and gpar(\"col\")"]
    pub startcol: ::std::os::raw::c_int,
    #[doc = "sets par(\"bg\") and gpar(\"fill\")"]
    pub startfill: ::std::os::raw::c_int,
    pub startlty: ::std::os::raw::c_int,
    pub startfont: ::std::os::raw::c_int,
    pub startgamma: f64,
    #[doc = "pointer to device specific parameters"]
    pub deviceSpecific: *mut ::std::os::raw::c_void,
    #[doc = "toggle for initial display list status"]
    pub displayListOn: Rboolean,
    #[doc = "can the device generate mousedown events"]
    pub canGenMouseDown: Rboolean,
    #[doc = "can the device generate mousemove events"]
    pub canGenMouseMove: Rboolean,
    #[doc = "can the device generate mouseup events"]
    pub canGenMouseUp: Rboolean,
    #[doc = "can the device generate keyboard events"]
    pub canGenKeybd: Rboolean,
    #[doc = "can the device generate idle events"]
    pub canGenIdle: Rboolean,
    #[doc = "This is set while getGraphicsEvent\nis actively looking for events"]
    pub gettingEvent: Rboolean,
    pub activate: ::std::option::Option<unsafe extern "C" fn(arg1: pDevDesc)>,
    pub circle: ::std::option::Option<
        unsafe extern "C" fn(x: f64, y: f64, r: f64, gc: pGEcontext, dd: pDevDesc),
    >,
    pub clip: ::std::option::Option<
        unsafe extern "C" fn(x0: f64, x1: f64, y0: f64, y1: f64, dd: pDevDesc),
    >,
    pub close: ::std::option::Option<unsafe extern "C" fn(dd: pDevDesc)>,
    pub deactivate: ::std::option::Option<unsafe extern "C" fn(arg1: pDevDesc)>,
    pub locator: ::std::option::Option<
        unsafe extern "C" fn(x: *mut f64, y: *mut f64, dd: pDevDesc) -> Rboolean,
    >,
    pub line: ::std::option::Option<
        unsafe extern "C" fn(x1: f64, y1: f64, x2: f64, y2: f64, gc: pGEcontext, dd: pDevDesc),
    >,
    pub metricInfo: ::std::option::Option<
        unsafe extern "C" fn(
            c: ::std::os::raw::c_int,
            gc: pGEcontext,
            ascent: *mut f64,
            descent: *mut f64,
            width: *mut f64,
            dd: pDevDesc,
        ),
    >,
    pub mode:
        ::std::option::Option<unsafe extern "C" fn(mode: ::std::os::raw::c_int, dd: pDevDesc)>,
    pub newPage: ::std::option::Option<unsafe extern "C" fn(gc: pGEcontext, dd: pDevDesc)>,
    pub polygon: ::std::option::Option<
        unsafe extern "C" fn(
            n: ::std::os::raw::c_int,
            x: *mut f64,
            y: *mut f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    pub polyline: ::std::option::Option<
        unsafe extern "C" fn(
            n: ::std::os::raw::c_int,
            x: *mut f64,
            y: *mut f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    pub rect: ::std::option::Option<
        unsafe extern "C" fn(x0: f64, y0: f64, x1: f64, y1: f64, gc: pGEcontext, dd: pDevDesc),
    >,
    pub path: ::std::option::Option<
        unsafe extern "C" fn(
            x: *mut f64,
            y: *mut f64,
            npoly: ::std::os::raw::c_int,
            nper: *mut ::std::os::raw::c_int,
            winding: Rboolean,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    pub raster: ::std::option::Option<
        unsafe extern "C" fn(
            raster: *mut ::std::os::raw::c_uint,
            w: ::std::os::raw::c_int,
            h: ::std::os::raw::c_int,
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
    pub cap: ::std::option::Option<unsafe extern "C" fn(dd: pDevDesc) -> SEXP>,
    pub size: ::std::option::Option<
        unsafe extern "C" fn(
            left: *mut f64,
            right: *mut f64,
            bottom: *mut f64,
            top: *mut f64,
            dd: pDevDesc,
        ),
    >,
    pub strWidth: ::std::option::Option<
        unsafe extern "C" fn(
            str_: *const ::std::os::raw::c_char,
            gc: pGEcontext,
            dd: pDevDesc,
        ) -> f64,
    >,
    pub text: ::std::option::Option<
        unsafe extern "C" fn(
            x: f64,
            y: f64,
            str_: *const ::std::os::raw::c_char,
            rot: f64,
            hadj: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    pub onExit: ::std::option::Option<unsafe extern "C" fn(dd: pDevDesc)>,
    #[doc = "device_getEvent is no longer used, but the slot is kept for back\n compatibility of the structure."]
    pub getEvent: ::std::option::Option<
        unsafe extern "C" fn(arg1: SEXP, arg2: *const ::std::os::raw::c_char) -> SEXP,
    >,
    pub newFrameConfirm: ::std::option::Option<unsafe extern "C" fn(dd: pDevDesc) -> Rboolean>,
    #[doc = "and strWidthUTF8"]
    pub hasTextUTF8: Rboolean,
    pub textUTF8: ::std::option::Option<
        unsafe extern "C" fn(
            x: f64,
            y: f64,
            str_: *const ::std::os::raw::c_char,
            rot: f64,
            hadj: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    pub strWidthUTF8: ::std::option::Option<
        unsafe extern "C" fn(
            str_: *const ::std::os::raw::c_char,
            gc: pGEcontext,
            dd: pDevDesc,
        ) -> f64,
    >,
    pub wantSymbolUTF8: Rboolean,
    #[doc = "Is rotated text good enough to be preferable to Hershey in\ncontour labels?  Old default was FALSE."]
    pub useRotatedTextInContour: Rboolean,
    #[doc = "This is an environment holding event handlers."]
    pub eventEnv: SEXP,
    pub eventHelper:
        ::std::option::Option<unsafe extern "C" fn(dd: pDevDesc, code: ::std::os::raw::c_int)>,
    pub holdflush: ::std::option::Option<
        unsafe extern "C" fn(dd: pDevDesc, level: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    #[doc = "1 = no, 2 = yes"]
    pub haveTransparency: ::std::os::raw::c_int,
    #[doc = "1 = no, 2 = fully, 3 = semi"]
    pub haveTransparentBg: ::std::os::raw::c_int,
    #[doc = "1 = no, 2 = yes, 3 = except for missing values"]
    pub haveRaster: ::std::os::raw::c_int,
    #[doc = "1 = no, 2 = yes"]
    pub haveCapture: ::std::os::raw::c_int,
    #[doc = "1 = no, 2 = yes"]
    pub haveLocator: ::std::os::raw::c_int,
    pub setPattern:
        ::std::option::Option<unsafe extern "C" fn(pattern: SEXP, dd: pDevDesc) -> SEXP>,
    pub releasePattern: ::std::option::Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,
    pub setClipPath:
        ::std::option::Option<unsafe extern "C" fn(path: SEXP, ref_: SEXP, dd: pDevDesc) -> SEXP>,
    pub releaseClipPath: ::std::option::Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,
    pub setMask:
        ::std::option::Option<unsafe extern "C" fn(path: SEXP, ref_: SEXP, dd: pDevDesc) -> SEXP>,
    pub releaseMask: ::std::option::Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,
    #[doc = "This should match R_GE_version,\n BUT it does not have to.\n It give the graphics engine a chance to work with\n graphics device packages BEFORE they update to\n changes in R_GE_version."]
    pub deviceVersion: ::std::os::raw::c_int,
    #[doc = "This can be used to OVERRIDE canClip so that graphics engine\n leaves ALL clipping to the graphics device"]
    pub deviceClip: Rboolean,
    pub defineGroup: ::std::option::Option<
        unsafe extern "C" fn(
            source: SEXP,
            op: ::std::os::raw::c_int,
            destination: SEXP,
            dd: pDevDesc,
        ) -> SEXP,
    >,
    pub useGroup:
        ::std::option::Option<unsafe extern "C" fn(ref_: SEXP, trans: SEXP, dd: pDevDesc)>,
    pub releaseGroup: ::std::option::Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,
    pub stroke:
        ::std::option::Option<unsafe extern "C" fn(path: SEXP, gc: pGEcontext, dd: pDevDesc)>,
    pub fill: ::std::option::Option<
        unsafe extern "C" fn(path: SEXP, rule: ::std::os::raw::c_int, gc: pGEcontext, dd: pDevDesc),
    >,
    pub fillStroke: ::std::option::Option<
        unsafe extern "C" fn(path: SEXP, rule: ::std::os::raw::c_int, gc: pGEcontext, dd: pDevDesc),
    >,
    pub capabilities: ::std::option::Option<unsafe extern "C" fn(cap: SEXP) -> SEXP>,
    pub glyph: ::std::option::Option<
        unsafe extern "C" fn(
            n: ::std::os::raw::c_int,
            glyphs: *mut ::std::os::raw::c_int,
            x: *mut f64,
            y: *mut f64,
            font: SEXP,
            size: f64,
            colour: ::std::os::raw::c_int,
            rot: f64,
            dd: pDevDesc,
        ),
    >,
    #[doc = "Area for future expansion.\nBy zeroing this, devices are more likely to work if loaded\ninto a later version of R than that they were compiled under."]
    pub reserved: [::std::os::raw::c_char; 64usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GEDevDesc {
    #[doc = "Stuff that the devices can see (and modify).\n All detailed in GraphicsDevice.h"]
    pub dev: pDevDesc,
    #[doc = "toggle for display list status"]
    pub displayListOn: Rboolean,
    #[doc = "display list"]
    pub displayList: SEXP,
    #[doc = "A pointer to the end of the display list\nto avoid traversing pairlists"]
    pub DLlastElt: SEXP,
    #[doc = "The last element of the display list\n just prior to when the display list\n was last initialised"]
    pub savedSnapshot: SEXP,
    #[doc = "Has the device received any output?"]
    pub dirty: Rboolean,
    #[doc = "Should a graphics call be stored\n on the display list?\n Set to FALSE by do_recordGraphics,\n do_dotcallgr, and do_Externalgr\n so that nested calls are not\n recorded on the display list"]
    pub recordGraphics: Rboolean,
    #[doc = "Stuff about the device that only graphics systems see.\n The graphics engine has no idea what is in here.\n Used by graphics systems to store system state per device."]
    pub gesd: [*mut GESystemDesc; 24usize],
    #[doc = "per-device setting for 'ask' (use NewFrameConfirm)"]
    pub ask: Rboolean,
    #[doc = "Is a device appending a path ?"]
    pub appending: Rboolean,
}

pub type GEcallback = ::std::option::Option<
    unsafe extern "C" fn(arg1: GEevent, arg2: *mut GEDevDesc, arg3: SEXP) -> SEXP,
>;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GEevent {
    #[doc = "In response to this event, the registered graphics system\n should allocate and initialise the systemSpecific structure\n\n Should return R_NilValue on failure so that engine\n can tidy up memory allocation"]
    GE_InitState = 0,
    #[doc = "This event gives the registered system a chance to undo\n anything done in the initialisation."]
    GE_FinaliseState = 1,
    #[doc = "This is sent by the graphics engine prior to initialising\n the display list.  It give the graphics system the chance\n to squirrel away information it will need for redrawing the\n the display list"]
    GE_SaveState = 2,
    #[doc = "This is sent by the graphics engine prior to replaying the\n display list.  It gives the graphics system the chance to\n restore any information it saved on the GE_SaveState event"]
    GE_RestoreState = 6,
    #[doc = "Copy system state information to the current device.\n This is used when copying graphics from one device to another\n so all the graphics system needs to do is to copy across\n the bits required for the display list to draw faithfully\n on the new device."]
    GE_CopyState = 3,
    #[doc = "Create a snapshot of the system state that is sufficient\n for the current \"image\" to be reproduced"]
    GE_SaveSnapshotState = 4,
    #[doc = "Restore the system state that is saved by GE_SaveSnapshotState"]
    GE_RestoreSnapshotState = 5,
    #[doc = "When replaying the display list, the graphics engine\n checks, after each replayed action, that the action\n produced valid output.  This is the graphics system's\n chance to say that the output is crap (in which case the\n graphics engine will abort the display list replay)."]
    GE_CheckPlot = 7,
    #[doc = "The device wants to scale the current pointsize\n (for scaling an image)\n This is not a nice general solution, but a quick fix for\n the Windows device."]
    GE_ScalePS = 8,
}

pub type GEDevDesc = _GEDevDesc;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GESystemDesc {
    #[doc = "An array of information about each graphics system that\n has registered with the graphics engine.\n This is used to store graphics state for each graphics\n system on each device."]
    pub systemSpecific: *mut ::std::os::raw::c_void,
    #[doc = "An array of function pointers, one per graphics system that\n has registered with the graphics engine.\n\n system_Callback is called when the graphics engine wants\n to give a graphics system the chance to play with its\n device-specific information (stored in systemSpecific)\n There are two parameters:  an \"event\" to tell the graphics\n system why the graphics engine has called this function,\n and the systemSpecific pointer.  The graphics engine\n has to pass the systemSpecific pointer because only\n the graphics engine will know what array index to use."]
    pub callback: GEcallback,
}

extern "C" {

    // Functions
    pub fn GEaddDevice2(arg1: pGEDevDesc, arg2: *const ::std::os::raw::c_char);
    pub fn GECap(dd: pGEDevDesc) -> SEXP;
    pub fn GECircle(x: f64, y: f64, radius: f64, gc: pGEcontext, dd: pGEDevDesc);
    pub fn GEcreateDevDesc(dev: pDevDesc) -> pGEDevDesc;
    pub fn GEcurrentDevice() -> pGEDevDesc;
    pub fn GEdeviceNumber(arg1: pGEDevDesc) -> ::std::os::raw::c_int;
    pub fn GEExpressionHeight(expr: SEXP, gc: pGEcontext, dd: pGEDevDesc) -> f64;
    pub fn GEExpressionMetric(
        expr: SEXP,
        gc: pGEcontext,
        ascent: *mut f64,
        descent: *mut f64,
        width: *mut f64,
        dd: pGEDevDesc,
    );
    pub fn GEExpressionWidth(expr: SEXP, gc: pGEcontext, dd: pGEDevDesc) -> f64;
    pub fn GEfromDeviceHeight(value: f64, to: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEfromDeviceWidth(value: f64, to: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEfromDeviceX(value: f64, to: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEfromDeviceY(value: f64, to: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEgetDevice(arg1: ::std::os::raw::c_int) -> pGEDevDesc;
    pub fn GEinitDisplayList(dd: pGEDevDesc);
    pub fn GELine(x1: f64, y1: f64, x2: f64, y2: f64, gc: pGEcontext, dd: pGEDevDesc);
    pub fn GEMathText(
        x: f64,
        y: f64,
        expr: SEXP,
        xc: f64,
        yc: f64,
        rot: f64,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GEMetricInfo(
        c: ::std::os::raw::c_int,
        gc: pGEcontext,
        ascent: *mut f64,
        descent: *mut f64,
        width: *mut f64,
        dd: pGEDevDesc,
    );
    pub fn GEMode(mode: ::std::os::raw::c_int, dd: pGEDevDesc);
    pub fn GENewPage(gc: pGEcontext, dd: pGEDevDesc);
    pub fn GEPath(
        x: *mut f64,
        y: *mut f64,
        npoly: ::std::os::raw::c_int,
        nper: *mut ::std::os::raw::c_int,
        winding: Rboolean,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GEPolygon(
        n: ::std::os::raw::c_int,
        x: *mut f64,
        y: *mut f64,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GEPolyline(
        n: ::std::os::raw::c_int,
        x: *mut f64,
        y: *mut f64,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GERaster(
        raster: *mut ::std::os::raw::c_uint,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        rot: f64,
        interpolate: Rboolean,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GERect(x0: f64, y0: f64, x1: f64, y1: f64, gc: pGEcontext, dd: pGEDevDesc);
    pub fn GESetClip(x1: f64, y1: f64, x2: f64, y2: f64, dd: pGEDevDesc);
    pub fn GEStrHeight(
        str_: *const ::std::os::raw::c_char,
        enc: cetype_t,
        gc: pGEcontext,
        dd: pGEDevDesc,
    ) -> f64;
    pub fn GEStrMetric(
        str_: *const ::std::os::raw::c_char,
        enc: cetype_t,
        gc: pGEcontext,
        ascent: *mut f64,
        descent: *mut f64,
        width: *mut f64,
        dd: pGEDevDesc,
    );
    pub fn GEStrWidth(
        str_: *const ::std::os::raw::c_char,
        enc: cetype_t,
        gc: pGEcontext,
        dd: pGEDevDesc,
    ) -> f64;
    pub fn GESymbol(
        x: f64,
        y: f64,
        pch: ::std::os::raw::c_int,
        size: f64,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GEText(
        x: f64,
        y: f64,
        str_: *const ::std::os::raw::c_char,
        enc: cetype_t,
        xc: f64,
        yc: f64,
        rot: f64,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GEtoDeviceHeight(value: f64, from: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEtoDeviceWidth(value: f64, from: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEtoDeviceX(value: f64, from: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEtoDeviceY(value: f64, from: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn R_CheckDeviceAvailable();
    pub fn R_GE_checkVersionOrDie(version: ::std::os::raw::c_int);
}
