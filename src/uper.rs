//! # Aligned Packed Encoding Rules

/// Attempts to decode `T` from `input` using DER.
pub fn decode<T: crate::Decode>(input: &[u8]) -> Result<T, crate::per::de::Error> {
    todo!()
    // T::decode(&mut crate::per::de::Decoder::new(
    //     input,
    //     crate::per::de::DecoderOptions::der(),
    // ))
}

/// Attempts to encode `value` to DER.
pub fn encode<T: crate::Encode>(value: &T) -> Result<alloc::vec::Vec<u8>, crate::per::enc::Error> {
    todo!()
    // let mut enc = crate::per::enc::Encoder::new(crate::per::enc::EncoderOptions::aper());

    // value.encode(&mut enc)?;

    // Ok(enc.output())
}
