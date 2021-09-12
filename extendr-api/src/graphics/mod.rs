use crate::*;
use libR_sys::*;

pub mod color;

pub struct Context {
    context: R_GE_gcontext,
    xscale: (f64, f64),
    yscale: (f64, f64),
    offset: (f64, f64),
    scalar: f64,
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

    // pub(crate) fn asref(&self) -> &GEDevDesc {
    //     unsafe { &*self.inner }
    // }

    // pub(crate) fn dev(&self) -> &DevDesc {
    //     unsafe { &*self.asref().dev }
    // }
}

#[derive(PartialEq, Debug, Clone)]
pub enum LineEnd {
    RoundCap,
    ButtCap,
    SquareCap,
}

#[derive(PartialEq, Debug, Clone)]
pub enum LineJoin {
    RoundJoin,
    MitreJoin,
    BevelJoin,
}

#[derive(PartialEq, Debug, Clone)]
pub enum LineType {
    Blank,
    Solid,
    Dashed,
    Dotted,
    Dotdash,
    Longdash,
    Twodash,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Unit {
    Device,
    Normalized,
    Inches,
    CM,
}

#[derive(PartialEq, Debug, Clone)]
pub enum FontFace {
    PlainFont,
    BoldFont,
    ItalicFont,
    BoldItalicFont,
    SymbolFont,
}

fn unit_to_ge(unit: Unit) -> GEUnit {
    match unit {
        Unit::Device => GEUnit_GE_DEVICE,
        Unit::Normalized => GEUnit_GE_NDC,
        Unit::Inches => GEUnit_GE_INCHES,
        Unit::CM => GEUnit_GE_CM,
    }
}

impl Context {
    pub fn from_device(dev: &Device, unit: Unit) -> Self {
        #[allow(unused_unsafe)]
        unsafe {
            let offset = dev.to_device_coords((0., 0.), unit.clone());
            let mut xscale = dev.to_device_coords((1., 0.), unit.clone());
            let mut yscale = dev.to_device_coords((0., 1.), unit);
            xscale.0 -= offset.0;
            xscale.1 -= offset.1;
            yscale.0 -= offset.0;
            yscale.1 -= offset.1;

            // sqrt(abs(det(m)))
            let scalar = (xscale.0 * yscale.1 - xscale.1 * yscale.0).abs().sqrt();

            let context = R_GE_gcontext {
                col: color::rgb(0xff, 0xff, 0xff).to_i32(),
                fill: color::rgb(0xc0, 0xc0, 0xc0).to_i32(),
                gamma: 1.0,
                lwd: 1.0,
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

            Self {
                context,
                xscale,
                yscale,
                offset,
                scalar,
            }
        }
    }

    pub fn color(&mut self, col: color::Color) -> &mut Self {
        self.context.col = col.to_i32();
        self
    }

    pub fn fill(&mut self, fill: color::Color) -> &mut Self {
        self.context.fill = fill.to_i32();
        self
    }

    pub fn gamma(&mut self, gamma: f64) -> &mut Self {
        self.context.gamma = gamma;
        self
    }

    pub fn line_width(&mut self, lwd: f64) -> &mut Self {
        self.context.lwd = (lwd * self.scalar).max(1.0);
        self
    }

    pub fn line_type(&mut self, lty: LineType) -> &mut Self {
        use LineType::*;
        self.context.lty = match lty {
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
        self.context.lend = match lend {
            LineEnd::RoundCap => 1,
            LineEnd::ButtCap => 2,
            LineEnd::SquareCap => 3,
        };
        self
    }

    pub fn line_join(&mut self, ljoin: LineJoin) -> &mut Self {
        self.context.ljoin = match ljoin {
            LineJoin::RoundJoin => 1,
            LineJoin::MitreJoin => 2,
            LineJoin::BevelJoin => 3,
        };
        self
    }

    pub fn point_size(&mut self, ps: f64) -> &mut Self {
        self.context.ps = ps;
        self
    }

    pub fn line_mitre(&mut self, lmitre: f64) -> &mut Self {
        self.context.lmitre = lmitre * self.scalar;
        self
    }

    pub fn line_height(&mut self, lineheight: f64) -> &mut Self {
        self.context.lineheight = lineheight;
        self
    }

    // pub fn char_extra_size(&mut self, cex: f64) -> &mut Self {
    //     self.context.cex = cex;
    //     self
    // }

    pub fn font_face(&mut self, fontface: FontFace) -> &mut Self {
        use FontFace::*;
        self.context.fontface = match fontface {
            PlainFont => 1,
            BoldFont => 2,
            ItalicFont => 3,
            BoldItalicFont => 4,
            SymbolFont => 5,
        };
        self
    }

    pub fn font_family(&mut self, fontfamily: &str) -> &mut Self {
        let maxlen = self.context.fontfamily.len() - 1;

        for c in self.context.fontfamily.iter_mut() {
            *c = 0;
        }

        for (i, b) in fontfamily.bytes().enumerate().take(maxlen) {
            self.context.fontfamily[i] = b as std::os::raw::c_char;
        }
        self
    }

    pub fn transform(
        &mut self,
        xscale: (f64, f64),
        yscale: (f64, f64),
        offset: (f64, f64),
    ) -> &mut Self {
        self.xscale = xscale;
        self.yscale = yscale;
        self.offset = offset;
        self
    }

    pub(crate) fn context(&self) -> pGEcontext {
        unsafe { std::mem::transmute(&self.context) }
    }

    // Affine transform.
    pub(crate) fn t(&self, xy: (f64, f64)) -> (f64, f64) {
        (
            self.offset.0 + xy.0 * self.xscale.0 + xy.1 * self.yscale.0,
            self.offset.1 + xy.0 * self.xscale.1 + xy.1 * self.yscale.1,
        )
    }

    // Scalar transform (eg. radius etc).
    pub(crate) fn ts(&self, value: f64) -> f64 {
        value * self.scalar
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

    pub fn mode_off(&self) {
        unsafe {
            GEMode(0, self.inner());
        }
    }

    pub fn mode_on(&self) {
        unsafe {
            GEMode(1, self.inner());
        }
    }

    pub fn device_number(&self) -> i32 {
        unsafe { GEdeviceNumber(self.inner()) }
    }

    pub fn get_device(number: i32) -> Device {
        unsafe {
            Device {
                inner: GEgetDevice(number),
            }
        }
    }

    pub fn from_device_coords(&self, value: (f64, f64), from: Unit) -> (f64, f64) {
        let from = unit_to_ge(from);
        unsafe {
            (
                GEfromDeviceX(value.0, from, self.inner()),
                GEfromDeviceY(value.1, from, self.inner()),
            )
        }
    }

    pub fn to_device_coords(&self, value: (f64, f64), to: Unit) -> (f64, f64) {
        if to == Unit::Device {
            value
        } else {
            let to = unit_to_ge(to);
            unsafe {
                (
                    GEtoDeviceX(value.0, to, self.inner()),
                    GEtoDeviceY(value.1, to, self.inner()),
                )
            }
        }
    }

    pub fn from_device_wh(&self, value: (f64, f64), from: Unit) -> (f64, f64) {
        let from = unit_to_ge(from);
        unsafe {
            (
                GEfromDeviceWidth(value.0, from, self.inner()),
                GEfromDeviceHeight(value.1, from, self.inner()),
            )
        }
    }

    pub fn to_device_wh(&self, value: (f64, f64), to: Unit) -> (f64, f64) {
        let to = unit_to_ge(to);
        unsafe {
            (
                GEtoDeviceWidth(value.0, to, self.inner()),
                GEtoDeviceHeight(value.1, to, self.inner()),
            )
        }
    }

    pub fn new_page(&self, gc: &Context) {
        unsafe { GENewPage(gc.context(), self.inner()) }
    }

    pub fn line(&self, from: (f64, f64), to: (f64, f64), gc: &Context) {
        let from = gc.t(from);
        let to = gc.t(to);
        unsafe { GELine(from.0, from.1, to.0, to.1, gc.context(), self.inner()) }
    }

    pub fn polyline<T: IntoIterator<Item = (f64, f64)>>(&self, coords: T, gc: &Context) {
        let (mut x, mut y): (Vec<_>, Vec<_>) = coords.into_iter().map(|xy| gc.t(xy)).unzip();
        let xptr = x.as_mut_slice().as_mut_ptr();
        let yptr = y.as_mut_slice().as_mut_ptr();
        unsafe {
            GEPolyline(
                x.len() as std::os::raw::c_int,
                xptr,
                yptr,
                gc.context(),
                self.inner(),
            )
        }
    }

    pub fn polygon<T: IntoIterator<Item = (f64, f64)>>(&self, coords: T, gc: &Context) {
        let (mut x, mut y): (Vec<_>, Vec<_>) = coords.into_iter().map(|xy| gc.t(xy)).unzip();
        let xptr = x.as_mut_slice().as_mut_ptr();
        let yptr = y.as_mut_slice().as_mut_ptr();
        unsafe {
            GEPolygon(
                x.len() as std::os::raw::c_int,
                xptr,
                yptr,
                gc.context(),
                self.inner(),
            )
        }
    }

    // /// Return a list of (x, y) points generated from a spline.
    // /// The iterator returns ((x, y), s) where s is -1 to 1.
    // pub fn xspline<T: Iterator<Item = ((f64, f64), f64)> + Clone>(
    //     &self,
    //     coords: T,
    //     open: bool,
    //     rep_ends: bool,
    //     draw: bool,
    //     gc: &Context,
    // ) -> Robj {
    //     let (mut x, mut y): (Vec<_>, Vec<_>) = coords
    //         .clone()
    //         .map(|(xy, _s)| gc.t(xy))
    //         .unzip();
    //     let mut s: Vec<_> = coords.map(|(_xy, s)| s).collect();
    //     let xptr = x.as_mut_slice().as_mut_ptr();
    //     let yptr = y.as_mut_slice().as_mut_ptr();
    //     let sptr = s.as_mut_slice().as_mut_ptr();
    //     unsafe {
    //         new_owned(GEXspline(
    //             x.len() as std::os::raw::c_int,
    //             xptr,
    //             yptr,
    //             sptr,
    //             if open { 1 } else { 0 },
    //             if rep_ends { 1 } else { 0 },
    //             if draw { 1 } else { 0 },
    //             gc.context(),
    //             self.inner(),
    //         ))
    //     }
    // }

    pub fn circle(&self, center: (f64, f64), radius: f64, gc: &Context) {
        let center = gc.t(center);
        let radius = gc.ts(radius);
        unsafe { GECircle(center.0, center.1, radius, gc.context(), self.inner()) }
    }

    pub fn rectangle(&self, from: (f64, f64), to: (f64, f64), gc: &Context) {
        let from = gc.t(from);
        let to = gc.t(to);
        unsafe { GELine(from.0, from.1, to.0, to.1, gc.context(), self.inner()) }
    }

    pub fn path<T: IntoIterator<Item = impl IntoIterator<Item = (f64, f64)>>>(
        &self,
        coords: T,
        winding: bool,
        gc: &Context,
    ) {
        let mut x = Vec::new();
        let mut y = Vec::new();
        let mut nper: Vec<std::os::raw::c_int> = Vec::new();
        let coords = coords.into_iter();
        for segment in coords {
            let mut n = 0;
            for xy in segment {
                let xy = gc.t(xy);
                x.push(xy.0);
                y.push(xy.1);
                n += 1;
            }
            nper.push(n);
        }

        let xptr = x.as_mut_slice().as_mut_ptr();
        let yptr = y.as_mut_slice().as_mut_ptr();
        let nperptr = nper.as_mut_slice().as_mut_ptr();
        unsafe {
            GEPath(
                xptr,
                yptr,
                nper.len() as std::os::raw::c_int,
                nperptr,
                if winding { 1 } else { 0 },
                gc.context(),
                self.inner(),
            )
        }
    }

    // pub fn GERaster(&self,         raster: *mut ::std::os::raw::c_uint,        w: ::std::os::raw::c_int,        h: ::std::os::raw::c_int,        x: f64,        y: f64,        width: f64,        height: f64,        angle: f64,        interpolate: Rboolean,        gc: pGEcontext);
    // pub fn GECap(&self, dd: pGEDevDesc) -> SEXP;
    // pub fn GEText(&self,         x: f64,        y: f64,        str: *const ::std::os::raw::c_char,        enc: cetype_t,        xc: f64,        yc: f64,        rot: f64,        gc: pGEcontext);
}
