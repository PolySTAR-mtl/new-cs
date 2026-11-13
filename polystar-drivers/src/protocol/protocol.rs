use crate::units::Bytes;

use super::Field;

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
}