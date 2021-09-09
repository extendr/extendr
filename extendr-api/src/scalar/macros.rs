macro_rules! gen_unnop {
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

pub(in crate::scalar) use gen_unnop;
pub(in crate::scalar) use gen_binop;
pub(in crate::scalar) use gen_from;