use super::*;

pub trait IntoDataframe<T>: IntoIterator<Item = T> {
    fn into_dataframe(self) -> Result<Dataframe<T>>;
}

#[derive(PartialEq, Clone)]
pub struct Dataframe<T> {
    pub(crate) robj: Robj,
    marker: std::marker::PhantomData<T>,
}

impl<T> std::convert::TryFrom<&Robj> for Dataframe<T> {
    type Error = Error;
    fn try_from(robj: &Robj) -> Result<Self> {
        // TODO: check type using derived trait.
        if robj.inherits("data.frame") {
            Ok(Dataframe {
                robj: robj.clone(),
                marker: std::marker::PhantomData,
            })
        } else {
            Err(Error::ExpectedDataframe(robj.clone()))
        }
    }
}

impl<T> std::convert::TryFrom<Robj> for Dataframe<T> {
    type Error = Error;
    fn try_from(robj: Robj) -> Result<Self> {
        (&robj).try_into()
    }
}

macro_rules! impl_tuple {
    ( $($A : ident),* : $($a : ident),* : $($z : ident),* ) => {
        impl<$($A),*, I> IntoDataframe<($($A),*)> for I
        where
            $(robj::Robj: From<Vec<$A>>),*,
            I: IntoIterator<Item=($($A),*)>
        {
            fn into_dataframe(self) -> Result<Dataframe<($($A),*)>> {
                $(let mut $a = Vec::new();)*
                for ($($z),*) in self {
                    $($a.push($z);)*
                }
                let caller = eval_string("data.frame")?;
                let res = caller.call(Pairlist::from_pairs(&[
                    $((stringify!($a), Robj::from($a))),*
                ]))?;
                res.try_into()
            }
        }
    }
}

impl_tuple!(A, B: a, b: a1, b1);
impl_tuple!(A, B, C: a, b, c: a1, b1, c1);
impl_tuple!(A, B, C, D: a, b, c, d: a1, b1, c1, d1);
impl_tuple!(A, B, C, D, E: a, b, c, d, e: a1, b1, c1, d1, e1);
impl_tuple!(A, B, C, D, E, F: a, b, c, d, e, f: a1, b1, c1, d1, e1, f1);
impl_tuple!(
    A,
    B,
    C,
    D,
    E,
    F,
    G: a,
    b,
    c,
    d,
    e,
    f,
    g: a1,
    b1,
    c1,
    d1,
    e1,
    f1,
    g1
);
impl_tuple!(
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H: a,
    b,
    c,
    d,
    e,
    f,
    g,
    h: a1,
    b1,
    c1,
    d1,
    e1,
    f1,
    g1,
    h1
);

impl<T> Dataframe<T> {
    /// Use `#[derive(IntoDataframe)]` to use this.
    pub fn try_from_values<I: IntoDataframe<T>>(iter: I) -> Result<Self> {
        iter.into_dataframe()
    }
}

impl<T> GetSexp for Dataframe<T> {
    unsafe fn get(&self) -> SEXP {
        self.robj.get()
    }

    fn as_robj(&self) -> &Robj {
        &self.robj
    }

    fn as_robj_mut(&mut self) -> &mut Robj {
        &mut self.robj
    }
}

// These traits all derive from GetSexp

/// len() and is_empty()
impl<T> Length for Dataframe<T> {}

/// rtype() and rany()
impl<T> Types for Dataframe<T> {}

/// as_*()
impl<T> Conversions for Dataframe<T> {}

/// find_var() etc.
impl<T> Rinternals for Dataframe<T> {}

/// as_typed_slice_raw() etc.
impl<T> Slices for Dataframe<T> {}

/// dollar() etc.
impl<T> Operators for Dataframe<T> {}
