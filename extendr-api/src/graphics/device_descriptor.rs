use super::{color::Color, FontFace, LineType};

// From R internals[^1]:
//
// > There should be a ‘pointsize’ argument which defaults to 12, and it should
// > give the pointsize in big points (1/72 inch). How exactly this is
// > interpreted is font-specific, but it should use a font which works with
// > lines packed 1/6 inch apart, and looks good with lines 1/5 inch apart (that
// > is with 2pt leading).
//
// [^1]: https://cran.r-project.org/doc/manuals/r-release/R-ints.html#Conventions
const POINTSIZE: f64 = 12.0;

const PT: f64 = 1.0 / 72.0;
const PT_PER_INCH: f64 = 72.0;

// From R internals[^1]:
//
// > where ‘fnsize’ is the ‘size’ of the standard font (cex=1) on the device, in
// > device units.
//
// and it seems the Postscript device chooses `pointsize` as this.
//
// [^1]: https://cran.r-project.org/doc/manuals/r-release/R-ints.html#Handling-text
const FONTSIZE: f64 = POINTSIZE;

// From R internals[^1]:
//
// > The default size of a device should be 7 inches square.
//
// [^1]: https://cran.r-project.org/doc/manuals/r-release/R-ints.html#Conventions
const WIDTH_INCH: f64 = 7.0;
const HEIGH_INCH: f64 = 7.0;

#[allow(dead_code)]
pub(crate) enum CanHAdjOption {
    NotSupported = 0,
    FixedAdjustment = 1,
    VariableAdjustment = 2,
}

pub enum DevCapTransparency {
    Unset = 0,
    No = 1,
    Yes = 2,
}

pub enum DevCapTransparentBg {
    Unset = 0,
    No = 1,
    Fully = 2,
    Semi = 3,
}

#[allow(dead_code)]
pub(crate) enum DevCapRaster {
    Unset = 0,
    No = 1,
    Yes = 2,
    ExceptForMissingValues = 3,
}

#[allow(dead_code)]
pub(crate) enum DevCapCapture {
    Unset = 0,
    No = 1,
    Yes = 2,
}

#[allow(dead_code)]
pub(crate) enum DevCapLocator {
    Unset = 0,
    No = 1,
    Yes = 2,
}

/// A builder of [libR_sys::DevDesc].
///
// # Design notes (which feels a bit too internal to be exposed as an official document)
//
// Compared to the original [DevDesc], `DeviceDescriptor` omits several fields
// that seem not very useful. For example,
//
// - `clipLeft`, `clipRight`, `clipBottom`, and `clipTop`: In most of the cases,
//   this should match the device size at first.
// - `xCharOffset`, `yCharOffset`, and `yLineBias`: Because I get [the
//   hatred](https://github.com/wch/r-source/blob/9f284035b7e503aebe4a804579e9e80a541311bb/src/include/R_ext/GraphicsDevice.h#L101-L103).
//   They are rarely used.
// - `gamma`, and `canChangeGamma`: These fields are now ignored because gamma
//   support has been removed.
// - `deviceSpecific`: This can be provided later when we actually create a
//   [Device].
// - `canGenMouseDown`, `canGenMouseMove`, `canGenMouseUp`, `canGenKeybd`, and
//   `canGenIdle`: These fields are currently not used by R and preserved only
//   for backward-compatibility.
// - `gettingEvent`, `getEvent`: This is set true when getGraphicsEvent is
//   actively looking for events. Reading the description on ["6.1.6 Graphics
//   events" of R
//   Internals](https://cran.r-project.org/doc/manuals/r-release/R-ints.html#Graphics-events),
//   it seems this flag is not what is controlled by a graphic device.
// - `canHAdj`: it seems this parameter is used only for tweaking the `hadj`
//   before passing it to the `text()` function. This tweak probably can be done
//   inside `text()` easily, so let's pretend to be able to handle any
//   adjustments... c.f.
//   <https://github.com/wch/r-source/blob/9f284035b7e503aebe4a804579e9e80a541311bb/src/main/engine.c#L1995-L2000>
#[allow(non_snake_case)]
pub struct DeviceDescriptor {
    pub(crate) left: f64,
    pub(crate) right: f64,
    pub(crate) bottom: f64,
    pub(crate) top: f64,

    // Note: the header file questions about `ipr` and `cra` [1]. Actually,
    // svglite and ragg have `pointsize` and `scaling` parameters instead. But,
    // I couldn't be sure if it's enough as an framework (I mean, as a package,
    // abstracting these parameters to `pointsize` and `scaling` is definitely a
    // good idea), so I chose to expose these parameters as they are.
    //
    // [1]:
    //     https://github.com/wch/r-source/blob/9f284035b7e503aebe4a804579e9e80a541311bb/src/include/R_ext/GraphicsDevice.h#L75-L81
    pub(crate) ipr: [f64; 2],
    pub(crate) cra: [f64; 2],

    pub(crate) startps: f64,
    pub(crate) startcol: Color,
    pub(crate) startfill: Color,
    pub(crate) startlty: LineType,
    pub(crate) startfont: FontFace,
}

#[allow(non_snake_case)]
impl DeviceDescriptor {
    pub fn new() -> Self {
        Self {
            // The From R internals [1] " The default size of a device should be 7
            // inches square."
            left: 0.0,
            right: WIDTH_INCH * PT_PER_INCH,
            bottom: 0.0,
            top: HEIGH_INCH * PT_PER_INCH,

            ipr: [PT, PT],

            // Font size. Not sure why these 0.9 and 1.2 are chosen, but R
            // internals says this is "a good choice."
            cra: [0.9 * FONTSIZE, 1.2 * FONTSIZE],

            startps: POINTSIZE,
            startcol: Color::hex(0x000000),
            startfill: Color::hex(0xffffff),
            startlty: LineType::Solid,
            startfont: FontFace::Plain,
        }
    }

    /// Sets the device sizes (unit: point).
    ///
    /// If not specified, the following numbers (7 inches square, following [the
    /// R Internals' convetion]) will be used.
    ///
    /// * `left`: 0
    /// * `right`: 7 inches * points per inch = `7 * 72`
    /// * `bottom`: 0
    /// * `top`: 7 inches * points per inch = `7 * 72`
    ///
    ///  Please note that, depending on the the coordinate system of the device,
    ///  `left` might be larger than `right`, or `bottom` larger than `top` (for
    ///  example, in SVG, the origin is at the top left corner).
    ///
    /// [the R Internals' convetion]:
    ///     https://cran.r-project.org/doc/manuals/r-release/R-ints.html#Conventions
    pub fn device_size(mut self, left: f64, right: f64, bottom: f64, top: f64) -> Self {
        self.left = left;
        self.right = right;
        self.bottom = bottom;
        self.top = top;
        self
    }

    /// Sets inches per raster unit (i.e. point). **Note that most of the cases,
    /// there's no need to change this value.**
    ///
    /// A point is usually 1/72 (the default value), but another value can be
    /// specified here to scale the device. The first element is width, the
    /// second is height.
    pub fn ipr(mut self, ipr: [f64; 2]) -> Self {
        self.ipr = ipr;
        self
    }

    /// Sets the font size (unit: point). **Note that most of the cases, there's
    /// no need to change this value.**
    ///
    /// The first element is width, the second is height. If not specified,
    /// `[0.9 * 12.0, 1.2 * 12.0]`, which is [suggested by the R Internals as "a
    /// good choice"] will be used (12 point is the usual default for graphics
    /// devices).
    ///
    /// [suggested by the R Internals as "a good choice"]:
    ///     https://cran.r-project.org/doc/manuals/r-release/R-ints.html#Handling-text
    pub fn cra(mut self, cra: [f64; 2]) -> Self {
        self.cra = cra;
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
    /// If not specified, [FontFace::Plain] will be used.
    pub fn startfont(mut self, startfont: FontFace) -> Self {
        self.startfont = startfont;
        self
    }
}

impl Default for DeviceDescriptor {
    fn default() -> Self {
        Self::new()
    }
}
