use super::protocol::{
    PROTOCOL_FIELDS,
    FRAME_SIZE,
    Encoding,
    Field,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Unsigned(u64),
    Signed(i64),
    Centered(i64),
    Boolean(bool),
    Bitmask(u64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedField {
    pub name: &'static str,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    InvalidFrameSize {
        size: usize,
    },
    InvalidFrameLength {
        expected: usize,
        actual: usize,
    },
    InvalidFieldSize {
        name: &'static str,
        size: usize,
    },
    FieldOutOfBounds {
        name: &'static str,
        offset: usize,
        size: usize,
    },
    InvalidFieldRange {
        name: &'static str,
    },
}

pub struct Decoder<const N: usize> {
    fields: &'static [Field; N],
    frame_size: usize, // in bytes
}

impl<const N: usize> Decoder<N> {
    pub fn new(
        fields: &'static [Field; N],
        frame_size: usize,
    ) -> Result<Self, DecodeError> {
        let decoder = Self {
            fields,
            frame_size,
        };

        decoder.validate()?;

        Ok(decoder)
    }

    /// Validate protocol field definitions.
    pub fn validate(&self) -> Result<(), DecodeError> {
        let frame_bits = self.frame_size
            .checked_mul(8)
            .filter(|_| self.frame_size > 0)
            .ok_or(DecodeError::InvalidFrameSize {
                size: self.frame_size,
            })?;

        for field in self.fields {
            let name = field.get_name();
            let offset = field.get_offset_in_bits();
            let size = field.get_size_in_bits();
            let encoding = field.get_encoding();

            if size == 0 || size > 64 {
                return Err(DecodeError::InvalidFieldSize {
                    name,
                    size,
                });
            }

            let end = offset.checked_add(size);

            if end.is_none_or(|end| end > frame_bits) {
                return Err(DecodeError::FieldOutOfBounds {
                    name,
                    offset,
                    size,
                });
            }

            if let Encoding::Centered(center) = encoding {
                if center < 0 || center as u64 >= (1u64 << size) {
                    return Err(DecodeError::InvalidFieldRange {
                        name,
                    });
                }
            }
        }

        Ok(())
    }

    /// Decode a complete frame into a fixed-size array.
    pub fn decode(
        &self,
        frame: &[u8],
    ) -> Result<[DecodedField; N], DecodeError> {
        if frame.len() != self.frame_size {
            return Err(DecodeError::InvalidFrameLength {
                expected: self.frame_size,
                actual: frame.len(),
            });
        }

        let decoded = core::array::from_fn(|i| {
            let field = &self.fields[i];
            let name = field.get_name();
            let offset = field.get_offset_in_bits();
            let size = field.get_size_in_bits();
            let encoding = field.get_encoding();

            let raw = Self::extract_bits(
                frame,
                offset,
                size,
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

    /// Extract bits using little-endian bit ordering.
    fn extract_bits(
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

    /// Interpret an unsigned value as two's-complement signed.
    fn sign_extend(raw: u64, size: usize) -> i64 {
        let shift = 64 - size;
        ((raw << shift) as i64) >> shift
    }
}

/// DJI DR16 decoder type.
pub type Dr16Decoder = Decoder<13>;

/// Construct a DJI DR16 decoder.
pub fn create_dr16_decoder() -> Dr16Decoder {
    // Will crash if the decoder is invalid.
    Decoder::new(&PROTOCOL_FIELDS, FRAME_SIZE).expect("Invalid dr16 decoder")
}