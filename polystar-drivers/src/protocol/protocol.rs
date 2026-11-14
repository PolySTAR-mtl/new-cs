use crate::units::Bytes;

use super::{
    Encoding,
    Field,
    ProtocolError
};

pub struct Protocol<const N: usize> {
    fields: &'static [Field; N],
    frame_size: Bytes,
}

impl<const N: usize> Protocol<N> {
    pub const fn new(fields: &'static [Field; N], frame_size: Bytes) -> Self {
        Self {
            fields,
            frame_size
        }
    }

    pub const fn get_fields(&self) -> &'static [Field] {
        self.fields
    }

    pub const fn get_frame_size(&self) -> Bytes {
        self.frame_size
    }

    pub const fn validate(&self) -> Result<(), ProtocolError> {
        let frame_size = self.frame_size.value();
        let frame_bits = match frame_size.checked_mul(8) {
            Some(bits) if frame_size > 0 => bits,
            _ => {
                return Err(ProtocolError::InvalidFrameSize {
                    size: frame_size,
                });
            }
        };

        let mut i = 0;

        while i < N {
            let field = &self.fields[i];
            let name = field.get_name();
            let offset = field.get_offset_in_bits();
            let size = field.get_size_in_bits();
            let encoding = field.get_encoding();

            if size == 0 || size > 64 {
                return Err(ProtocolError::InvalidFieldSize {
                    name,
                    size,
                });
            }

            let end = match offset.checked_add(size) {
                Some(end) => end,
                None => {
                    return Err(ProtocolError::FieldOutOfBounds {
                        name,
                        offset,
                        size,
                    });
                }
            };

            if end > frame_bits {
                return Err(ProtocolError::FieldOutOfBounds {
                    name,
                    offset,
                    size,
                });
            }

            if let Encoding::Centered(center) = encoding {
                let max = if size == 64 {
                    u64::MAX
                } else {
                    (1u64 << size) - 1
                };

                if center < 0 || center as u64 > max {
                    return Err(ProtocolError::InvalidFieldRange {
                        name,
                    });
                }
            }

            i += 1;
        }

        i = 0;

        while i < N {
            let first = &self.fields[i];

            let first_start: usize = first.get_offset_in_bits();
            let first_end = first_start + first.get_size_in_bits();

            let mut j = i + 1;

            while j < N {
                let second = &self.fields[j];
                let second_start = second.get_offset_in_bits();
                let second_end = second_start + second.get_size_in_bits();

                if first_start < second_end && second_start < first_end {
                    return Err(ProtocolError::OverlappingFields {
                        first: first.get_name(),
                        second: second.get_name(),
                    });
                }

                j += 1;
            }

            i += 1;
        }

        Ok(())
    }
}