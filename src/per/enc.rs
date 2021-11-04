mod error;

use alloc::vec::Vec;

use crate::{Encode, types::{self, Tag}};

pub use error::Error;

pub struct Encoder {
    output: types::BitString,
}

impl Encoder {
    pub fn new() -> Self {
        Self { output: <_>::default() }
    }

    fn encode_length(&mut self, length: usize) {
    }

    fn extend<'input>(&mut self, input: impl Into<Input<'input>>) {
        match input.into() {
            Input::Bits(bits) => {
                self.output.extend_from_bitslice(bits);
            }
            Input::Bytes(bytes) => {
                self.output.extend(bytes);
            }
        }
    }

    fn encode_non_negative_binary_integer(&mut self, value: types::Integer) {
        todo!()
    }
}

pub enum Input<'input> {
    Bits(&'input types::BitString),
    Bytes(&'input [u8]),
}

impl<'input> From<&'input types::BitString> for Input<'input> {
    fn from(value: &'input types::BitString) -> Self {
        Self::Bits(value)
    }
}

impl<'input> From<&'input [u8]> for Input<'input> {
    fn from(value: &'input [u8]) -> Self {
        Self::Bytes(value)
    }
}

impl<'input> From<&'input Vec<u8>> for Input<'input> {
    fn from(value: &'input Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl crate::Encoder for Encoder {
    type Ok = ();
    type Error = Error;

    fn encode_any(&mut self, value: &types::Any) -> Result<Self::Ok, Self::Error> {
        self.encode_length(value.contents.len());
        self.extend(&value.contents);
        Ok(())
    }

    fn encode_bit_string(
        &mut self,
        tag: Tag,
        value: &types::BitString,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_bool(&mut self, _: Tag, value: bool) -> Result<Self::Ok, Self::Error> {
        self.output.push(value);
        Ok(())
    }

    fn encode_enumerated(&mut self, tag: Tag, value: isize) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_integer(
        &mut self,
        tag: Tag,
        value: &types::Integer,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_null(&mut self, tag: Tag) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_object_identifier(&mut self, tag: Tag, oid: &[u32]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_octet_string(&mut self, tag: Tag, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_utf8_string(&mut self, tag: Tag, value: &str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_utc_time(
        &mut self,
        tag: Tag,
        value: &types::UtcTime,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_generalized_time(
        &mut self,
        tag: Tag,
        value: &types::GeneralizedTime,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_sequence_of<E: Encode>(
        &mut self,
        tag: Tag,
        values: &[E],
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_set_of<E: Encode>(
        &mut self,
        tag: Tag,
        values: &types::SetOf<E>,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_explicit_prefix<V: Encode>(
        &mut self,
        tag: Tag,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_sequence<F>(&mut self, tag: Tag, encoder_scope: F) -> Result<Self::Ok, Self::Error>
    where
        F: FnOnce(&mut Self) -> Result<Self::Ok, Self::Error>,
    {
        todo!()
    }

    fn encode_set<F>(&mut self, tag: Tag, encoder_scope: F) -> Result<Self::Ok, Self::Error>
    where
        F: FnOnce(&mut Self) -> Result<Self::Ok, Self::Error>,
    {
        todo!()
    }
}
