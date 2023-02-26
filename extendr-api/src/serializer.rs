//! See <https://serde.rs/impl-serializer.html>

use crate::error::{Error, Result};
use crate::na::CanBeNA;
use crate::robj::{Attributes, GetSexp, Length, Rinternals, Types};
use crate::scalar::{Rbool, Rfloat, Rint};
use crate::wrapper::{
    Doubles, Environment, Expressions, Function, Integers, Language, Logicals, Pairlist, Primitive,
    Promise, Raw, Rstr, Symbol, S4,
};
use crate::{List, Rany, Robj};
use serde::{ser, Serialize};

impl ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::from(msg.to_string())
    }
}

struct RobjSerializer {
    robj: Option<Robj>,
}

struct SerializeSeq<'a> {
    values: Vec<Robj>,
    parent: &'a mut RobjSerializer,
}

struct SerializeTuple<'a> {
    values: Vec<Robj>,
    parent: &'a mut RobjSerializer,
}

struct SerializeTupleStruct<'a> {
    values: Vec<Robj>,
    parent: &'a mut RobjSerializer,
}

struct SerializeTupleVariant<'a> {
    values: Vec<Robj>,
    parent: &'a mut RobjSerializer,
    variant: String,
}

struct SerializeMap<'a> {
    values: Vec<(String, Robj)>,
    key: String,
    parent: &'a mut RobjSerializer,
}

struct SerializeStruct<'a> {
    values: Vec<(String, Robj)>,
    parent: &'a mut RobjSerializer,
}

struct SerializeStructVariant<'a> {
    values: Vec<(String, Robj)>,
    parent: &'a mut RobjSerializer,
    variant: String,
}

/// Convert a serializable object to a Robj.
///
/// Requires the "serde" feature.
///
/// ```toml
/// extendr-api = { version = "0.4", features = ["serde"] }
/// ```
///
/// Example:
///
/// ```rust
/// use extendr_api::prelude::*;
/// use extendr_api::serializer::to_robj;
/// use serde::Serialize;
/// test! {
///     #[derive(Serialize)]
///     struct Test {
///         int: i32,
///         seq: Vec<&'static str>,
///     }
///
///     let test = Test {
///         int: 1,
///         seq: vec!["a", "b"],
///     };
///
///     let expected = list!(int=1, seq=list!("a", "b"));
///     assert_eq!(to_robj(&test).unwrap(), Robj::from(expected));
/// }
/// ```
pub fn to_robj<T>(value: &T) -> Result<Robj>
where
    T: Serialize,
{
    let mut serializer = RobjSerializer { robj: None };

    value.serialize(&mut serializer)?;
    Ok(serializer.robj.unwrap())
}

impl<'a> ser::Serializer for &'a mut RobjSerializer {
    // The output type produced by this `RobjSerializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `RobjSerializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = Error;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // RobjSerializer struct.
    type SerializeSeq = self::SerializeSeq<'a>;
    type SerializeTuple = self::SerializeTuple<'a>;
    type SerializeTupleStruct = self::SerializeTupleStruct<'a>;
    type SerializeTupleVariant = self::SerializeTupleVariant<'a>;
    type SerializeMap = self::SerializeMap<'a>;
    type SerializeStruct = self::SerializeStruct<'a>;
    type SerializeStructVariant = self::SerializeStructVariant<'a>;

    /// Map a bool to a Robj.
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.robj = Some(Robj::from(v));
        Ok(())
    }

    /// Map a i8 to a Robj.
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.robj = Some(Robj::from(v));
        Ok(())
    }

    /// Map a i16 to a Robj.
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.robj = Some(Robj::from(v));
        Ok(())
    }

    /// Map a i32 to a Robj.
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.robj = Some(Robj::from(v));
        Ok(())
    }

    /// Map a i64 to a Robj.
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.robj = Some(Robj::from(v));
        Ok(())
    }

    /// Map a u8 to a Robj.
    fn serialize_u8(self, v: u8) -> Result<()> {
        self.robj = Some(Robj::from(v));
        Ok(())
    }

    /// Map a u16 to a Robj.
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.robj = Some(Robj::from(v));
        Ok(())
    }

    /// Map a u32 to a Robj.
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.robj = Some(Robj::from(v));
        Ok(())
    }

    /// Map a u64 to a Robj.
    fn serialize_u64(self, v: u64) -> Result<()> {
        self.robj = Some(Robj::from(v));
        Ok(())
    }

    /// Map a f32 to a Robj.
    fn serialize_f32(self, v: f32) -> Result<()> {
        self.robj = Some(Robj::from(v));
        Ok(())
    }

    /// Map a f64 to a Robj.
    fn serialize_f64(self, v: f64) -> Result<()> {
        self.robj = Some(Robj::from(v));
        Ok(())
    }

    /// Map a char to a Robj string.
    fn serialize_char(self, v: char) -> Result<()> {
        self.robj = Some(Robj::from(v.to_string()));
        Ok(())
    }

    /// Map a string slice to a Robj string.
    fn serialize_str(self, v: &str) -> Result<()> {
        self.robj = Some(Robj::from(v));
        Ok(())
    }

    /// Raw objects.
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.robj = Some(Robj::from(v));
        Ok(())
    }

    /// None of an option is NULL.
    fn serialize_none(self) -> Result<()> {
        self.robj = Some(Robj::from(()));
        Ok(())
    }

    /// Some of an option is that value.
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // In Serde, unit means an anonymous value containing no data. Map this to
    // R as `NULL`.
    fn serialize_unit(self) -> Result<()> {
        self.robj = Some(Robj::from(()));
        Ok(())
    }

    // Unit struct means a named value containing no data. Again, since there is
    // no data, map this to R as `NULL`. There is no need to serialize the
    // name in most formats.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    /// Unit variant: Enum::Name
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.robj = Some(Robj::from(variant));
        Ok(())
    }

    /// Wrapper struct: Wrap(T)
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    /// Wrapper struct in enum: Enum::Wrap(T)
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let value = to_robj(&value)?;
        let list = List::from_pairs([(variant, value)]);
        self.robj = Some(list.into());
        Ok(())
    }

    /// Start of a vector or other sequence.
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        let vec = if let Some(len) = len {
            Vec::with_capacity(len)
        } else {
            Vec::new()
        };
        Ok(SerializeSeq {
            values: vec,
            parent: self,
        })
    }

    /// Start of a tuple.
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(SerializeTuple {
            values: Vec::with_capacity(len),
            parent: self,
        })
    }

    /// Start of a tuple struct eg. `Point(i32, i32)`.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(Self::SerializeTupleStruct {
            values: Vec::with_capacity(len),
            parent: self,
        })
    }

    /// Start of a struct variant eg. `Enum::Point{ x: i32, y: i32}`.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(SerializeTupleVariant {
            values: Vec::with_capacity(len),
            variant: variant.to_string(),
            parent: self,
        })
    }

    /// Start of a map. We require that keys must be strings.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SerializeMap {
            values: Vec::new(),
            key: String::new(),
            parent: self,
        })
    }

    /// Start of a struct. Collect just the values in a list.
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        Ok(SerializeStruct {
            values: Vec::with_capacity(len),
            parent: self,
        })
    }

    /// eg. `Enum::Point { x: i32, y: i32 }`
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(SerializeStructVariant {
            values: Vec::new(),
            variant: variant.to_string(),
            parent: self,
        })
    }
}

/// Arrays, vectors and slices -> List
impl<'a> ser::SerializeSeq for self::SerializeSeq<'a> {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.values.push(to_robj(&value)?);
        Ok(())
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        self.parent.robj = Some(List::from_values(self.values).into());
        Ok(())
    }
}

/// Tuples -> list!(...)
impl<'a> ser::SerializeTuple for SerializeTuple<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.values.push(to_robj(&value)?);
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.parent.robj = Some(List::from_values(self.values).into());
        Ok(())
    }
}

/// Tuple structs -> list!(...)
impl<'a> ser::SerializeTupleStruct for self::SerializeTupleStruct<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.values.push(to_robj(&value)?);
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.parent.robj = Some(List::from_values(self.values).into());
        Ok(())
    }
}

/// Tuple variants -> list!(variant = list!(...))
impl<'a> ser::SerializeTupleVariant for self::SerializeTupleVariant<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.values.push(to_robj(&value)?);
        Ok(())
    }

    fn end(self) -> Result<()> {
        let value = List::from_values(self.values).into();
        let list = List::from_pairs([(self.variant, value)]);
        self.parent.robj = Some(list.into());
        Ok(())
    }
}

/// Maps must have string keys -> list!(key=value, ...)
impl<'a> ser::SerializeMap for self::SerializeMap<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let key = to_robj(&key)?;
        if let Some(key_str) = key.as_str() {
            self.key = key_str.to_string();
            Ok(())
        } else {
            Err(Error::ExpectedString(key))
        }
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let key = std::mem::take(&mut self.key);
        self.values.push((key, to_robj(&value)?));
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.parent.robj = Some(List::from_pairs(self.values).into());
        Ok(())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a> ser::SerializeStruct for self::SerializeStruct<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.values.push((key.to_string(), to_robj(&value)?));
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.parent.robj = Some(List::from_pairs(self.values).into());
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`.
impl<'a> ser::SerializeStructVariant for self::SerializeStructVariant<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.values.push((key.to_string(), to_robj(&value)?));
        Ok(())
    }

    fn end(self) -> Result<()> {
        let value = List::from_pairs(self.values).into();
        let list = List::from_pairs([(self.variant, value)]);
        self.parent.robj = Some(list.into());
        Ok(())
    }
}

impl ser::Serialize for Symbol {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl ser::Serialize for Primitive {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let string = self.deparse().unwrap();
        string.serialize(serializer)
    }
}

impl ser::Serialize for Expressions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let string = self.deparse().unwrap();
        string.serialize(serializer)
    }
}

impl ser::Serialize for S4 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let string = self.deparse().unwrap();
        string.serialize(serializer)
    }
}

impl ser::Serialize for Pairlist {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut s = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self.iter() {
            s.serialize_key(&k)?;
            s.serialize_value(&v)?;
        }
        s.end()
    }
}

impl ser::Serialize for Function {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_unit()
        // if let (Some(formals), Some(body), Some(environment)) = (
        //     self.formals(), self.body(), self.environment()
        // ) {
        //     use crate as extendr_api;
        //     list!(formals=formals, body=body, environment=environment).serialize(serializer)
        // } else {
        //     serializer.serialize_str("Function")
        // }
    }
}

impl ser::Serialize for Environment {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_unit()
    }
}

impl ser::Serialize for Promise {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_unit()
    }
}

impl ser::Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_unit()
    }
}

impl ser::Serialize for Integers {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if self.len() == 1 {
            self.elt(0).serialize(serializer)
        } else {
            use serde::ser::SerializeSeq;
            let mut s = serializer.serialize_seq(Some(self.len()))?;
            for v in self.iter() {
                s.serialize_element(&v)?;
            }
            s.end()
        }
    }
}

impl ser::Serialize for Logicals {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if self.len() == 1 {
            self.elt(0).serialize(serializer)
        } else {
            use serde::ser::SerializeSeq;
            let mut s = serializer.serialize_seq(Some(self.len()))?;
            for v in self.iter() {
                s.serialize_element(&v)?;
            }
            s.end()
        }
    }
}

impl ser::Serialize for List {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if self.has_names() {
            serializer.collect_map(self.iter())
        } else {
            serializer.collect_seq(self.values())
        }
    }
}

impl ser::Serialize for Doubles {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if self.len() == 1 {
            self.elt(0).serialize(serializer)
        } else {
            use serde::ser::SerializeSeq;
            let mut s = serializer.serialize_seq(Some(self.len()))?;
            for v in self.iter() {
                s.serialize_element(&v)?;
            }
            s.end()
        }
    }
}

impl ser::Serialize for Rstr {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if self.is_na() {
            serializer.serialize_unit()
        } else {
            serializer.serialize_str(self.as_str())
        }
    }
}

impl ser::Serialize for Raw {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_bytes(self.as_robj().as_raw_slice().unwrap())
    }
}

impl ser::Serialize for Rint {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if let Some(v) = (*self).into() {
            serializer.serialize_i32(v)
        } else {
            serializer.serialize_unit()
        }
    }
}

impl ser::Serialize for Rfloat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if let Some(v) = (*self).into() {
            serializer.serialize_f64(v)
        } else {
            serializer.serialize_unit()
        }
    }
}

impl ser::Serialize for Rbool {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if let Some(v) = (*self).into() {
            serializer.serialize_bool(v)
        } else {
            serializer.serialize_unit()
        }
    }
}

impl ser::Serialize for Robj {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self.as_any() {
            Rany::Null(_) => serializer.serialize_unit(),
            Rany::Symbol(value) => value.serialize(serializer),
            Rany::Pairlist(value) => value.serialize(serializer),
            Rany::Function(value) => value.serialize(serializer),
            Rany::Environment(value) => value.serialize(serializer),
            Rany::Promise(value) => value.serialize(serializer),
            Rany::Language(value) => value.serialize(serializer),
            Rany::Special(value) => value.serialize(serializer),
            Rany::Builtin(value) => value.serialize(serializer),
            Rany::Rstr(value) => value.serialize(serializer),
            Rany::Logicals(value) => value.serialize(serializer),
            Rany::Integers(value) => value.serialize(serializer),
            Rany::Doubles(value) => value.serialize(serializer),
            Rany::Complexes(_complex) => serializer.serialize_unit(),
            Rany::Strings(value) => value.serialize(serializer),
            Rany::Dot(_dot) => serializer.serialize_unit(),
            Rany::Any(_any) => serializer.serialize_unit(),
            Rany::List(value) => value.serialize(serializer),
            Rany::Expressions(value) => value.serialize(serializer),
            Rany::Bytecode(_bytecode) => serializer.serialize_unit(),
            Rany::ExternalPtr(_externalptr) => serializer.serialize_unit(),
            Rany::WeakRef(_weakref) => serializer.serialize_unit(),
            Rany::Raw(value) => value.serialize(serializer),
            Rany::S4(value) => value.serialize(serializer),
            Rany::Unknown(_unknown) => serializer.serialize_unit(),
        }
    }
}
