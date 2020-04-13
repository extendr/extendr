//! R object handling.
//! 
//! See. https://cran.r-project.org/doc/manuals/R-exts.html

use libR_sys::{SEXP, R_PreserveObject, R_ReleaseObject, R_NilValue, Rf_mkCharLen};
use libR_sys::{Rf_ScalarInteger, Rf_ScalarReal, Rf_ScalarLogical};
use libR_sys::{TYPEOF, INTEGER, REAL, PRINTNAME, R_CHAR, LOGICAL, STRING_PTR};
use libR_sys::{Rf_xlength, Rf_install, Rf_allocVector, R_xlen_t};
use std::os::raw;
use std::ffi::{CString};
use libR_sys::{NILSXP,SYMSXP,LISTSXP,CLOSXP,ENVSXP,PROMSXP,LANGSXP,SPECIALSXP,BUILTINSXP,CHARSXP,LGLSXP,INTSXP,REALSXP,CPLXSXP,STRSXP,DOTSXP,ANYSXP,VECSXP};
use libR_sys::{EXPRSXP, BCODESXP, EXTPTRSXP, WEAKREFSXP, RAWSXP, S4SXP, NEWSXP, FREESXP};

pub enum Robj<'a> {
    // This object owns the SEXP and must free it.
    Owned(SEXP),

    // This object references a SEXP such as an input parameter.
    // The borrow checker should ensure it does not outlive the
    // underlying SEXP.
    Borrowed(&'a SEXP),

    // This object references a SEXP owned by libR.
    Sys(SEXP),
}

// Wrapper for creating symbols.
pub struct Symbol<'a>(&'a str);

// Wrapper for creating logical vectors.
pub struct Logical<'a>(&'a [i32]);

// Wrapper for creating character objects.
pub struct Character<'a>(&'a str);

impl<'a> Robj<'a> {
    unsafe fn get(&self) -> SEXP {
        match self {
            Robj::Owned(sexp) => *sexp,
            Robj::Borrowed(sexp) => **sexp,
            Robj::Sys(sexp) => *sexp,
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
        unsafe {
            match self.sexptype() {
                CHARSXP => {
                    let ptr = R_CHAR(self.get()) as *const u8;
                    let slice = std::slice::from_raw_parts(ptr, self.len());
                    // Technically we should cope with strings that are
                    // not ASCII or UTF8. If you observe these in the wild...
                    Some(std::str::from_utf8_unchecked(slice))
                }
                SYMSXP => {
                    let ptr = R_CHAR(PRINTNAME(self.get())) as *const u8;
                    let slice = std::slice::from_raw_parts(ptr, self.len());
                    Some(std::str::from_utf8_unchecked(slice))
                }
                _ => None
            }
        }
    }

}

impl<'a> PartialEq for Robj<'a> {
    fn eq(&self, rhs: &Robj) -> bool {
        if self.sexptype() == rhs.sexptype() && self.len() == rhs.len() {
            unsafe {
                let lsexp = self.get();
                let rsexp = rhs.get();
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

impl<'a> std::fmt::Debug for Robj<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.sexptype() {
            NILSXP => write!(f, "()"),
            SYMSXP => write!(f, "Symbol({:?})", self.as_str().unwrap()),
            // LISTSXP => false,
            // CLOSXP => false,
            // ENVSXP => false,
            // PROMSXP => false,
            // LANGSXP => false,
            // SPECIALSXP => false,
            // BUILTINSXP => false,
            CHARSXP => write!(f, "Character({:?})", self.as_str().unwrap()),
            LGLSXP => {
                let slice = self.as_i32_slice().unwrap();
                if slice.len() == 1 {
                    write!(f, "{}", if slice[0] == 0 {false} else {true})
                } else {
                    write!(f, "Logical(&{:?})", slice)
                }
            }
            INTSXP => {
                let slice = self.as_i32_slice().unwrap();
                if slice.len() == 1 {
                    write!(f, "{:?}", slice[0])
                } else {
                    write!(f, "&{:?}", self.as_i32_slice().unwrap())
                }
            },
            REALSXP => {
                let slice = self.as_f64_slice().unwrap();
                if slice.len() == 1 {
                    write!(f, "{:?}", slice[0])
                } else {
                    write!(f, "&{:?}", slice)
                }
            },
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

// Borrow an already protected SEXP SEXP
impl<'a> From<&'a SEXP> for Robj<'a> {
    fn from(sexp: &'a SEXP) -> Self {
        Robj::Borrowed(sexp)
    }
}

impl<'a> Drop for Robj<'a> {
    fn drop(&mut self) {
        unsafe {
            match self {
                Robj::Owned(sexp) => R_ReleaseObject(*sexp),
                Robj::Borrowed(_) => (),
                Robj::Sys(_) => (),
            }
        }
    }
}

/// Convert a null to an Robj.
impl<'a> From<()> for Robj<'a> {
    fn from(_: ()) -> Self {
        // Note: we do not need to protect this.
        unsafe {
            Robj::Sys(R_NilValue)
        }
    }
}

/// Convert a 32 bit integer to an Robj.
impl<'a> From<bool> for Robj<'a> {
    fn from(val: bool) -> Self {
        unsafe {
            let sexp = Rf_ScalarLogical(val as raw::c_int);
            R_PreserveObject(sexp);
            Robj::Owned(sexp)
        }
    }
}

/// Convert a 32 bit integer to an Robj.
impl<'a> From<i32> for Robj<'a> {
    fn from(val: i32) -> Self {
        unsafe {
            let sexp = Rf_ScalarInteger(val as raw::c_int);
            R_PreserveObject(sexp);
            Robj::Owned(sexp)
        }
    }
}

/// Convert a 64 bit float to an Robj.
impl<'a> From<f64> for Robj<'a> {
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
impl<'a> From<usize> for Robj<'a> {
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

/// Convert a wrapped string ref to an Robj char object.
impl<'a> From<Character<'a>> for Robj<'a> {
    fn from(val: Character) -> Self {
        unsafe {
            let sexp = Rf_mkCharLen(val.0.as_ptr() as *const raw::c_char, val.0.len() as i32);
            R_PreserveObject(sexp);
            Robj::Owned(sexp)
        }
    }
}

/// Convert a string ref to an Robj string array object.
impl<'a> From<&str> for Robj<'a> {
    fn from(val: &str) -> Self {
        unsafe {
            let sexp = Rf_allocVector(STRSXP, 1);
            R_PreserveObject(sexp);
            let ssexp = Rf_mkCharLen(val.as_ptr() as *const raw::c_char, val.len() as i32);
            let ptr = STRING_PTR(sexp);
            let slice = std::slice::from_raw_parts_mut(ptr, 1);
            slice[0] = ssexp;
            Robj::Owned(sexp)
        }
    }
}

impl<'a> From<&[i32]> for Robj<'a> {
    fn from(vals: &[i32]) -> Self {
        unsafe {
            let len = vals.len();
            let sexp = Rf_allocVector(INTSXP, len as R_xlen_t);
            R_PreserveObject(sexp);
            let ptr = INTEGER(sexp);
            let slice = std::slice::from_raw_parts_mut(ptr, len);
            for (i, &v) in vals.iter().enumerate() {
                slice[i] = v;
            }
            Robj::Owned(sexp)
        }
    }
}

impl<'a> From<Logical<'a>> for Robj<'a> {
    fn from(vals: Logical<'a>) -> Self {
        unsafe {
            let len = vals.0.len();
            let sexp = Rf_allocVector(LGLSXP, len as R_xlen_t);
            R_PreserveObject(sexp);
            let ptr = LOGICAL(sexp);
            let slice = std::slice::from_raw_parts_mut(ptr, len);
            for (i, &v) in vals.0.iter().enumerate() {
                slice[i] = v;
            }
            Robj::Owned(sexp)
        }
    }
}

impl<'a> From<&[bool]> for Robj<'a> {
    fn from(vals: &[bool]) -> Self {
        unsafe {
            let len = vals.len();
            let sexp = Rf_allocVector(LGLSXP, len as R_xlen_t);
            R_PreserveObject(sexp);
            let ptr = LOGICAL(sexp);
            let slice = std::slice::from_raw_parts_mut(ptr, len);
            for (i, &v) in vals.iter().enumerate() {
                slice[i] = v as i32;
            }
            Robj::Owned(sexp)
        }
    }
}

impl<'a> From<&[f64]> for Robj<'a> {
    fn from(vals: &[f64]) -> Self {
        unsafe {
            let len = vals.len();
            let sexp = Rf_allocVector(REALSXP, len as R_xlen_t);
            R_PreserveObject(sexp);
            let ptr = REAL(sexp);
            let slice = std::slice::from_raw_parts_mut(ptr, len);
            for (i, &v) in vals.iter().enumerate() {
                slice[i] = v;
            }
            Robj::Owned(sexp)
        }
    }
}

impl<'a> From<Symbol<'a>> for Robj<'a> {
    fn from(name: Symbol) -> Self {
        unsafe {
            if let Ok(name) = CString::new(name.0) {
                Robj::Sys(Rf_install(name.as_ptr()))
            } else {
                Robj::from(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Robj::from(())), "()");
        assert_eq!(format!("{:?}", Robj::from(Symbol("x"))), "Symbol(\"x\")");
        assert_eq!(format!("{:?}", Robj::from(Character("x"))), "Character(\"x\")");
        assert_eq!(format!("{:?}", Robj::from(true)), "true");
        assert_eq!(format!("{:?}", Robj::from(false)), "false");
        assert_eq!(format!("{:?}", Robj::from(Logical(&[1, 2]))), "Logical(&[1, 2])");
        assert_eq!(format!("{:?}", Robj::from(1)), "1");
        assert_eq!(format!("{:?}", Robj::from(1.)), "1.0");
    }
}
