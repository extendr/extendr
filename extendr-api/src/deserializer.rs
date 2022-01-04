

use crate::robj::{Robj, Types, Length};
use crate::{Rany, List};
use crate::error::{Error, Result};
use serde::de::{Visitor, Deserializer, Deserialize, SeqAccess, DeserializeSeed};
use serde::forward_to_deserialize_any;
use std::convert::TryFrom;

struct RobjDeserializer(Robj);

/// Convert any R object to a Deserialize object.
pub fn from_robj<'a, T>(robj: Robj) -> Result<T>
where
    T: Deserialize<'a>,
{
    let deserializer = RobjDeserializer(robj);
    let t = T::deserialize(deserializer)?;
    Ok(t)
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display
    {
        Error::Other(msg.to_string())
    }
}

struct ListGetter {
    list: List,
    elem: usize,
}

impl<'de> SeqAccess<'de> for ListGetter {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.elem >= self.list.len() {
            Ok(None)
        } else {
            let elem = self.elem;
            self.elem += 1;
            let de = RobjDeserializer(self.list.elt(elem)?);
            seed.deserialize(de).map(Some)
        }
    }
}

impl<'de> Deserializer<'de> for RobjDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // println!("deserialize_any robj={:?}", self.0);
        // match self.0.as_any() {
        //     Rany::Null(_) => visitor.visit_unit(),
        //     Rany::Integer(val) => {
        //         visitor.visit_i32(val.as_integer())
        //     }
        //     _ => Err(Error::Other(format!("unexpected {:?}", self.0.rtype()))),
        //     // 'n' => self.deserialize_unit(visitor),
        //     // 't' | 'f' => self.deserialize_bool(visitor),
        //     // '"' => self.deserialize_str(visitor),
        //     // '0'..='9' => self.deserialize_u64(visitor),
        //     // '-' => self.deserialize_i64(visitor),
        //     // '[' => self.deserialize_seq(visitor),
        //     // '{' => self.deserialize_map(visitor),
        //     // _ => Err(Error::Syntax),
        // }
        Err(Error::Other(format!("unexpected {:?}", self.0.rtype())))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Rany::Null(_) = self.0.as_any() {
            visitor.visit_unit()
        } else {
            Err(Error::ExpectedNull(self.0))
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(bool::try_from(self.0)?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(i8::try_from(self.0)?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(i16::try_from(self.0)?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(i32::try_from(self.0)?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(i64::try_from(self.0)?)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i128(i64::try_from(self.0)? as i128)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(u8::try_from(self.0)?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(u16::try_from(self.0)?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(u32::try_from(self.0)?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(u64::try_from(self.0)?)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u128(u64::try_from(self.0)? as u128)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(f32::try_from(self.0)?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(f64::try_from(self.0)?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(s) = self.0.as_str() {
            let mut c= s.chars();
            if let Some(ch) = c.next() {
                if c.next() == None {
                    return visitor.visit_char(ch)
                }
            }
        }
        Err(Error::ExpectedString(self.0))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(s) = self.0.as_str() {
            return visitor.visit_borrowed_str(s)
        }
        Err(Error::ExpectedString(self.0))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(s) = self.0.as_str() {
            return visitor.visit_string(s.into())
        }
        Err(Error::ExpectedString(self.0))
    }
    
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Rany::Raw(val) = self.0.as_any() {
            visitor.visit_bytes(val.as_slice())
        } else {
            Err(Error::ExpectedRaw(self.0))
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Rany::Raw(val) = self.0.as_any() {
            visitor.visit_byte_buf(val.as_slice().to_owned())
        } else {
            Err(Error::ExpectedRaw(self.0))
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Rany::Null(_) = self.0.as_any() {
            visitor.visit_none()
        } else if self.0.is_na() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.0.as_any() {
            Rany::List(val) => {
                let lg = ListGetter { list: val.clone(), elem: 0 };
                Ok(visitor.visit_seq(lg)?)
            }
            _ => Err(Error::ExpectedList(self.0))
        }
    }

    forward_to_deserialize_any! {
        tuple_struct map struct enum identifier ignored_any
    }
}

