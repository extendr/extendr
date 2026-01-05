//! Hash implementation for R objects.
//!
//! This module provides a Hash implementation for Robj that allows R objects
//! to be used as keys in HashMaps and HashSets. The implementation follows
//! these principles:
//!
//! - For most types, hash by content (values, structure)
//! - For reference types (environments, symbols), hash by pointer to match R's identity semantics
//! - For external pointers, hash by the pointer address
//! - Handle cycles in recursive structures
//! - For Doubles we convert the the f64 to bits then hash the bits inspired by the  [`ordered-float`](https://docs.rs/ordered-float/latest/src/ordered_float/lib.rs.html#2203-2212) crate
//!
//! Note: For environments (ENVSXP), we hash by pointer address to match R's identical() behavior.
use crate::{
    scalar::{Rbool, Rfloat, Rint},
    wrapper::{
        rstr::{self, Rstr},
        Complexes, Doubles, Integers, List, Logicals, Raw, Strings,
    },
    AsStrIter, AsTypedSlice, Attributes, Conversions, Environment, Expressions, Function, GetSexp,
    Language, Length, Pairlist, Robj, Symbol, Types, S4, SEXP,
};
use extendr_ffi::{
    R_NilValue, R_WeakRefKey, R_WeakRefValue, Rcomplex, Rf_isObject, CAR, CDR, PRINTNAME, SEXPTYPE,
    TAG,
};
use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

const HASH_CYCLE_MARKER: u8 = 0xFF;

impl Hash for Robj {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut stack = HashSet::new();
        hash_robj(self, state, &mut stack);
    }
}

fn hash_robj<H: Hasher>(robj: &Robj, state: &mut H, stack: &mut HashSet<SEXP>) {
    let sexp = unsafe { robj.get() };

    if stack.contains(&sexp) {
        HASH_CYCLE_MARKER.hash(state);
        return;
    }

    stack.insert(sexp);
    // seed similar to R's vhash_one: OBJECT flag + 2*TYPEOF + 100*LENGTH
    let obj_flag = unsafe { Rf_isObject(sexp) };
    obj_flag.hash(state);
    robj.sexptype().hash(state);
    robj.len().hash(state);
    if robj.len() != 0 {
        hash_robj_body(robj, state, stack);
    }
    // at the end of the (potential) cycle, we no longer have to track this
    // item in the stack
    stack.remove(&sexp);
}

fn hash_robj_body<H: Hasher>(robj: &Robj, state: &mut H, stack: &mut HashSet<SEXP>) {
    use SEXPTYPE::*;
    let sexp = unsafe { robj.get() };
    match robj.sexptype() {
        // null is not hashed we have no wrapper type for it.
        NILSXP => {}
        // LGLSXP: logical vector
        LGLSXP => {
            if let Some(values) = robj.as_logical_slice() {
                values.hash(state);
            }
        }
        // INTSXP: integer vector
        INTSXP => {
            if let Some(values) = robj.as_integer_slice() {
                values.hash(state);
            }
        }
        // REALSXP: double/numeric vector
        REALSXP => {
            if let Some(values) = robj.as_real_slice() {
                hash_real_slice(values, state);
            }
        }
        // CPLXSXP: complex vector
        CPLXSXP => {
            if let Some(values) = <Robj as AsTypedSlice<'_, Rcomplex>>::as_typed_slice(robj) {
                hash_complex_slice(values, state);
            }
        }
        // RAWSXP: raw byte vector
        RAWSXP => {
            if let Some(values) = robj.as_raw_slice() {
                values.hash(state);
            }
        }
        // CHARSXP/STRSXP: hash string bytes
        CHARSXP | STRSXP => hash_string_vector(robj, state),
        // SYMSXP: hash symbol printname
        SYMSXP => unsafe {
            let printname = PRINTNAME(sexp);
            if let Some(text) = rstr::charsxp_to_str(printname) {
                text.hash(state);
            } else {
                sexp.hash(state);
            }
        },
        // ENVSXP: hash by pointer (identity), matching R's behavior
        ENVSXP => hash_environment(robj, state),
        // VECSXP: hash elements in order, ignore names/attributes
        VECSXP => hash_vector(robj, state, stack),
        // EXPRSXP: expression vector, order matters, names ignored
        EXPRSXP => hash_expressions(robj, state, stack),
        // LISTSXP/LANGSXP/DOTSXP: walk CONS cells, include tags
        LISTSXP | LANGSXP | DOTSXP => hash_pairlist_with_tags(sexp, state, stack),
        // CLOSXP: hash body and environment pointer
        CLOSXP => hash_closure(robj, state, stack),
        // EXTPTRSXP: external pointer wrapper
        EXTPTRSXP => hash_external_ptr(robj, state, stack),
        // Pointer hash only for remaining code-like types
        PROMSXP | ANYSXP | SPECIALSXP | BUILTINSXP | FUNSXP | BCODESXP | NEWSXP | FREESXP => {
            sexp.hash(state)
        }
        // weak references, where key is usually ENVSXP or EXTPTRSXP
        WEAKREFSXP => hash_weakref(robj, state, stack),
        #[cfg(not(use_objsxp))]
        // S4SXP: formal S4 objects
        S4SXP => hash_s4_by_slots(robj, state, stack),
        #[cfg(use_objsxp)]
        // OBJSXP: formal S4 objects (4.4+)
        OBJSXP => hash_s4_by_slots(robj, state, stack),
    }
}

fn hash_closure<H: Hasher>(robj: &Robj, state: &mut H, stack: &mut HashSet<SEXP>) {
    unsafe {
        let sexp = robj.get();
        let formals = Robj::from_sexp(extendr_ffi::get_closure_formals(sexp));
        hash_robj(&formals, state, stack);
        let body = Robj::from_sexp(extendr_ffi::backports::get_closure_body(sexp));
        hash_robj(&body, state, stack);
        let env = extendr_ffi::get_closure_env(sexp);
        env.hash(state);
    }
}

/// This function requires that `robj` is of type [`SEXPTYPE::STRSXP`] or [`SEXPTYPE::CHARSXP`]
fn hash_string_vector<H: Hasher>(robj: &Robj, state: &mut H) {
    if let Some(iter) = robj.as_str_iter() {
        for value in iter {
            value.hash(state);
        }
    }
}

fn hash_environment<H: Hasher>(robj: &Robj, state: &mut H) {
    // Hash environment by pointer (identity) to match R's behavior and PartialEq.
    // R's identical() compares environments by pointer, not by contents.
    let sexp = unsafe { robj.get() };
    sexp.hash(state);
}

// `SEXPTYPE::VECSXP`
fn hash_vector<H: Hasher>(robj: &Robj, state: &mut H, stack: &mut HashSet<SEXP>) {
    if let Some(list) = robj.as_list() {
        for value in list.values() {
            hash_robj(&value, state, stack);
        }
    }
}

/// This function requires that `robj` is of type
/// [`SEXPTYPE::LISTSXP`],
/// [`SEXPTYPE::LANGSXP`], or
/// [`SEXPTYPE::DOTSXP`].
fn hash_pairlist_with_tags<H: Hasher>(mut sexp: SEXP, state: &mut H, stack: &mut HashSet<SEXP>) {
    unsafe {
        while sexp != R_NilValue {
            let car = Robj::from_sexp(CAR(sexp));
            hash_robj(&car, state, stack);
            let tag = TAG(sexp);
            if tag != R_NilValue {
                let tag_sexp = Robj::from_sexp(PRINTNAME(tag));
                hash_robj(&tag_sexp, state, stack);
            }
            sexp = CDR(sexp);
        }
    }
}

fn hash_expressions<H: Hasher>(robj: &Robj, state: &mut H, stack: &mut HashSet<SEXP>) {
    if let Some(exprs) = robj.as_expressions() {
        for value in exprs.values() {
            hash_robj(&value, state, stack);
        }
    }
}

fn hash_external_ptr<H: Hasher>(robj: &Robj, state: &mut H, stack: &mut HashSet<SEXP>) {
    unsafe {
        let sexp = robj.get();
        let addr = extendr_ffi::R_ExternalPtrAddr(sexp);
        addr.hash(state);
        let tag = extendr_ffi::R_ExternalPtrTag(sexp);
        if tag != R_NilValue {
            hash_robj(&Robj::from_sexp(tag), state, stack);
        }
        let prot = extendr_ffi::R_ExternalPtrProtected(sexp);
        if prot != R_NilValue {
            hash_robj(&Robj::from_sexp(prot), state, stack);
        }
    }
}

fn hash_weakref<H: Hasher>(robj: &Robj, state: &mut H, stack: &mut HashSet<SEXP>) {
    unsafe {
        let sexp = robj.get();
        let key = R_WeakRefKey(sexp);
        if key != R_NilValue {
            let key_robj = Robj::from_sexp(key);
            // this _should_ only be ENVSXP or EXTPTRSXP...
            hash_robj(&key_robj, state, stack);
        }
        let value = R_WeakRefValue(sexp);
        if value != R_NilValue {
            let value_robj = Robj::from_sexp(value);
            hash_robj(&value_robj, state, stack);
        }
    }
}

/// This requires that `robj` is [`SEXPTYPE::OBJSXP`] or [`SEXPTYPE::S4SXP`].
fn hash_s4_by_slots<H: Hasher>(robj: &Robj, state: &mut H, stack: &mut HashSet<SEXP>) {
    if let Some(classes) = robj.class() {
        for class in classes {
            class.hash(state);
        }
    }

    if let Some(list) = robj.as_list() {
        if let Some(names) = list.names() {
            for name in names {
                name.hash(state);
            }
        }
        for value in list.values() {
            hash_robj(&value, state, stack);
        }
    }
}

fn hash_real_slice<H: Hasher>(values: &[f64], state: &mut H) {
    for value in values {
        hash_f64_value(*value, state);
    }
}

fn hash_complex_slice<H: Hasher>(values: &[Rcomplex], state: &mut H) {
    for value in values {
        hash_f64_value(value.r, state);
        hash_f64_value(value.i, state);
    }
}

fn hash_f64_value<H: Hasher>(value: f64, state: &mut H) {
    let canonical_bits = {
        use crate::na::CanBeNA;

        if value.is_nan() {
            if value.is_na() {
                f64::na().to_bits()
            } else {
                f64::NAN.to_bits()
            }
        } else if value == 0.0 {
            // catches both +0.0 and -0.0
            0u64
        } else {
            value.to_bits()
        }
    };
    canonical_bits.hash(state);
}

// Delegate hashing to the Robj hash impl
impl Hash for Pairlist {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.robj.hash(state);
    }
}

impl Hash for Function {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.robj.hash(state);
    }
}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.robj.hash(state);
    }
}

impl Hash for Language {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.robj.hash(state);
    }
}

impl Hash for List {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.robj.hash(state);
    }
}

impl Hash for Doubles {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.robj.hash(state);
    }
}

impl Hash for Integers {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.robj.hash(state);
    }
}

impl Hash for Strings {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.robj.hash(state);
    }
}

impl Hash for Logicals {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.robj.hash(state);
    }
}

impl Hash for Complexes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.robj.hash(state);
    }
}

impl Hash for Raw {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.robj.hash(state);
    }
}

impl Hash for Rstr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Robj::from(self.clone()).hash(state);
    }
}

impl Hash for Rint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Hash for Rfloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_f64_value(self.0, state);
    }
}

impl Hash for Rbool {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Hash for Environment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.robj.hash(state);
    }
}

impl Hash for Expressions {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.robj.hash(state);
    }
}

impl Hash for S4 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.robj.hash(state);
    }
}

// Eq is required for hasing
impl Eq for List {}
impl Eq for Doubles {}
impl Eq for Integers {}
impl Eq for Strings {}
impl Eq for Logicals {}
impl Eq for Complexes {}
impl Eq for Raw {}
impl Eq for Rstr {}
impl Eq for Rint {}
impl Eq for Rfloat {}
impl Eq for Rbool {}
