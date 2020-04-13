//! R object handling.
//! 
//! See. https://cran.r-project.org/doc/manuals/R-exts.html

use libR_sys::{SEXP, R_PreserveObject, R_ReleaseObject, R_NilValue, Rf_mkCharLen};
use libR_sys::{Rf_ScalarInteger, Rf_ScalarReal, Rf_ScalarLogical};
use libR_sys::{TYPEOF, INTEGER, REAL, PRINTNAME, R_CHAR, LOGICAL, STRING_PTR, RAW, VECTOR_ELT, STRING_ELT};
use libR_sys::{Rf_xlength, Rf_install, Rf_allocVector, R_xlen_t};
use std::os::raw;
use std::ffi::{CString};
use libR_sys::{NILSXP,SYMSXP,LISTSXP,CLOSXP,ENVSXP,PROMSXP,LANGSXP,SPECIALSXP,BUILTINSXP,CHARSXP,LGLSXP,INTSXP,REALSXP,CPLXSXP,STRSXP,DOTSXP,ANYSXP,VECSXP};
use libR_sys::{EXPRSXP, BCODESXP, EXTPTRSXP, WEAKREFSXP, RAWSXP, S4SXP, NEWSXP, FREESXP};

pub enum Robj {
    /// This object owns the SEXP and must free it.
    Owned(SEXP),

    /// This object references a SEXP such as an input parameter.
    Borrowed(SEXP),

    /// This object references a SEXP owned by libR.
    Sys(SEXP),
}

pub const TRUE: bool = true;
pub const FALSE: bool = false;
pub const NULL: () = ();

/// Wrapper for creating symbols.
pub struct Symbol<'a>(&'a str);

/// Wrapper for creating logical vectors.
pub struct Logical<'a>(&'a [i32]);

/// Wrapper for creating character objects.
pub struct Character<'a>(&'a str);

impl Robj {
    /// Get a copy of the underlying SEXP.
    /// Note: this is unsafe.
    unsafe fn get(&self) -> SEXP {
        match self {
            Robj::Owned(sexp) => *sexp,
            Robj::Borrowed(sexp) => *sexp,
            Robj::Sys(sexp) => *sexp,
        }
    }

    /// Get the XXXSXP type of the object.
    pub fn sexptype(&self) -> u32 {
        unsafe { TYPEOF(self.get()) as u32 }
    }

    /// Get the extended length of the object.
    pub fn len(&self) -> usize {
        unsafe { Rf_xlength(self.get()) as usize }
    }

    /// Get a read-only reference to the content of an integer or logical vector.
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

    /// Get a read-only reference to the content of a double vector.
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

    /// Get a read-only reference to the content of an integer or logical vector.
    pub fn as_u8_slice(&self) -> Option<&[u8]> {
        match self.sexptype() {
            RAWSXP => {
                unsafe {
                    let ptr = RAW(self.get()) as *const u8;
                    Some(std::slice::from_raw_parts(ptr, self.len()))
                }
            }
            _ => None
        }
    }

    pub fn vector_iter(&self) -> Option<VecIter> {
        match self.sexptype() {
            VECSXP => {
                unsafe {
                    Some(VecIter{ vector: self.get(), i: 0, len: self.len()})
                }
            }
            _ => None
        }
    }

    pub fn str_iter(&self) -> Option<StrIter> {
        match self.sexptype() {
            STRSXP => {
                unsafe {
                    Some(StrIter{ vector: self.get(), i: 0, len: self.len()})
                }
            }
            _ => None
        }
    }

    /// Get a read-only reference to a char, symbol or string type.
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

/// Return true if two objects are equal.
impl PartialEq<Robj> for Robj {
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
                    RAWSXP => self.as_u8_slice() == rhs.as_u8_slice(),
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

/// Compare equality with strings.
impl PartialEq<str> for Robj {
    fn eq(&self, rhs: &str) -> bool {
        self.as_str() == Some(rhs)
    }
}

/// Implement {:?} formatting.
impl std::fmt::Debug for Robj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.sexptype() {
            NILSXP => write!(f, "NULL"),
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
                    write!(f, "{}", if slice[0] == 0 {"FALSE"} else {"TRUE"})
                } else {
                    write!(f, "Logical(&{:?})", slice)
                }
            }
            INTSXP => {
                let slice = self.as_i32_slice().unwrap();
                if slice.len() == 1 {
                    write!(f, "{:?}", slice[0])
                } else {
                    write!(f, "{:?}", self.as_i32_slice().unwrap())
                }
            },
            REALSXP => {
                let slice = self.as_f64_slice().unwrap();
                if slice.len() == 1 {
                    write!(f, "{:?}", slice[0])
                } else {
                    write!(f, "{:?}", slice)
                }
            },
            // CPLXSXP => false,
            VECSXP | EXPRSXP | WEAKREFSXP => {
                write!(f, "[")?;
                let mut sep = "";
                for obj in self.vector_iter().unwrap() {
                    write!(f, "{}{:?}", sep, obj)?;
                    sep = ", ";
                }
                write!(f, "]")
            }
            STRSXP => {
                write!(f, "[")?;
                let mut sep = "";
                for obj in self.str_iter().unwrap() {
                    write!(f, "{}{:?}", sep, obj)?;
                    sep = ", ";
                }
                write!(f, "]")
            }
            // DOTSXP => false,
            // ANYSXP => false,
            // VECSXP => false,
            // EXPRSXP => false,
            // BCODESXP => false,
            // EXTPTRSXP => false,
            // WEAKREFSXP => false,
            RAWSXP => {
                let slice = self.as_u8_slice().unwrap();
                if slice.len() == 1 {
                    write!(f, "{}", slice[0])
                } else {
                    write!(f, "{:?}", slice)
                }
            },
            // S4SXP => false,
            // NEWSXP => false,
            // FREESXP => false,
            _ => write!(f, "??")
        }
    }
}

/// Borrow an already protected SEXP
/// Note that the SEXP must outlive the generated object.
impl From<SEXP> for Robj {
    fn from(sexp: SEXP) -> Self {
        Robj::Borrowed(sexp)
    }
}

/// Release any owned objects.
impl Drop for Robj {
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
impl From<()> for Robj {
    fn from(_: ()) -> Self {
        // Note: we do not need to protect this.
        unsafe {
            Robj::Sys(R_NilValue)
        }
    }
}

/// Convert a 32 bit integer to an Robj.
impl From<bool> for Robj {
    fn from(val: bool) -> Self {
        unsafe {
            let sexp = Rf_ScalarLogical(val as raw::c_int);
            R_PreserveObject(sexp);
            Robj::Owned(sexp)
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

/// Convert a wrapped string ref to an Robj char object.
impl<'a> From<Character<'a>> for Robj {
    fn from(val: Character) -> Self {
        unsafe {
            let sexp = Rf_mkCharLen(val.0.as_ptr() as *const raw::c_char, val.0.len() as i32);
            R_PreserveObject(sexp);
            Robj::Owned(sexp)
        }
    }
}

/// Convert a string ref to an Robj string array object.
impl<'a> From<&'a str> for Robj {
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

/// Convert an integer slice to an integer object.
impl<'a> From<&'a [i32]> for Robj {
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

/// Convert an integer slice to a logical object.
impl<'a> From<Logical<'a>> for Robj {
    fn from(vals: Logical) -> Self {
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

/// Convert a bool slice to a logical object.
impl From<&[bool]> for Robj {
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

/// Convert a double slice to a numeric object.
impl From<&[f64]> for Robj {
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

/// Convert a byte slice to a raw object.
impl From<&[u8]> for Robj {
    fn from(vals: &[u8]) -> Self {
        unsafe {
            let len = vals.len();
            let sexp = Rf_allocVector(RAWSXP, len as R_xlen_t);
            R_PreserveObject(sexp);
            let ptr = RAW(sexp);
            let slice = std::slice::from_raw_parts_mut(ptr, len);
            for (i, &v) in vals.iter().enumerate() {
                slice[i] = v;
            }
            Robj::Owned(sexp)
        }
    }
}

/// Convert a string to a symbol.
impl<'a> From<Symbol<'a>> for Robj {
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

// Iterator over the objects in a vector or string.
pub struct VecIter {
    vector: SEXP,
    i: usize,
    len: usize,
}

impl Iterator for VecIter {
    type Item = Robj;
 
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;
        if i >= self.len {
            return None;
        } else {
            Some(Robj::from(unsafe {VECTOR_ELT(self.vector, i as isize)}))
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.i += n;
        self.next()
    }
}

pub struct StrIter {
    vector: SEXP,
    i: usize,
    len: usize,
}

impl Iterator for StrIter {
    type Item = &'static str;
 
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;
        if i >= self.len {
            return None;
        } else {
            unsafe {
                let sexp = STRING_ELT(self.vector, i as isize);
                let ptr = R_CHAR(sexp) as *const u8;
                let slice = std::slice::from_raw_parts(ptr, Rf_xlength(sexp) as usize);
                Some(std::str::from_utf8_unchecked(slice))
            }
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.i += n;
        self.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        // Special values
        assert_eq!(format!("{:?}", Robj::from(NULL)), "NULL");
        assert_eq!(format!("{:?}", Robj::from(TRUE)), "TRUE");
        assert_eq!(format!("{:?}", Robj::from(FALSE)), "FALSE");

        // Scalars
        assert_eq!(format!("{:?}", Robj::from(1)), "1");
        assert_eq!(format!("{:?}", Robj::from(1.)), "1.0");
        assert_eq!(format!("{:?}", Robj::from("hello")), "[\"hello\"]");

        // Vectors
        assert_eq!(format!("{:?}", Robj::from(&[1, 2, 3][..])), "[1, 2, 3]");
        assert_eq!(format!("{:?}", Robj::from(&[1., 2., 3.][..])), "[1.0, 2.0, 3.0]");
        assert_eq!(format!("{:?}", Robj::from(&[1_u8, 2_u8, 3_u8][..])), "[1, 2, 3]");

        // Wrappers
        assert_eq!(format!("{:?}", Robj::from(Symbol("x"))), "Symbol(\"x\")");
        assert_eq!(format!("{:?}", Robj::from(Character("x"))), "Character(\"x\")");
        assert_eq!(format!("{:?}", Robj::from(Logical(&[1, 0]))), "Logical(&[1, 0])");
    }
}
