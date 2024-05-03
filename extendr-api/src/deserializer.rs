//! Convert R objects to a wide variety of types.
//!
use crate::error::{Error, Result};
use crate::na::CanBeNA;
use crate::robj::{Attributes, Length, Robj, Types};
use crate::scalar::{Rbool, Rfloat, Rint};
use crate::wrapper::{Doubles, Integers, List, Logicals, Rstr, Strings};
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
    let t = T::deserialize(robj)?;
    Ok(t)
}

// Allow errors to propagate to extendr errors.
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

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(val) = self.into() {
            visitor.visit_i32(val)
        } else {
            visitor.visit_unit()
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

// Allow us to use Doubles and Rfloat.
impl<'de> Deserializer<'de> for Rfloat {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(val) = self.into() {
            visitor.visit_f64(val)
        } else {
            visitor.visit_unit()
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

// Allow us to use Logicals and Rbool.
impl<'de> Deserializer<'de> for Rbool {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(val) = self.into() {
            visitor.visit_bool(val)
        } else {
            visitor.visit_unit()
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
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

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.as_str())
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char string
        str bytes byte_buf option unit unit_struct newtype_struct seq tuple
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
            Rany::Strings(s) if s.len() == 1 => {
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

// Enable sequences from Integers, Doubles and Logicals.
impl<'de, E: Deserializer<'de>> SeqAccess<'de> for SliceGetter<'de, E>
where
    Error: From<<E as Deserializer<'de>>::Error>,
    E: Copy,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.list.is_empty() {
            Ok(None)
        } else {
            let res = seed.deserialize(self.list[0])?;
            self.list = &self.list[1..];
            Ok(Some(res))
        }
    }
}

// Given an Robj, generate a value of many kinds.
impl<'de> Deserializer<'de> for &'de Robj {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.len();
        match self.as_any() {
            Rany::Null(_) => self.deserialize_unit(visitor),
            Rany::Integers(_v) => {
                if len == 1 {
                    self.deserialize_i32(visitor)
                } else {
                    self.deserialize_seq(visitor)
                }
            }
            Rany::Doubles(_v) => {
                if len == 1 {
                    self.deserialize_f64(visitor)
                } else {
                    self.deserialize_seq(visitor)
                }
            }
            Rany::Logicals(_v) => {
                if len == 1 {
                    self.deserialize_bool(visitor)
                } else {
                    self.deserialize_seq(visitor)
                }
            }
            Rany::List(_v) => self.deserialize_seq(visitor),
            Rany::Strings(_v) => {
                if len == 1 {
                    self.deserialize_str(visitor)
                } else {
                    self.deserialize_seq(visitor)
                }
            }
            _ => Err(Error::Other(format!(
                "deserialize_any: unexpected {:?}",
                self.rtype()
            ))),
        }
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
            Rany::Integers(val) => {
                let lg = SliceGetter { list: &*val };
                Ok(visitor.visit_seq(lg)?)
            }
            Rany::Doubles(val) => {
                let lg = SliceGetter { list: &*val };
                Ok(visitor.visit_seq(lg)?)
            }
            Rany::Logicals(val) => {
                let lg = SliceGetter { list: &*val };
                Ok(visitor.visit_seq(lg)?)
            }
            Rany::Strings(_val) => {
                // Grubby hack that will go away once PRs are merged.
                // use std::convert::TryInto;
                // let val : Strings = val.clone().try_into().unwrap();
                // let lg = StringGetter { list: &*val };
                // Ok(visitor.visit_seq(lg)?)
                unimplemented!("Deserialize shortcut for Strings");
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

struct RintVisitor;

impl<'de> Visitor<'de> for RintVisitor {
    type Value = Rint;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between -2^31+1 and 2^31")
    }

    fn visit_i32<E>(self, value: i32) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.into())
    }

    fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Rint::na())
    }
}

impl<'de> Deserialize<'de> for Rint {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Rint, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i32(RintVisitor)
    }
}

struct RfloatVisitor;

impl<'de> Visitor<'de> for RfloatVisitor {
    type Value = Rfloat;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a floating point value")
    }

    fn visit_f64<E>(self, value: f64) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.into())
    }

    fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Rfloat::na())
    }
}

impl<'de> Deserialize<'de> for Rfloat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Rfloat, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_f64(RfloatVisitor)
    }
}

struct RboolVisitor;

impl<'de> Visitor<'de> for RboolVisitor {
    type Value = Rbool;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a boolean point value")
    }

    fn visit_bool<E>(self, value: bool) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.into())
    }

    fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Rbool::na())
    }
}

impl<'de> Deserialize<'de> for Rbool {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Rbool, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bool(RboolVisitor)
    }
}

struct RobjVisitor;

impl<'de> Visitor<'de> for RobjVisitor {
    type Value = Robj;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a value convertable to a Robj")
    }

    fn visit_bool<E>(self, value: bool) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.into())
    }

    fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v > i32::MIN as i64 && v <= i32::MAX as i64 {
            Ok((v as i32).into())
        } else {
            Ok((v as f64).into())
        }
    }

    fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v <= i32::MAX as u64 {
            Ok((v as i32).into())
        } else {
            Ok((v as f64).into())
        }
    }

    fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.into())
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.into())
    }

    fn visit_bytes<E>(self, v: &[u8]) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.into())
    }

    fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Robj::from(()))
    }

    fn visit_some<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        // All sequences get converted to lists at the moment.
        // We could check the first element and then assume the rest are the sme.
        let mut values: Vec<Robj> = Vec::with_capacity(seq.size_hint().unwrap_or(8));
        while let Some(value) = seq.next_element()? {
            values.push(value);
        }
        Ok(values.into())
    }

    fn visit_map<M>(self, mut access: M) -> std::result::Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut keys: Vec<&str> = Vec::with_capacity(access.size_hint().unwrap_or(8));
        let mut values: Vec<Robj> = Vec::with_capacity(access.size_hint().unwrap_or(8));

        while let Some((key, value)) = access.next_entry()? {
            keys.push(key);
            values.push(value);
        }

        Ok(List::from_values(values).set_names(keys).unwrap())
    }

    fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Robj::from(()))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_enum<A>(self, _data: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        // TODO: find an example of this.
        unimplemented!();
        // let (de, variant) = data.variant()?;
        // de.deserialize_any(self)
    }
}

impl<'de> Deserialize<'de> for Robj {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Robj, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(RobjVisitor)
    }
}

struct IntegersVisitor;

impl<'de> Visitor<'de> for IntegersVisitor {
    type Value = Integers;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a value convertable to Integers")
    }

    fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v > i32::MIN as i64 && v <= i32::MAX as i64 {
            Ok(Integers::from_values([v as i32]))
        } else {
            Err(serde::de::Error::custom("out of range for Integers"))
        }
    }

    fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v <= i32::MAX as u64 {
            Ok(Integers::from_values([v as i32]))
        } else {
            Err(serde::de::Error::custom("out of range for Integers"))
        }
    }

    fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Integers::from_values([Rint::na()]))
    }

    fn visit_some<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values: Vec<Rint> = Vec::with_capacity(seq.size_hint().unwrap_or(8));
        while let Some(value) = seq.next_element()? {
            values.push(value);
        }
        Ok(Integers::from_values(values))
    }

    fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Integers::from_values([Rint::na()]))
    }
}

impl<'de> Deserialize<'de> for Integers {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Integers, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(IntegersVisitor)
    }
}

struct DoublesVisitor;

impl<'de> Visitor<'de> for DoublesVisitor {
    type Value = Doubles;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a value convertable to Doubles")
    }

    fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Doubles::from_values([v as f64]))
    }

    fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Doubles::from_values([v as f64]))
    }

    fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Doubles::from_values([v]))
    }

    fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Doubles::from_values([Rfloat::na()]))
    }

    fn visit_some<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values: Vec<Rfloat> = Vec::with_capacity(seq.size_hint().unwrap_or(8));
        while let Some(value) = seq.next_element()? {
            values.push(value);
        }
        Ok(Doubles::from_values(values))
    }

    fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Doubles::from_values([Rfloat::na()]))
    }
}

impl<'de> Deserialize<'de> for Doubles {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Doubles, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DoublesVisitor)
    }
}

struct LogicalsVisitor;

impl<'de> Visitor<'de> for LogicalsVisitor {
    type Value = Logicals;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a value convertable to Logicals")
    }

    fn visit_bool<E>(self, v: bool) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Logicals::from_values([v]))
    }

    fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Logicals::from_values([Rbool::na()]))
    }

    fn visit_some<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values: Vec<Rbool> = Vec::with_capacity(seq.size_hint().unwrap_or(8));
        while let Some(value) = seq.next_element()? {
            values.push(value);
        }
        Ok(Logicals::from_values(values))
    }

    fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Logicals::from_values([Rbool::na()]))
    }
}

impl<'de> Deserialize<'de> for Logicals {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Logicals, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(LogicalsVisitor)
    }
}

struct StringsVisitor;

impl<'de> Visitor<'de> for StringsVisitor {
    type Value = Strings;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a value convertable to Strings")
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.into())
    }

    fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Strings::from_values([<&str>::na()]))
    }

    fn visit_some<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values: Vec<Rstr> = Vec::with_capacity(seq.size_hint().unwrap_or(8));
        while let Some(value) = seq.next_element()? {
            values.push(value);
        }
        Ok(Strings::from_values(values))
    }

    fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Strings::from_values([<&str>::na()]))
    }
}

impl<'de> Deserialize<'de> for Strings {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Strings, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StringsVisitor)
    }
}

struct RstrVisitor;

impl<'de> Visitor<'de> for RstrVisitor {
    type Value = Rstr;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a value convertable to Rstr")
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.into())
    }

    fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Rstr::from_string(<&str>::na()))
    }

    fn visit_some<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Rstr::from_string(<&str>::na()))
    }
}

impl<'de> Deserialize<'de> for Rstr {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Rstr, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(RstrVisitor)
    }
}
