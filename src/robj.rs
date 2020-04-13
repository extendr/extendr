//! R object handling.
//! 
//! See. https://cran.r-project.org/doc/manuals/R-exts.html

use libR_sys::{SEXP, R_PreserveObject, R_ReleaseObject, R_NilValue, Rf_mkCharLen};
use libR_sys::{Rf_ScalarInteger, Rf_ScalarReal};
use libR_sys::{TYPEOF, INTEGER, REAL, PRINTNAME, R_CHAR};
use libR_sys::{Rf_xlength};
use std::os::raw;
use libR_sys::{NILSXP,SYMSXP,LISTSXP,CLOSXP,ENVSXP,PROMSXP,LANGSXP,SPECIALSXP,BUILTINSXP,CHARSXP,LGLSXP,INTSXP,REALSXP,CPLXSXP,STRSXP,DOTSXP,ANYSXP,VECSXP};
use libR_sys::{EXPRSXP, BCODESXP, EXTPTRSXP, WEAKREFSXP, RAWSXP, S4SXP, NEWSXP, FREESXP};

pub enum Robj {
    Owned(SEXP),
    Borrowed(SEXP),
}

impl Robj {
    fn get(&self) -> SEXP {
        match self {
            Robj::Owned(sexp) => *sexp,
            Robj::Borrowed(sexp) => *sexp,
        }
    }

    pub fn sexptype(&self) -> u32 {
        unsafe { TYPEOF(self.get()) as u32 }
    }

    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.get()) as usize }
    }

    pub fn as_i32_slice(&self) -> Option<&[i32]> {
        match self.sexptype() {
            LGLSXP | INTSXP => {
                unsafe {
                    let ptr = INTEGER(self.get()) as *const i32;
                    Some(std::slice::from_raw_parts(ptr, self.len()))
                }
            }
            _ => None
        }
    }

    pub fn as_f64_slice(&self) -> Option<&[f64]> {
        match self.sexptype() {
            REALSXP => {
                unsafe {
                    let ptr = REAL(self.get()) as *const f64;
                    Some(std::slice::from_raw_parts(ptr, self.len()))
                }
            }
            _ => None
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self.sexptype() {
            CHARSXP => {
                unsafe {
                    let ptr = R_CHAR(self.get()) as *const u8;
                    let slice = std::slice::from_raw_parts(ptr, self.len());
                    // Technically we should cope with strings that are
                    // not ASCII or UTF8. If you observe these in the wild...
                    Some(std::str::from_utf8_unchecked(slice))
                }
            }
            _ => None
        }
    }
}

impl PartialEq for Robj {
    fn eq(&self, rhs: &Robj) -> bool {
        if self.sexptype() == rhs.sexptype() && self.len() == rhs.len() {
            let lsexp = self.get();
            let rsexp = rhs.get();
            unsafe {
                match self.sexptype() {
                    NILSXP => true,
                    SYMSXP => PRINTNAME(lsexp) == PRINTNAME(rsexp),
                    LISTSXP => false,
                    CLOSXP => false,
                    ENVSXP => false,
                    PROMSXP => false,
                    LANGSXP => false,
                    SPECIALSXP => false,
                    BUILTINSXP => false,
                    CHARSXP => self.as_str() == rhs.as_str(),
                    LGLSXP | INTSXP => self.as_i32_slice() == rhs.as_i32_slice(),
                    REALSXP => self.as_f64_slice() == rhs.as_f64_slice(),
                    CPLXSXP => false,
                    STRSXP => false,
                    DOTSXP => false,
                    ANYSXP => false,
                    VECSXP => false,
                    EXPRSXP => false,
                    BCODESXP => false,
                    EXTPTRSXP => false,
                    WEAKREFSXP => false,
                    RAWSXP => false,
                    S4SXP => false,
                    NEWSXP => false,
                    FREESXP => false,
                    _ => false
                }
            }
        } else {
            false
        }
    }
}

impl std::fmt::Debug for Robj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let sexp = self.get();
            match self.sexptype() {
                NILSXP => write!(f, "Robj::from(())"),
                SYMSXP => write!(f, "Robj::from_symbol({:?})", Robj::from(PRINTNAME(sexp))),
                // LISTSXP => false,
                // CLOSXP => false,
                // ENVSXP => false,
                // PROMSXP => false,
                // LANGSXP => false,
                // SPECIALSXP => false,
                // BUILTINSXP => false,
                CHARSXP => write!(f, "Robj::from({:?})", self.as_str().unwrap()),
                LGLSXP => write!(f, "Robj::from_logical({:?})", self.as_i32_slice().unwrap()),
                INTSXP => write!(f, "Robj::from({:?})", self.as_i32_slice().unwrap()),
                REALSXP => write!(f, "Robj::from({:?})", self.as_f64_slice().unwrap()),
                // CPLXSXP => false,
                // STRSXP => false,
                // DOTSXP => false,
                // ANYSXP => false,
                // VECSXP => false,
                // EXPRSXP => false,
                // BCODESXP => false,
                // EXTPTRSXP => false,
                // WEAKREFSXP => false,
                // RAWSXP => false,
                // S4SXP => false,
                // NEWSXP => false,
                // FREESXP => false,
                _ => write!(f, "??")
            }
        }
    }
}

// Borrow an already protected SEXP SEXP
impl From<SEXP> for Robj {
    fn from(sexp: SEXP) -> Self {
        Robj::Borrowed(sexp)
    }
}

impl Drop for Robj {
    fn drop(&mut self) {
        unsafe {
            match self {
                Robj::Owned(sexp) => R_ReleaseObject(*sexp),
                Robj::Borrowed(_) => ()
            }
        }
    }
}

/// Convert a null to an Robj.
impl From<()> for Robj {
    fn from(_: ()) -> Self {
        // Note: we do not need to protect this.
        unsafe {
            Robj::Borrowed(R_NilValue)
        }
    }
}

/// Convert a 32 bit integer to an Robj.
impl From<i32> for Robj {
    fn from(val: i32) -> Self {
        unsafe {
            let sexp = Rf_ScalarInteger(val as raw::c_int);
            R_PreserveObject(sexp);
            Robj::Owned(sexp)
        }
    }
}

/// Convert a 64 bit float to an Robj.
impl From<f64> for Robj {
    fn from(val: f64) -> Self {
        unsafe {
            let sexp = Rf_ScalarReal(val as raw::c_double);
            R_PreserveObject(sexp);
            Robj::Owned(sexp)
        }
    }
}

/// Convert a length value to an Robj.
/// Note: This is good only up to 2^53.
impl From<usize> for Robj {
    fn from(val: usize) -> Self {
        unsafe {
            let sexp = if val >= 0x80000000 {
                Rf_ScalarReal(val as raw::c_double)
            } else {
                Rf_ScalarInteger(val as raw::c_int)
            };
            R_PreserveObject(sexp);
            Robj::Owned(sexp)
        }
    }
}

/// Convert a string ref to an Robj char object.
impl From<&str> for Robj {
    fn from(val: &str) -> Self {
        unsafe {
            let sexp = Rf_mkCharLen(val.as_ptr() as *const raw::c_char, val.len() as i32);
            R_PreserveObject(sexp);
            Robj::Owned(sexp)
        }
    }
}

/// Convert a string slice to an Robj string object.
impl From<&[&str]> for Robj {
    fn from(val: &[&str]) -> Self {
        unsafe {
            let sexp = Rf_mkCharLen(val.as_ptr() as *const raw::c_char, val.len() as i32);
            R_PreserveObject(sexp);
            Robj::Owned(sexp)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Robj::from(())), "Robj::from(())");
        assert_eq!(format!("{:?}", Robj::from("x")), "Robj::from(\"x\")");
        assert_eq!(format!("{:?}", Robj::from(1)), "Robj::from([1])");
        assert_eq!(format!("{:?}", Robj::from(1.)), "Robj::from([1.0])");
    }
}
