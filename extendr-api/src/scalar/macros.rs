/// Generates unary operators for scalar types.
macro_rules! gen_unop {
    ($type : tt, $opname : ident, $expr: expr, $docstring: expr) => {
        impl $opname for $type {
            type Output = $type;

            paste::paste! {
                #[doc = $docstring]
                fn [<$opname:lower>](self) -> Self::Output {
                    if let Some(lhs) = self.into() {
                        let f = $expr;
                        if let Some(res) = f(lhs) {
                            // Note that if res is NA, this will also be NA.
                            return $type::from(res);
                        }
                    }
                    $type::na()
                }
            }
        }

        impl $opname for &$type {
            type Output = $type;

            paste::paste! {
                #[doc = $docstring]
                fn [< $opname:lower >](self) -> Self::Output {
                    if let Some(lhs) = (*self).into() {
                        let f = $expr;
                        if let Some(res) = f(lhs) {
                            // Note that if res is NA, this will also be NA.
                            return $type::from(res);
                        }
                    }
                    $type::na()
                }
            }
        }
    };
}

/// Generates binary operators for scalar types.
// TODO: binary operators for pairs `(Rtype, Type)` and `(Type, Rtype)` using references?
macro_rules! gen_binop {
    ($type : tt, $type_prim : tt, $opname : ident, $expr: expr, $docstring: expr) => {
        impl $opname<$type> for $type {
            type Output = $type;

            paste::paste! {
                #[doc = $docstring]
                fn [< $opname:lower >](self, rhs: $type) -> Self::Output {
                    if let Some(lhs) = self.clone().into() {
                        if let Some(rhs) = rhs.into() {
                            let f = $expr;
                            if let Some(res) = f(lhs, rhs) {
                                // Note that if res is NA, this will also be NA.
                                return $type::from(res);
                            }
                        }
                    }
                    $type::na()
                }
            }
        }

        impl $opname<$type> for &$type {
            type Output = $type;

            paste::paste! {
                #[doc = $docstring]
                fn [< $opname:lower >](self, rhs: $type) -> Self::Output {
                    if let Some(lhs) = self.clone().into() {
                        if let Some(rhs) = rhs.into() {
                            let f = $expr;
                            if let Some(res) = f(lhs, rhs) {
                                // Note that if res is NA, this will also be NA.
                                return $type::from(res);
                            }
                        }
                    }
                    $type::na()
                }
            }
        }

        impl $opname<$type_prim> for $type {
            type Output = $type;

            paste::paste! {
                #[doc = $docstring]
                fn [< $opname:lower >](self, rhs: $type_prim) -> Self::Output {
                    if let Some(lhs) = self.clone().into() {
                        let f = $expr;
                        if let Some(res) = f(lhs, rhs) {
                            // Note that if res is NA, this will also be NA.
                            return $type::from(res);
                        }
                    }
                    $type::na()
                }
            }
        }

        impl $opname<$type> for $type_prim {
            type Output = $type;

            paste::paste! {
                #[doc = $docstring]
                fn [< $opname:lower >](self, rhs: $type) -> Self::Output {
                    if let Some(rhs) = rhs.clone().into() {
                        let f = $expr;
                        if let Some(res) = f(self, rhs) {
                            // Note that if res is NA, this will also be NA.
                            return $type::from(res);
                        }
                    }
                    $type::na()
                }
            }
        }
    };
}

/// Generates binary operate-assign operators for scalar types.
macro_rules! gen_binopassign {
    ($type : tt, $type_prim : tt, $opname : ident, $expr: expr, $docstring: expr) => {
        impl $opname<$type> for $type {
            paste::paste! {
                #[doc = $docstring]
                fn [< $opname:snake >](&mut self, other: $type) {
                    // `.clone()` is needed to convert &mut Rint -> Rint -> Option<$type_prim>
                    match (self.clone().into(), other.into()) {
                        (Some(lhs), Some(rhs)) => {
                            let f = $expr;
                            match f(lhs, rhs) {
                                Some(res) => *self = $type::from(res),
                                None => *self = $type::na(),
                            }
                        },
                        _ => *self = $type::na(),
                    }
                }
            }
        }

        impl $opname<$type> for &mut $type {
            paste::paste! {
                #[doc = $docstring]
                fn [< $opname:snake >](&mut self, other: $type) {
                    // `.clone()` is needed to convert &mut &mut Rint -> Rint -> Option<$type_prim>
                    match (self.clone().into(), other.into()) {
                        (Some(lhs), Some(rhs)) => {
                            let f = $expr;
                            match f(lhs, rhs) {
                                Some(res) => **self = $type::from(res),
                                None => **self = $type::na(),
                            }
                        },
                        _ => **self = $type::na(),
                    }
                }
            }
        }

        impl $opname<$type_prim> for $type {
            paste::paste! {
                #[doc = $docstring]
                fn [< $opname:snake >](&mut self, other: $type_prim) {
                    // `.clone()` is needed to convert &mut Rint -> Rint -> Option<$type_prim>
                    match self.clone().into() {
                        Some(lhs) => {
                            let f = $expr;
                            match f(lhs, other) {
                                Some(res) => *self = $type::from(res),
                                None => *self = $type::na(),
                            }
                        }
                        None => *self = $type::na(),
                    }
                }
            }
        }

        impl $opname<$type_prim> for &mut $type {
            paste::paste! {
                #[doc = $docstring]
                fn [< $opname:snake >](&mut self, other: $type_prim) {
                    // `.clone()` is needed to convert &mut &mut Rint -> Rint -> Option<$type_prim>
                    match self.clone().into() {
                        Some(lhs) => {
                            let f = $expr;
                            match f(lhs, other) {
                                Some(res) => **self = $type::from(res),
                                None => **self = $type::na(),
                            }
                        }
                        None => **self = $type::na(),
                    }
                }
            }
        }

        impl $opname<$type> for Option<$type_prim> {
            paste::paste! {
                #[doc = $docstring]
                fn [< $opname:snake >](&mut self, other: $type) {
                    match (*self, other.into()) {
                        (Some(lhs), Some(rhs)) => {
                            let f = $expr;
                            *self = f(lhs, rhs);
                        },
                        _ => *self = None,
                    }
                }
            }
        }
    };
}

/// Generates conversions from primitive to scalar type.
macro_rules! gen_from_primitive {
    ($type : tt, $type_prim : tt) => {
        impl From<$type_prim> for $type {
            fn from(v: $type_prim) -> Self {
                Self(v)
            }
        }

        impl From<Option<$type_prim>> for $type {
            fn from(v: Option<$type_prim>) -> Self {
                if let Some(v) = v {
                    v.into()
                } else {
                    $type::na()
                }
            }
        }
    };
}

/// Generates conversions from scalar type.
macro_rules! gen_from_scalar {
    ($type : tt, $type_prim : tt) => {
        impl From<$type> for Option<$type_prim> {
            fn from(v: $type) -> Self {
                if v.is_na() {
                    None
                } else {
                    Some(v.0)
                }
            }
        }

        impl From<$type> for Robj {
            fn from(value: $type) -> Self {
                Robj::from(value.0)
            }
        }
    };
}

/// Generates required methods:
/// 1. static `na()`
/// 2. instance `inner()`
macro_rules! gen_impl {
    ($type : ident, $type_prim : ty) => {
        /// Get underlying value.
        pub fn inner(&self) -> $type_prim {
            self.0
        }
    };
}

/// Generates scalar trait implementations:
/// 1. `Clone`
/// 2. `Copy`
/// 3. `IsNA`
/// 4. `Debug`
/// 5. `PartialEq`
macro_rules! gen_trait_impl {
    ($type : ident, $type_prim : ty, $na_check : expr, $na_val : expr) => {
        impl Clone for $type {
            fn clone(&self) -> Self {
                Self(self.0)
            }
        }

        impl Copy for $type {}

        paste::paste! {
            #[doc = "```"]
            #[doc = "use extendr_api::prelude::*;"]
            #[doc = "test! {"]
            #[doc = "    assert!((<" $type ">::na()).is_na());"]
            #[doc = "}"]
            #[doc = "```"]
            impl CanBeNA for $type {
                /// Return true is the is a NA value.
                fn is_na(&self) -> bool {
                    $na_check(self)
                }
                /// Construct a NA.
                fn na() -> Self {
                    $type($na_val)
                }
            }
        }
        impl std::fmt::Debug for $type {
            /// Debug format.
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let z: Option<$type_prim> = (*self).into();
                if let Some(val) = z {
                    write!(f, "{}", val)
                } else {
                    write!(f, "na")
                }
            }
        }

        paste::paste! {
            #[doc = "```"]
            #[doc = "use extendr_api::prelude::*;"]
            #[doc = "test! {"]
            #[doc = "    assert!(<" $type ">::default().eq(&<" $type ">::default()));"]
            #[doc = "    assert!(!<" $type ">::na().eq(&<" $type ">::na()));"]
            #[doc = "}"]
            #[doc = "```"]
            impl PartialEq<$type> for $type {
                fn eq(&self, other: &$type) -> bool {
                    !(self.is_na() || other.is_na()) && self.0 == other.0
                }
            }
        }

        paste::paste! {
            #[doc = "```"]
            #[doc = "use extendr_api::prelude::*;"]
            #[doc = "test! {"]
            #[doc = "    assert!(<" $type ">::default().eq(&<" $type_prim ">::default()));"]
            #[doc = "}"]
            #[doc = "```"]
            impl PartialEq<$type_prim> for $type {
                /// NA always fails.
                fn eq(&self, other: &$type_prim) -> bool {
                    !self.is_na() && self.0 == *other
                }
            }
        }

        paste::paste! {
            #[doc = "```"]
            #[doc = "use extendr_api::prelude::*;"]
            #[doc = "test! {"]
            #[doc = "    assert_eq!(<" $type ">::default().0, <" $type_prim ">::default());"]
            #[doc = "}"]
            #[doc = "```"]
            impl std::default::Default for $type {
                fn default() -> Self {
                    $type(<$type_prim>::default())
                }
            }
        }
    };
}

/// Generates `std::iter::Sum` for scalar types.
macro_rules! gen_sum_iter {
    ($type : tt, $zero : expr) => {
        impl std::iter::Sum for $type {
            paste::paste! {
                #[doc = "Yields NA on overflow if NAs present."]
                #[doc = "```"]
                #[doc = "use extendr_api::prelude::*;"]
                #[doc = "use std::iter::Sum;"]
                #[doc = "test! {"]
                #[doc = "    let x = (0..100).map(|x| " $type "::default());"]
                #[doc = "    assert_eq!(<" $type " as Sum>::sum(x), <" $type ">::default());"]
                #[doc = "}"]
                #[doc = "```"]
                fn sum<I: Iterator<Item = $type>>(iter: I) -> $type {
                    iter.fold($type::from($zero), |a, b| a + b)
                }
            }
        }
    };
}

pub(in crate::scalar) use gen_binop;
pub(in crate::scalar) use gen_binopassign;
pub(in crate::scalar) use gen_from_primitive;
pub(in crate::scalar) use gen_from_scalar;
pub(in crate::scalar) use gen_impl;
pub(in crate::scalar) use gen_sum_iter;
pub(in crate::scalar) use gen_trait_impl;
pub(in crate::scalar) use gen_unop;
