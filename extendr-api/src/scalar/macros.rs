macro_rules! gen_unop {
    ($type : tt, $opname1 : ident, $opname2: ident, $expr: expr, $docstring: expr) => {
        impl $opname1 for $type {
            type Output = $type;

            #[doc = $docstring]
            fn $opname2(self) -> Self::Output {
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

        impl $opname1 for &$type {
            type Output = $type;

            #[doc = $docstring]
            fn $opname2(self) -> Self::Output {
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
    };
}
macro_rules! gen_binop {
    ($type : tt, $type_prim : tt, $opname1 : ident, $opname2: ident, $expr: expr, $docstring: expr) => {
        impl $opname1<$type> for $type {
            type Output = $type;

            #[doc = $docstring]
            fn $opname2(self, rhs: $type) -> Self::Output {
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

        impl $opname1<$type> for &$type {
            type Output = $type;

            #[doc = $docstring]
            fn $opname2(self, rhs: $type) -> Self::Output {
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

        impl $opname1<$type_prim> for $type {
            type Output = $type;

            #[doc = $docstring]
            fn $opname2(self, rhs: $type_prim) -> Self::Output {
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
    };
}


macro_rules! gen_from {
    // TODO: Should conversions fail if `NA` is provided?
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

    }
}

macro_rules! gen_scalar_impl {
    ($type : tt, $type_prim : tt, $na_val : expr) => {

        impl $type {
            /// Construct a NA.
            pub fn na() -> Self {
                $type($na_val)
            }

            /// Get underlying value.
            pub fn inner(&self) -> $type_prim {
                self.0
            }
        }

        impl Clone for $type {
            fn clone(&self) -> Self {
                Self(self.0)
            }
        }

        impl Copy for $type {}

        impl IsNA for $type {
            /// Return true is the is a NA value.
            fn is_na(&self) -> bool {
                self.0 == $na_val
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

        impl PartialEq<$type_prim> for $type {
            /// NA always fails.
            fn eq(&self, other: &$type_prim) -> bool {
                !self.is_na() && self.0 == *other
            }
        }
    }
}

macro_rules! gen_sum_iter {
    ($type : tt, $zero : expr) => {
        impl std::iter::Sum for $type {
            /// Yields NA on overflow if NAs present.
            fn sum<I: Iterator<Item = $type>>(iter: I) -> $type {
                iter.fold($type::from($zero), |a, b| a + b)
            }
        }

    }
}

pub(in crate::scalar) use gen_unop;
pub(in crate::scalar) use gen_binop;
pub(in crate::scalar) use gen_from;
pub(in crate::scalar) use gen_scalar_impl;
pub(in crate::scalar) use gen_sum_iter;