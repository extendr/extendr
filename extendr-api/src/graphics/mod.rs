use crate::*;
use libR_sys::*;

pub struct Context {
    inner: R_GE_gcontext
}

#[derive(Clone, Debug, PartialEq)]
pub struct DevDesc {
    inner: pGEDevDesc
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    inner: Robj
}

impl DevDesc {
    pub (crate) fn inner(&self) -> pGEDevDesc {
        self.inner
    }
}

impl Context {
    pub fn new() -> Self {
        let inner = R_GE_gcontext {
            col: 0,
            fill: 0,
            gamma: 1.0,
            lwd: 1.0,
            lty: 0,
            lend: R_GE_lineend_GE_ROUND_CAP,
            ljoin: R_GE_linejoin_GE_BEVEL_JOIN,
            lmitre: 1.0,
            cex: 1.0,
            ps: 1.0,
            lineheight: 1.0,
            fontface: 0,
            fontfamily: [0; 201],
            // patternFill: R_NilValue,
        };
        Self { inner }
    }

    // pub fn from_GP(dd: &DevDesc) -> Self {
    //     // let mut x = Self::new();
    //     // unsafe { gcontextFromGP(x.inner_mut(), dd.inner()) };
    //     // let gpptr = Rf_gpptr(dd.inner_mut());
    //     let inner = R_GE_gcontext {
    //         col: gpptr(dd),
    //         fill: 0,
    //         gamma: 1.0,
    //         lwd: 1.0,
    //         lty: 0,
    //         lend: R_GE_lineend_GE_ROUND_CAP,
    //         ljoin: R_GE_linejoin_GE_BEVEL_JOIN,
    //         lmitre: 1.0,
    //         cex: 1.0,
    //         ps: 1.0,
    //         lineheight: 1.0,
    //         fontface: 0,
    //         fontfamily: [0; 201],
    //         // patternFill: R_NilValue,
    //     };
    //     Self { inner }
    // }

    pub (crate) fn inner(&self) -> pGEcontext {
        unsafe { std::mem::transmute(&self.inner) }
    }

    // pub (crate) fn inner_mut(&mut self) -> pGEcontext {
    //     &mut self.inner as pGEcontext
    // }
}

#[allow(non_snake_case)]
impl DevDesc {
    pub fn current() -> DevDesc {
        unsafe { DevDesc{ inner: GEcurrentDevice() } }
    }

    pub fn deviceNumber(&self) -> i32 {
        unsafe { GEdeviceNumber(self.inner()) }
    }

    pub fn getDevice(number: i32) -> DevDesc {
        unsafe { DevDesc { inner: GEgetDevice(number) } }
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

