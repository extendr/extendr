//! S4 class support.

use super::*;

#[derive(PartialEq, Clone)]
pub struct S4 {
    pub(crate) robj: Robj,
}

impl S4 {
    /// Create a S4 class.
    ///
    /// Example:
    /// ```
    /// use extendr_api::prelude::*;
    ///
    /// test! {
    ///     let class = S4::set_class("fred", pairlist!(x="numeric"), r!(()))?;
    /// }
    /// ```
    pub fn set_class(name: &str, representation: Pairlist, contains: Robj) -> Result<S4> {
        use crate as extendr_api;
        let res = R!(r#"setClass({{name}}, {{representation}}, {{contains}})"#)?;
        res.try_into()
    }

    /// Create a S4 object.
    ///
    /// Example:
    /// ```
    /// use extendr_api::prelude::*;
    ///
    /// test! {
    ///     S4::set_class("fred", pairlist!(x="numeric"), r!(()))?;
    ///     let mut robj : S4 = R!(r#"new("fred")"#)?.try_into()?;
    /// }
    /// ```
    pub fn new(name: &str) -> Result<S4> {
        use crate as extendr_api;
        let res = R!(r#"new({{name}})"#)?;
        res.try_into()
    }

    /// Get a named slot from a S4 object.
    ///
    /// Example:
    /// ```
    /// use extendr_api::prelude::*;
    ///
    /// test! {
    ///     S4::set_class("fred", pairlist!(xyz="numeric"), r!(()))?;
    ///     let robj : S4 = R!(r#"new("fred")"#)?.try_into()?;
    ///     assert_eq!(robj.get_slot("xyz").unwrap().len(), 0);
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
                Some(Robj::from_sexp(R_do_slot(self.get(), name.get())))
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
    ///     S4::set_class("fred", pairlist!(xyz="numeric"), r!(()))?;
    ///     let mut robj : S4 = R!(r#"new("fred")"#)?.try_into()?;
    ///     let xyz = sym!(xyz);
    ///     assert_eq!(robj.get_slot(xyz.clone()).unwrap().len(), 0);
    ///     robj.set_slot(xyz.clone(), r!([0.0, 1.0]));
    ///     assert_eq!(robj.get_slot(xyz), Some(r!([0.0, 1.0])));
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
    ///     S4::set_class("fred", pairlist!(xyz="numeric"), r!(()))?;
    ///     let robj : S4 = R!(r#"new("fred")"#)?.try_into()?;
    ///     assert_eq!(robj.has_slot("xyz"), true);
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

// TODO: Think about these functions in the future.
//
// Currently, S4 support is not a top priority, but we hope that what we have
// covered the basics for now.
//
// extern "C" {
//     pub fn R_S4_extends(klass: SEXP, useTable: SEXP) -> SEXP;
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

impl std::fmt::Debug for S4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("S4").finish()
    }
}
