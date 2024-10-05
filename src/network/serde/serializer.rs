use super::error::{Error, SerdeResult};
use crate::Result;
use serde::{ser, Serialize};

pub struct Serializer {
    output: Vec<i8>,
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<i8>>
where
    T: Serialize,
{
    let mut serializer = Serializer { output: vec![] };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeMap = Self;
    type SerializeSeq = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;

    fn serialize_bool(self, v: bool) -> SerdeResult<()> {
        self.output.push(i8::from(v));

        Ok(())
    }

    fn serialize_i8(self, v: i8) -> SerdeResult<()> {
        self.output.push(v);

        Ok(())
    }

    #[allow(clippy::cast_possible_wrap)]
    fn serialize_i16(self, v: i16) -> SerdeResult<()> {
        for byte in v.to_be_bytes() {
            self.output.push(byte as i8);
        }

        Ok(())
    }

    #[allow(clippy::cast_possible_wrap)]
    fn serialize_i32(self, v: i32) -> SerdeResult<()> {
        for byte in v.to_be_bytes() {
            self.output.push(byte as i8);
        }

        Ok(())
    }

    #[allow(clippy::cast_possible_wrap)]
    fn serialize_i64(self, v: i64) -> SerdeResult<()> {
        for byte in v.to_be_bytes() {
            self.output.push(byte as i8);
        }

        Ok(())
    }

    #[allow(clippy::cast_possible_wrap)]
    fn serialize_u8(self, v: u8) -> SerdeResult<()> {
        self.output.push(v as i8);

        Ok(())
    }

    #[allow(clippy::cast_possible_wrap)]
    fn serialize_u16(self, v: u16) -> SerdeResult<()> {
        for byte in v.to_be_bytes() {
            self.output.push(byte as i8);
        }

        Ok(())
    }

    #[allow(clippy::cast_possible_wrap)]
    fn serialize_u32(self, v: u32) -> SerdeResult<()> {
        for byte in v.to_be_bytes() {
            self.output.push(byte as i8);
        }

        Ok(())
    }

    #[allow(clippy::cast_possible_wrap)]
    fn serialize_u64(self, v: u64) -> SerdeResult<()> {
        for byte in v.to_be_bytes() {
            self.output.push(byte as i8);
        }

        Ok(())
    }

    #[allow(clippy::cast_possible_wrap)]
    fn serialize_f32(self, v: f32) -> SerdeResult<()> {
        for byte in v.to_be_bytes() {
            self.output.push(byte as i8);
        }

        Ok(())
    }

    #[allow(clippy::cast_possible_wrap)]
    fn serialize_f64(self, v: f64) -> SerdeResult<()> {
        for byte in v.to_be_bytes() {
            self.output.push(byte as i8);
        }

        Ok(())
    }

    fn serialize_char(self, _v: char) -> SerdeResult<()> {
        unimplemented!();
    }

    #[allow(clippy::cast_possible_wrap)]
    fn serialize_str(self, v: &str) -> SerdeResult<()> {
        let to_java = cesu8::to_java_cesu8(v);

        let len = i16::try_from(to_java.len());
        if let Err(e) = len {
            return Err(Error::Message(e.to_string()));
        }

        self.serialize_i16(len.unwrap())?;
        for byte in to_java.iter().copied() {
            self.output.push(byte as i8);
        }

        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> SerdeResult<()> {
        unimplemented!();
    }

    fn serialize_none(self) -> SerdeResult<()> {
        unimplemented!();
    }

    fn serialize_some<T>(self, _value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!();
    }

    fn serialize_unit(self) -> SerdeResult<()> {
        unimplemented!();
    }

    fn serialize_unit_struct(self, _name: &'static str) -> SerdeResult<()> {
        unimplemented!();
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> SerdeResult<()> {
        unimplemented!();
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!();
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!();
    }

    fn serialize_seq(self, _len: Option<usize>) -> SerdeResult<Self::SerializeSeq> {
        unimplemented!();
    }

    fn serialize_tuple(self, _len: usize) -> SerdeResult<Self::SerializeTuple> {
        unimplemented!();
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> SerdeResult<Self::SerializeTupleStruct> {
        unimplemented!();
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> SerdeResult<Self::SerializeTupleVariant> {
        unimplemented!();
    }

    fn serialize_map(self, _len: Option<usize>) -> SerdeResult<Self::SerializeMap> {
        unimplemented!();
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> SerdeResult<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> SerdeResult<Self::SerializeStructVariant> {
        unimplemented!();
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!();
    }

    fn end(self) -> SerdeResult<()> {
        unimplemented!();
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!();
    }

    fn end(self) -> SerdeResult<()> {
        unimplemented!();
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!();
    }

    fn end(self) -> SerdeResult<()> {
        unimplemented!();
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!();
    }

    fn end(self) -> SerdeResult<()> {
        unimplemented!();
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> SerdeResult<()>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        unimplemented!();
    }

    fn serialize_key<T>(&mut self, _key: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!();
    }

    fn serialize_value<T>(&mut self, _value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!();
    }

    fn end(self) -> SerdeResult<()> {
        unimplemented!();
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> SerdeResult<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> SerdeResult<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!();
    }

    fn end(self) -> SerdeResult<()> {
        unimplemented!();
    }
}
