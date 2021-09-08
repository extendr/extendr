use crate::*;
use libR_sys::*;

pub struct Context {
    inner: R_GE_gcontext,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Device {
    inner: pGEDevDesc,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    inner: Robj,
}

impl Device {
    pub(crate) fn inner(&self) -> pGEDevDesc {
        self.inner
    }
}

pub enum LineEnd {
    RoundCap,
    ButtCap,
    SquareCap,
}

pub enum LineJoin {
    RoundJoin,
    MitreJoin,
    BevelJoin,
}

pub enum LineType {
    Blank,
    Solid,
    Dashed,
    Dotted,
    Dotdash,
    Longdash,
    Twodash,
}

// pub enum Mode {
//     Off,
//     On,
//     InputOn,
// }

pub enum FontFace {
    PlainFont,
    BoldFont,
    ItalicFont,
    BoldItalicFont,
    SymbolFont,
}

impl Context {
    pub fn new() -> Self {
        #[allow(unused_unsafe)]
        unsafe {
            let inner = R_GE_gcontext {
                col: -1,
                fill: -1,
                gamma: 1.0,
                lwd: 5.0,
                lty: 0,
                lend: R_GE_lineend_GE_ROUND_CAP,
                ljoin: R_GE_linejoin_GE_ROUND_JOIN,
                lmitre: 10.0,
                cex: 1.0,
                ps: 1.0,
                lineheight: 1.0,
                fontface: 0,
                fontfamily: [0; 201],

                #[cfg(use_r_patternfill)]
                patternFill: R_NilValue,
            };
            Self { inner }
        }
    }

    pub fn rgb(red: u8, green: u8, blue: u8) -> i32 {
        red as i32 | (green as i32) << 8 | (blue as i32) << 16 | 0xff << 24
    }

    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> i32 {
        red as i32 | (green as i32) << 8 | (blue as i32) << 16 | (alpha as i32) << 24
    }

    pub fn color(&mut self, col: i32) -> &mut Self {
        self.inner.col = col;
        self
    }

    pub fn fill(&mut self, fill: i32) -> &mut Self {
        self.inner.fill = fill;
        self
    }

    pub fn gamma(&mut self, gamma: f64) -> &mut Self {
        self.inner.gamma = gamma;
        self
    }

    pub fn line_width(&mut self, lwd: f64) -> &mut Self {
        self.inner.lwd = lwd;
        self
    }

    pub fn line_type(&mut self, lty: LineType) -> &mut Self {
        use LineType::*;
        self.inner.lty = match lty {
            Blank => -1,
            Solid => 0,
            Dashed => 4 + (4 << 4),
            Dotted => 1 + (3 << 4),
            Dotdash => 1 + (3 << 4) + (4 << 8) + (3 << 12),
            Longdash => 7 + (3 << 4),
            Twodash => 2 + (2 << 4) + (6 << 8) + (2 << 12),
        };
        self
    }

    pub fn line_end(&mut self, lend: LineEnd) -> &mut Self {
        self.inner.lend = match lend {
            LineEnd::RoundCap => 1,
            LineEnd::ButtCap => 2,
            LineEnd::SquareCap => 3,
        };
        self
    }

    pub fn line_join(&mut self, ljoin: LineJoin) -> &mut Self {
        self.inner.ljoin = match ljoin {
            LineJoin::RoundJoin => 1,
            LineJoin::MitreJoin => 2,
            LineJoin::BevelJoin => 3,
        };
        self
    }

    pub fn point_size(&mut self, ps: f64) -> &mut Self {
        self.inner.ps = ps;
        self
    }

    pub fn line_mitre(&mut self, lmitre: f64) -> &mut Self {
        self.inner.lmitre = lmitre;
        self
    }

    pub fn line_height(&mut self, lineheight: f64) -> &mut Self {
        self.inner.lineheight = lineheight;
        self
    }

    // pub fn char_extra_size(&mut self, cex: f64) -> &mut Self {
    //     self.inner.cex = cex;
    //     self
    // }

    pub fn font_face(&mut self, fontface: FontFace) -> &mut Self {
        use FontFace::*;
        self.inner.fontface = match fontface {
            PlainFont => 1,
            BoldFont => 2,
            ItalicFont => 3,
            BoldItalicFont => 4,
            SymbolFont => 5,
        };
        self
    }

    pub fn font_family(&mut self, fontfamily: &str) -> &mut Self {
        let maxlen = self.inner.fontfamily.len() - 1;

        for c in self.inner.fontfamily.iter_mut() {
            *c = 0;
        }

        for (i, b) in fontfamily.bytes().enumerate().take(maxlen) {
            self.inner.fontfamily[i] = b as std::os::raw::c_char;
        }
        self
    }

    pub(crate) fn inner(&self) -> pGEcontext {
        unsafe { std::mem::transmute(&self.inner) }
    }

    // pub (crate) fn inner_mut(&mut self) -> pGEcontext {
    //     &mut self.inner as pGEcontext
    // }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(non_snake_case)]
impl Device {
    pub fn current() -> Device {
        unsafe {
            Device {
                inner: GEcurrentDevice(),
            }
        }
    }

    pub fn off(&self) {
        unsafe {
            GEMode(0, self.inner());
        }
    }

    pub fn on(&self) {
        unsafe {
            GEMode(1, self.inner());
        }
    }

    pub fn deviceNumber(&self) -> i32 {
        unsafe { GEdeviceNumber(self.inner()) }
    }

    pub fn getDevice(number: i32) -> Device {
        unsafe {
            Device {
                inner: GEgetDevice(number),
            }
        }
    }

    pub fn fromDeviceX(&self, value: f64, to: GEUnit) -> f64 {
        unsafe { GEfromDeviceX(value, to, self.inner()) }
    }

    pub fn toDeviceX(&self, value: f64, from: GEUnit) -> f64 {
        unsafe { GEtoDeviceX(value, from, self.inner()) }
    }

    pub fn fromDeviceY(&self, value: f64, to: GEUnit) -> f64 {
        unsafe { GEfromDeviceY(value, to, self.inner()) }
    }

    pub fn toDeviceY(&self, value: f64, from: GEUnit) -> f64 {
        unsafe { GEtoDeviceY(value, from, self.inner()) }
    }

    pub fn fromDeviceWidth(&self, value: f64, to: GEUnit) -> f64 {
        unsafe { GEfromDeviceWidth(value, to, self.inner()) }
    }

    pub fn toDeviceWidth(&self, value: f64, from: GEUnit) -> f64 {
        unsafe { GEtoDeviceWidth(value, from, self.inner()) }
    }

    pub fn fromDeviceHeight(&self, value: f64, to: GEUnit) -> f64 {
        unsafe { GEfromDeviceHeight(value, to, self.inner()) }
    }

    pub fn toDeviceHeight(&self, value: f64, from: GEUnit) -> f64 {
        unsafe { GEtoDeviceHeight(value, from, self.inner()) }
    }

    pub fn setClip(&self, x1: f64, y1: f64, x2: f64, y2: f64) {
        unsafe { GESetClip(x1, y1, x2, y2, self.inner()) }
    }

    pub fn newPage(&self, gc: &mut Context) {
        unsafe { GENewPage(gc.inner(), self.inner()) }
    }

    pub fn line(&self, x1: f64, y1: f64, x2: f64, y2: f64, gc: &Context) {
        unsafe { GELine(x1, y1, x2, y2, gc.inner(), self.inner()) }
    }

    // pub fn GEPolyline(&self,         n: ::std::os::raw::c_int,        x: *mut f64,        y: *mut f64,        gc: pGEcontext);
    // pub fn GEPolygon(&self,         n: ::std::os::raw::c_int,        x: *mut f64,        y: *mut f64,        gc: pGEcontext);
    // pub fn GEXspline(&self,         n: ::std::os::raw::c_int,        x: *mut f64,        y: *mut f64,        s: *mut f64,        open: Rboolean,        repEnds: Rboolean,        draw: Rboolean,        gc: pGEcontext) -> SEXP;
    // pub fn GECircle(&self, x: f64, y: f64, radius: f64, gc: pGEcontext);
    // pub fn GERect(&self, x0: f64, y0: f64, x1: f64, y1: f64, gc: pGEcontext);
    // pub fn GEPath(&self,         x: *mut f64,        y: *mut f64,        npoly: ::std::os::raw::c_int,        nper: *mut ::std::os::raw::c_int,        winding: Rboolean,        gc: pGEcontext);
    // pub fn GERaster(&self,         raster: *mut ::std::os::raw::c_uint,        w: ::std::os::raw::c_int,        h: ::std::os::raw::c_int,        x: f64,        y: f64,        width: f64,        height: f64,        angle: f64,        interpolate: Rboolean,        gc: pGEcontext);
    // pub fn GECap(&self, dd: pGEDevDesc) -> SEXP;
    // pub fn GEText(&self,         x: f64,        y: f64,        str: *const ::std::os::raw::c_char,        enc: cetype_t,        xc: f64,        yc: f64,        rot: f64,        gc: pGEcontext);
    // pub fn GEMode(&self, mode: ::std::os::raw::c_int);
}
