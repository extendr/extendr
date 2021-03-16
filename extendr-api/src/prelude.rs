//! # Common exports for extendr-api.
//!
//! This allows us to be more selective about exports and avoid users
//! using deprecated features.

pub use super::{
    new_owned, print_r_error, print_r_output, FromRobj, IsNA, RType, FALSE, NA_INTEGER, NA_LOGICAL,
    NA_REAL, NA_STRING, NULL, TRUE,
};

pub use super::error::{Error, Result};

pub use super::functions::{
    base_env, base_namespace, blank_scalar_string, blank_string,
    current_env, empty_env, eval_string, find_namespace, global_env,
    global_function, global_var, local_var, na_str, na_string, namespace_registry, nil_value,
    srcref, parse,
};

pub use super::wrapper::symbol::{
    base_symbol, brace_symbol,
    bracket_2_symbol, bracket_symbol, class_symbol, device_symbol, dim_symbol,
    dimnames_symbol, dollar_symbol, dot_defined, dot_method, dot_package_name, dot_target,
    dots_symbol, double_colon_symbol, lastvalue_symbol, levels_symbol, missing_arg,
    mode_symbol, na_rm_symbol, name_symbol, names_symbol, namespace_env_symbol,
    package_symbol, previous_symbol, quote_symbol,
    row_names_symbol, seeds_symbol, sort_list_symbol, source_symbol, spec_symbol,
    triple_colon_symbol, tsp_symbol, unbound_value,
};

pub use crate::{append, append_lang, append_with_name, args, call, lang, make_lang};
pub use crate::{
    data_frame, factor, global, list, pairlist, r, reprint, reprintln, rprint, rprintln, sym, test,
    var, R,
};

pub use super::logical::Bool;

pub use super::wrapper::{RArray, RColumn, RMatrix, RMatrix3D};

pub use super::robj::{IntoRobj, Robj, RobjItertools};

pub use super::thread_safety::{
    catch_r_error, handle_panic, single_threaded, this_thread_id, throw_r_error,
};

pub use super::wrapper::{
    Character, EnvIter, Environment, Expression, FromList, Function, Language, List, ListIter,
    Nullable, Pairlist, Primitive, Promise, Raw, Symbol,
};

#[cfg(feature = "ndarray")]
pub use super::robj_ndarray::*;

#[cfg(feature = "ndarray")]
pub use ndarray::*;

pub use extendr_macros::{extendr, extendr_module};

pub use super::iter::{Int, Logical, Real, StrIter};

pub use std::convert::{TryFrom, TryInto};

pub use std::ops::Index;
