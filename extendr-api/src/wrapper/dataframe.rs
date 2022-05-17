use super::*;

pub trait IntoDataFrameRow<T> {
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
        if robj.is_list() && robj.inherits("data.frame") {
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
