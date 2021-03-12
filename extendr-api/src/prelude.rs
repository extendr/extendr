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
    base_env, base_namespace, base_symbol, blank_scalar_string, blank_string, brace_symbol,
    bracket_2_symbol, bracket_symbol, class_symbol, current_env, device_symbol, dim_symbol,
    dimnames_symbol, dollar_symbol, dot_defined, dot_method, dot_package_name, dot_target,
    dots_symbol, double_colon_symbol, empty_env, eval_string, find_namespace, global_env,
    global_function, global_var, lastvalue_symbol, levels_symbol, local_var, missing_arg,
    mode_symbol, na_rm_symbol, na_str, na_string, name_symbol, names_symbol, namespace_env_symbol,
    namespace_registry, new_env, new_env_with_capacity, nil_value, package_symbol, parse,
    previous_symbol, quote_symbol, row_names_symbol, seeds_symbol, sort_list_symbol, source_symbol,
    spec_symbol, srcref, triple_colon_symbol, tsp_symbol, unbound_value,
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
    Character, EnvIter, Environment, Expression, Function, Language, List, ListIter, Nullable,
    Pairlist, Primitive, Promise, Raw, Symbol,
};

#[cfg(feature = "ndarray")]
pub use super::robj_ndarray::*;

#[cfg(feature = "ndarray")]
pub use ndarray::*;

pub use extendr_macros::{extendr, extendr_module};

pub use super::iter::{Int, Logical, PairlistTagIter, PairlistValueIter, Real, StrIter};
