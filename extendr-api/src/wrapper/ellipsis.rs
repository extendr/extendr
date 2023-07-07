use crate::prelude::*;
use crate::{Error, Robj, Rtype, Types};

#[derive(PartialEq, Clone)]
pub struct Ellipsis {
    pub(crate) robj: Robj,
}

impl Ellipsis {}

impl<'a> TryFrom<&'a Robj> for Ellipsis {
    type Error = Error;

    fn try_from(robj: &'a Robj) -> std::result::Result<Self, Self::Error> {
        match robj.rtype() {
            Rtype::Dot => Ok(Self { robj: robj.clone() }),
            Rtype::Environment => try_from_env(robj),
            _ => Err(Error::Other("Placeholder".into())),
        }
    }
}

fn try_from_env(env: &Robj) -> Result<Ellipsis> {
    <Environment as TryFrom<&Robj>>::try_from(env)
        .and_then(|e| {
            e.iter()
                .find(|(k, _)| *k == "...")
                .ok_or(Error::Other("Ellipsis missing".into()))
        })
        .and_then(|(_, v)| <Ellipsis as TryFrom<&Robj>>::try_from(&v))
}
