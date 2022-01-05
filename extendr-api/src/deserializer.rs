//! Convert R objects to a wide variety of types.
//!
use crate::error::{Error, Result};
use crate::robj::{Attributes, Length, Robj, Types};
use crate::scalar::Rint;
use crate::wrapper::{Rstr, Strings};
use crate::Rany;
use serde::de::{
    Deserialize, DeserializeSeed, Deserializer, EnumAccess, MapAccess, SeqAccess, VariantAccess,
    Visitor,
};
use serde::forward_to_deserialize_any;
use std::convert::TryFrom;

/// Convert any R object to a Deserialize object.
pub fn from_robj<'de, T>(robj: &'de Robj) -> Result<T>
where
    T: Deserialize<'de>,
{
    // let deserializer = RobjDeserializer(robj);
    let t = T::deserialize(robj)?;
    Ok(t)
}

// Allow errors to popagate to extendr errors.
impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Other(msg.to_string())
    }
}

// Convert unnamed lists to sequences.
struct ListGetter<'a> {
    list: &'a [Robj],
}

impl<'de> SeqAccess<'de> for ListGetter<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.list.is_empty() {
            Ok(None)
        } else {
            let e = &self.list[0];
            self.list = &self.list[1..];
            seed.deserialize(e).map(Some)
        }
    }
}

// Convert named lists to maps.
struct NamedListGetter<'a> {
    keys: &'a [Rstr],
    values: &'a [Robj],
}

impl<'de> MapAccess<'de> for NamedListGetter<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.keys.is_empty() {
            Ok(None)
        } else {
            let e = &self.keys[0];
            self.keys = &self.keys[1..];
            seed.deserialize(e).map(Some)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        let e = &self.values[0];
        self.values = &self.values[1..];
        seed.deserialize(e)
    }
}

// Allow us to use Integers, Doubles and Logicals.
struct SliceGetter<'a, E> {
    list: &'a [E],
}

// Allow us to use Integers and Rint.
impl<'de> Deserializer<'de> for Rint {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(val) = self.into() {
            visitor.visit_i32(val)
        } else {
            Err(Error::MustNotBeNA(Robj::from(())))
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(val) = self.into() {
            visitor.visit_i32(val)
        } else {
            Err(Error::MustNotBeNA(Robj::from(())))
        }
    }

    forward_to_deserialize_any! {
        bool i8 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

// Decode identifiers from the "names" attribute of lists.
impl<'de> Deserializer<'de> for &'de Rstr {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        return visitor.visit_borrowed_str(self.as_str());
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum ignored_any
    }
}

// Get the variant name and content of an enum.
impl<'de> EnumAccess<'de> for &'de Robj {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        match self.as_any() {
            Rany::String(s) if s.len() == 1 => {
                let variant = seed.deserialize(self)?;
                Ok((variant, self))
            }
            Rany::List(list) if list.len() == 1 => {
                if let Some(keys) = self.get_attrib(crate::wrapper::symbol::names_symbol()) {
                    if let Ok(keys) = Strings::try_from(keys) {
                        let keys = keys.as_slice();
                        let values = &list.as_slice()[0];
                        let variant = seed.deserialize(&keys[0])?;
                        return Ok((variant, values));
                    }
                }
                Err(Error::Other("Expected named List for enum".into()))
            }
            _ => Err(Error::Other("Expected String or List for enum".into())),
        }
    }
}

// Decode enum variants of various kinds.
impl<'de> VariantAccess<'de> for &'de Robj {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_seq(self, visitor)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_map(self, visitor)
    }
}

// Enable sequences from Integers, Doubles etc.
impl<'de> SeqAccess<'de> for SliceGetter<'de, Rint> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.list.is_empty() {
            Ok(None)
        } else {
            let res = seed.deserialize(self.list[0]).map(Some);
            self.list = &self.list[1..];
            res
        }
    }
}

// Given an Robj, generate a value of many kinds.
impl<'de> Deserializer<'de> for &'de Robj {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::Other(format!("unexpected {:?}", self.rtype())))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Rany::Null(_) = self.as_any() {
            visitor.visit_unit()
        } else {
            Err(Error::ExpectedNull(self.clone()))
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(bool::try_from(self.clone())?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(i8::try_from(self.clone())?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(i16::try_from(self.clone())?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(i32::try_from(self.clone())?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(i64::try_from(self.clone())?)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i128(i64::try_from(self.clone())? as i128)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(u8::try_from(self.clone())?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(u16::try_from(self.clone())?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(u32::try_from(self.clone())?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(u64::try_from(self.clone())?)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u128(u64::try_from(self.clone())? as u128)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(f32::try_from(self.clone())?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(f64::try_from(self.clone())?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let s = <&str>::try_from(self.clone())?;
        let mut c = s.chars();
        if let Some(ch) = c.next() {
            if c.next() == None {
                return visitor.visit_char(ch);
            }
        }
        Err(Error::ExpectedString(self.clone()))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(<&str>::try_from(self.clone())?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(<&str>::try_from(self.clone())?.into())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Rany::Raw(val) = self.as_any() {
            visitor.visit_bytes(val.as_slice())
        } else {
            Err(Error::ExpectedRaw(self.clone()))
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Rany::Raw(val) = self.as_any() {
            visitor.visit_byte_buf(val.as_slice().to_owned())
        } else {
            Err(Error::ExpectedRaw(self.clone()))
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #![allow(clippy::if_same_then_else)]
        if let Rany::Null(_) = self.as_any() {
            visitor.visit_none()
        } else if self.is_na() {
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

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.as_any() {
            Rany::List(val) => {
                let lg = ListGetter {
                    list: val.as_slice(),
                };
                Ok(visitor.visit_seq(lg)?)
            }
            Rany::Integer(val) => {
                let lg = SliceGetter { list: &*val };
                Ok(visitor.visit_seq(lg)?)
            }
            _ => Err(Error::ExpectedList(self.clone())),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.as_any() {
            Rany::List(val) => {
                if let Some(keys) = self.get_attrib(crate::wrapper::symbol::names_symbol()) {
                    if let Ok(keys) = Strings::try_from(keys) {
                        let keys = keys.as_slice();
                        let lg = NamedListGetter {
                            keys,
                            values: val.as_slice(),
                        };
                        return visitor.visit_map(lg);
                    }
                }
                Err(Error::ExpectedList(self.clone()))
            }
            _ => Err(Error::ExpectedList(self.clone())),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self)
    }
}
