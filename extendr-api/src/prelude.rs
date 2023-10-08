//! # Common exports for extendr-api.
//!
//! This allows us to be more selective about exports and avoid users
//! using deprecated features.

pub use super::{
    print_r_error, print_r_output, CanBeNA, FromRobj, Rtype, FALSE, NA_INTEGER, NA_LOGICAL,
    NA_REAL, NA_STRING, NULL, TRUE,
};

pub use super::na::*;

pub use super::error::{Error, Result};

pub use super::functions::{
    base_env, base_namespace, blank_scalar_string, blank_string, current_env, empty_env,
    eval_string, eval_string_with_params, find_namespace, find_namespaced_function, global_env,
    global_function, global_var, local_var, na_string, namespace_registry, new_env, nil_value,
    parse, srcref,
};

pub use super::wrapper::symbol::{
    base_symbol, brace_symbol, bracket_2_symbol, bracket_symbol, class_symbol, device_symbol,
    dim_symbol, dimnames_symbol, dollar_symbol, dot_defined, dot_method, dot_package_name,
    dot_target, dots_symbol, double_colon_symbol, lastvalue_symbol, levels_symbol, missing_arg,
    mode_symbol, na_rm_symbol, name_symbol, names_symbol, namespace_env_symbol, package_symbol,
    previous_symbol, quote_symbol, row_names_symbol, seeds_symbol, sort_list_symbol, source_symbol,
    spec_symbol, triple_colon_symbol, tsp_symbol, unbound_value,
};

// Exported macros have crate scope.
pub use crate::{append, append_lang, append_with_name, args, lang, make_lang};

// Exported macros have crate scope.
pub use crate::{
    data_frame, factor, global, list, r, reprint, reprintln, rprint, rprintln, sym, test, var,
};

pub use super::wrapper::{
    AltComplexImpl, AltIntegerImpl, AltLogicalImpl, AltRawImpl, AltRealImpl, AltStringImpl, Altrep,
    AltrepImpl, RArray, RColumn, RMatrix, RMatrix3D,
};

#[cfg(use_r_altlist)]
pub use super::wrapper::AltListImpl;

pub use super::wrapper::s4::S4;

pub use super::wrapper::{Conversions, MatrixConversions};

pub use super::robj::{
    AsStrIter, Attributes, Eval, GetSexp, IntoRobj, Length, Operators, Rinternals, Robj,
    RobjItertools, Slices, Types,
};

pub use super::thread_safety::{catch_r_error, handle_panic, single_threaded, throw_r_error};

pub use super::wrapper::{
    Complexes, Dataframe, Doubles, EnvIter, Environment, Expressions, ExternalPtr, FromList,
    Function, Integers, IntoDataFrameRow, Language, List, ListIter, Logicals, Nullable, Pairlist,
    Primitive, Promise, Raw, Rstr, Strings, Symbol,
};

pub use extendr_macros::{call, extendr, extendr_module, pairlist, IntoDataFrameRow, Rraw, R};

pub use super::iter::StrIter;

pub use std::convert::{TryFrom, TryInto};

pub use super::scalar::*;

pub use super::Nullable::*;

pub use super::optional::*;

#[cfg(feature = "ndarray")]
pub use ::ndarray::*;

#[cfg(feature = "either")]
pub use ::either::*;
