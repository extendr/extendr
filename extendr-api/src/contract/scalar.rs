use crate::{CanBeNA, Robj};

pub trait GenericScalar: Sized + Default + Copy + Clone + CanBeNA + PartialEq + Into<Robj> {}
pub trait Scalar<T> : GenericScalar + PartialEq<T> + Into<Option<T>> + From<T> + From<Option<T>>
    where T : Sized + Copy + Clone + PartialEq
{
    fn inner(&self) -> T;
}
