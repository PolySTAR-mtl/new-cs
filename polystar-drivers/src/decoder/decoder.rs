use crate::protocol::{
        Encoding, Endianness, Protocol,
    };

use super::{
    DecodeError,
    DecodedField,
    Value,
};

pub struct Decoder<const N: usize> {
    protocol: &'static Protocol<N>
}

impl<const N: usize> Decoder<N> {
    pub fn new(
        protocol: &'static Protocol<N>
    ) -> Self {
        Self {
            protocol
        }
    }

    /// Decode a complete frame into a fixed-size array.
    pub fn decode(
        &self,
        frame: &[u8],
    ) -> Result<[DecodedField; N], DecodeError> {

        let frame_size_in_bytes = self.protocol.get_frame_size().value();
        if frame.len() != frame_size_in_bytes {
            return Err(DecodeError::InvalidFrameLength {
                expected: frame_size_in_bytes,
                actual: frame.len(),
            });
        }

        let decoded = core::array::from_fn(|i| {
            let field = &self.protocol.get_fields()[i];
            let name = field.get_name();
            let offset = field.get_offset_in_bits();
            let size = field.get_size_in_bits();
            let encoding = field.get_encoding();
            let endianness = field.get_endianness();

            let raw = Self::extract_bits(
                frame,
                offset,
                size,
                endianness
            );

            let value = match encoding {
                Encoding::Unsigned => Value::Unsigned(raw),

                Encoding::Signed => {
                    Value::Signed(Self::sign_extend(raw, size))
                }

                Encoding::Centered(center) => {
                    Value::Centered(raw as i64 - center as i64)
                }

                Encoding::Boolean => Value::Boolean(raw != 0),

                Encoding::Bitmask => Value::Bitmask(raw),
            };

            DecodedField {
                name,
                value,
            }
        });

        Ok(decoded)
    }

    fn extract_bits_little_endian(
        frame: &[u8],
        offset: usize,
        size: usize,
    ) -> u64 {
        let mut result = 0u64;

        for i in 0..size {
            let bit_position = offset + i;
            let byte_index = bit_position / 8;
            let bit_index = bit_position % 8;

            let bit = ((frame[byte_index] >> bit_index) & 1) as u64;

            result |= bit << i;
        }

        result
    }

    fn extract_bits_big_endian(
        frame: &[u8],
        offset: usize,
        size: usize,
    ) -> u64 {
        let mut result = 0u64;

        for i in 0..size {
            let bit_position = offset + i;
            let byte_index = bit_position / 8;
            let bit_index =  7 - (bit_position % 8);

            let bit = ((frame[byte_index] >> bit_index) & 1) as u64;

            result |= bit << i;
        }

        result
    }

    fn extract_bits(
        frame: &[u8],
        offset: usize,
        size: usize,
        endianness: Endianness
    ) -> u64 {
        match endianness {
            Endianness::Little =>
                Self::extract_bits_little_endian(frame, offset, size),
            
            Endianness::Big => 
                Self::extract_bits_big_endian(frame, offset, size),
        }
    }

    /// Interpret an unsigned value as two's-complement signed.
    fn sign_extend(raw: u64, size: usize) -> i64 {
        let shift = 64 - size;
        ((raw << shift) as i64) >> shift
    }
}