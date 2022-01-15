use crate::*;
use libR_sys::*;

use super::color;

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
        //
        // I couldn't follow how this `startfont` is used, but it seems this "font"
        // means "font face", considering `GPar`'s `font` is set to `fontface` of
        // `pGEcontext` (c.f. https://github.com/wch/r-source/blob/9f284035b7e503aebe4a804579e9e80a541311bb/src/library/graphics/src/graphics.c#L2568).
        // Anyway, as `GInit()` sets `1`, it seems we can use the same number here.
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

        // R internals says:
        //
        // Some devices can produce high-quality rotated text, but those based on
        // bitmaps often cannot. Those which can should set useRotatedTextInContour
        // to be true from graphics API version 4.
        //
        // It seems this is used only by plot3d, so FALSE should be appropriate in
        // most of the cases.
        useRotatedTextInContour: 0,

        // If the graphic device is to handle user interaction, set these. For more
        // details can be found at
        // https://cran.r-project.org/doc/manuals/r-devel/R-ints.html#Graphics-events
        eventEnv: unsafe { R_EmptyEnv },
        eventHelper: None,

        // The header file says:
        //
        // Allows graphics devices to have multiple levels of suspension: when this
        // reaches zero output is flushed.
        holdflush: None,

        // Device capabilities. In all cases, 0 means NA (unset), and 1 means no.
        // It seems 2 or larger numbers typically represents "yes."
        haveTransparency: 0,
        haveTransparentBg: 0,
        haveRaster: 0,
        haveCapture: 0,
        haveLocator: 0,

        // Patterns and gradients (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/definitions/definitions.html#internals)
        #[cfg(use_r_ge_version_14)]
        setPattern: None,
        #[cfg(use_r_ge_version_14)]
        releasePattern: None,

        // Clipping paths (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/definitions/definitions.html#internals)
        #[cfg(use_r_ge_version_14)]
        setClipPath: None,
        #[cfg(use_r_ge_version_14)]
        releaseClipPath: None,

        // Masks (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/definitions/definitions.html#internals)
        #[cfg(use_r_ge_version_14)]
        setMask: None,
        #[cfg(use_r_ge_version_14)]
        releaseMask: todo!(),

        // Surprisingly, we can set the device version other than the actual graphic
        // device version (probably to avoid the "Graphics API version mismatch" error).
        #[cfg(use_r_ge_version_14)]
        deviceVersion: R_GE_definitions as _,

        // If TRUE, the graphic engine does no clipping and the device is supposed
        // to handle all of them.
        #[cfg(use_r_ge_version_14)]
        deviceClip: 0,

        // Group compositing operations and affine transformation
        // (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/groups/groups.html)
        #[cfg(use_r_ge_version_15)]
        defineGroup: None,
        #[cfg(use_r_ge_version_15)]
        useGroup: None,
        #[cfg(use_r_ge_version_15)]
        releaseGroup: None,

        // Stroking and filling paths (ref: https://www.stat.auckland.ac.nz/~paul/Reports/GraphicsEngine/paths/paths.html)
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
