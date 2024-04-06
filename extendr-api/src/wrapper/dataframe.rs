use super::*;

pub trait IntoDataFrameRow<T> {
    fn into_dataframe(self) -> Result<Dataframe<T>>;
}

/// A representation of a typed `data.frame`
///
/// A `data.frame` can be created from Rust by using the [`IntoDataFrameRow`] trait
/// which can be derived for a single `struct` that represents a single row.
/// The type of the row is captured by the marker `T`.
///
/// Note that at present, you can create a `Dataframe<T>` but you cannot extract
/// `T` from the object. `<T>` is purely a marker that indicates the struct that
/// was used to create its rows.
///
/// As a result, using `Dataframe<T>` as a function argument _will not_ perform
/// any type checking on the type.
#[derive(PartialEq, Clone)]
pub struct Dataframe<T> {
    pub(crate) robj: Robj,
    _marker: std::marker::PhantomData<T>,
}

impl<T> From<Dataframe<T>> for Robj {
    fn from(value: Dataframe<T>) -> Self {
        value.robj
    }
}

impl<T> FromRobj<'_> for Dataframe<T> {
    fn from_robj(robj: &Robj) -> std::result::Result<Self, &'static str> {
        robj.try_into().or(Err("expected a `data.frame`"))
    }
}

impl<T> std::convert::TryFrom<&Robj> for Dataframe<T> {
    type Error = Error;
    fn try_from(robj: &Robj) -> Result<Self> {
        // TODO: check type using derived trait.
        if !(robj.is_list() && robj.inherits("data.frame")) {
            return Err(Error::ExpectedDataframe(robj.clone()));
        }
        Ok(Dataframe {
            robj: robj.clone(),
            _marker: std::marker::PhantomData,
        })
    }
}

impl<T> std::convert::TryFrom<Robj> for Dataframe<T> {
    type Error = Error;
    fn try_from(robj: Robj) -> Result<Self> {
        (&robj).try_into()
    }
}

impl<T> Dataframe<T> {
    /// Use `#[derive(IntoDataFrameRow)]` to use this.
    pub fn try_from_values<I: IntoDataFrameRow<T>>(iter: I) -> Result<Self> {
        iter.into_dataframe()
    }
}

impl<T> Attributes for Dataframe<T> {}

impl<T> std::fmt::Debug for Dataframe<T>
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
