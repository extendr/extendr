//! S4 class support.

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct S4 {
    pub(crate) robj: Robj,
}

impl S4 {
    /// Make a new S4 type.
    ///
    /// Example:
    /// ```
    /// use extendr_api::prelude::*;
    ///
    /// test! {
    ///     let robj = S4::new();
    ///     assert_eq!(robj.rtype(), RType::S4);
    /// }
    /// ```
    pub fn new() -> Self {
        let robj = single_threaded(|| unsafe { new_owned(Rf_allocS4Object()) });
        Self { robj }
    }

    /// Get a named slot from a S4 object.
    ///
    /// Example:
    /// ```
    /// use extendr_api::prelude::*;
    ///
    /// test! {
    ///     let mut robj = S4::new();
    ///     let xyz = sym!(xyz);
    ///     assert_eq!(robj.get_slot(xyz.clone()), None);
    ///     robj.set_slot(xyz.clone(), 1234);
    ///     assert_eq!(robj.get_slot(xyz), Some(r!(1234)));
    /// }
    /// ```
    pub fn get_slot<'a, N>(&self, name: N) -> Option<Robj>
    where
        Self: 'a,
        Robj: From<N> + 'a,
    {
        let name = Robj::from(name);
        unsafe {
            if R_has_slot(self.get(), name.get()) != 0 {
                Some(new_owned(R_do_slot(self.get(), name.get())))
            } else {
                None
            }
        }
    }

    /// Set a named slot in a S4 object.
    ///
    /// Example:
    /// ```
    /// use extendr_api::prelude::*;
    ///
    /// test! {
    ///     let mut robj = S4::new();
    ///     let xyz = sym!(xyz);
    ///     assert_eq!(robj.get_slot(xyz.clone()), None);
    ///     robj.set_slot(xyz.clone(), 1234);
    ///     assert_eq!(robj.get_slot(xyz), Some(r!(1234)));
    /// }
    /// ```
    pub fn set_slot<N, V>(&mut self, name: N, value: V) -> Result<S4>
    where
        N: Into<Robj>,
        V: Into<Robj>,
    {
        let name = name.into();
        let value = value.into();
        unsafe {
            single_threaded(|| {
                catch_r_error(|| R_do_slot_assign(self.get(), name.get(), value.get()))
                    .map(|_| self.clone())
            })
        }
    }

    /// Check if a named slot exists.
    ///
    /// Example:
    /// ```
    /// use extendr_api::prelude::*;
    ///
    /// test! {
    ///     let mut robj = S4::new();
    ///     let xyz = sym!(xyz);
    ///     assert_eq!(robj.has_slot(xyz.clone()), false);
    ///     robj.set_slot(xyz.clone(), 1234);
    ///     assert_eq!(robj.has_slot(xyz.clone()), true);
    /// }
    /// ```
    pub fn has_slot<'a, N>(&self, name: N) -> bool
    where
        Self: 'a,
        Robj: From<N> + 'a,
    {
        let name = Robj::from(name);
        unsafe { R_has_slot(self.get(), name.get()) != 0 }
    }
}

impl Default for wrapper::s4::S4 {
    fn default() -> Self {
        Self::new()
    }
}

// Think about these in the future.
//
// extern "C" {
//     pub fn R_S4_extends(klass: SEXP, useTable: SEXP) -> SEXP;
// }
// extern "C" {
//     pub fn R_do_MAKE_CLASS(what: *const ::std::os::raw::c_char) -> SEXP;
// }
// extern "C" {
//     pub fn R_getClassDef(what: *const ::std::os::raw::c_char) -> SEXP;
// }
// extern "C" {
//     pub fn R_getClassDef_R(what: SEXP) -> SEXP;
// }
// extern "C" {
//     pub fn R_has_methods_attached() -> Rboolean;
// }
// extern "C" {
//     pub fn R_isVirtualClass(class_def: SEXP, env: SEXP) -> Rboolean;
// }
// extern "C" {
//     pub fn R_extends(class1: SEXP, class2: SEXP, env: SEXP) -> Rboolean;
// }
// extern "C" {
//     pub fn R_do_new_object(class_def: SEXP) -> SEXP;
// }
// extern "C" {
//     pub fn R_check_class_and_super(
//         x: SEXP,
//         valid: *mut *const ::std::os::raw::c_char,
//         rho: SEXP,
//     ) -> ::std::os::raw::c_int;
// }
// extern "C" {
//     pub fn R_check_class_etc(
//         x: SEXP,
//         valid: *mut *const ::std::os::raw::c_char,
//     ) -> ::std::os::raw::c_int;
// }
