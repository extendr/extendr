//! This provides an abstraction for R's `data.frame`-constructor in Rust.
//! For a given `struct` say `CustomRow`, one may implement or derive [`IntoDataFrameRow`],
//! thus being able to convert `Vec<CustomRow>` to an instance of `DataFrame<CustomRow>`,
//! see [`DataFrame`].
//!
//!
//! [`IntoDataFrameRow`]: ::extendr_macros::IntoDataFrameRow
//!
use super::*;

/// A trait to convert a collection of `IntoDataFrameRow` into
/// [`DataFrame`]. Typical usage involves using the derive-macro [`IntoDataFrameRow`]
/// on a struct, which would generate `impl IntoDataFrameRow<T> for Vec<T>`.
///
/// [`IntoDataFrameRow`]: ::extendr_macros::IntoDataFrameRow
pub trait IntoDataFrameRow<T> {
    fn into_dataframe(self) -> Result<DataFrame<T>>;
}

/// A representation of a typed `data.frame`
///
/// A `data.frame` can be created from Rust by using the [`IntoDataFrameRow`] trait
/// which can be derived for a single `struct` that represents a single row.
/// The type of the row is captured by the marker `T`.
///
/// Note that at present, you can create a `DataFrame<T>` but you cannot extract
/// `T` from the object. `<T>` is purely a marker that indicates the struct that
/// was used to create its rows.
///
/// As a result, using `DataFrame<T>` as a function argument _will not_ perform
/// any type checking on the type.
#[derive(PartialEq, Clone)]
pub struct DataFrame<T> {
    pub(crate) robj: RObj,
    _marker: std::marker::PhantomData<T>,
}

impl<T> From<DataFrame<T>> for RObj {
    fn from(value: DataFrame<T>) -> Self {
        value.robj
    }
}

impl<T> std::convert::TryFrom<&RObj> for DataFrame<T> {
    type Error = Error;
    fn try_from(robj: &RObj) -> Result<Self> {
        // TODO: check type using derived trait.
        if !(robj.is_list() && robj.inherits("data.frame")) {
            return Err(Error::ExpectedDataFrame(robj.clone()));
        }
        Ok(DataFrame {
            robj: robj.clone(),
            _marker: std::marker::PhantomData,
        })
    }
}

impl<T> std::convert::TryFrom<RObj> for DataFrame<T> {
    type Error = Error;
    fn try_from(robj: RObj) -> Result<Self> {
        (&robj).try_into()
    }
}

impl<T> DataFrame<T> {
    /// Use `#[derive(IntoDataFrameRow)]` to use this.
    pub fn try_from_values<I: IntoDataFrameRow<T>>(iter: I) -> Result<Self> {
        iter.into_dataframe()
    }
}

impl<T> Attributes for DataFrame<T> {}

impl<T> std::fmt::Debug for DataFrame<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "dataframe!({})",
            self.as_list()
                .unwrap()
                .iter()
                .map(|(k, v)| if !k.is_empty() {
                    format!("{}={:?}", k, v)
                } else {
                    format!("{:?}", v)
                })
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl<T> From<Option<DataFrame<T>>> for RObj {
    fn from(value: Option<DataFrame<T>>) -> Self {
        match value {
            None => nil_value(),
            Some(value) => value.into(),
        }
    }
}

#[deprecated(note = "Use DataFrame instead", since = "0.9.0")]
pub type Dataframe<T> = DataFrame<T>;
