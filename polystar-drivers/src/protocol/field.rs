use super::{
    Encoding,
    Endianness,
};

#[derive(Debug, Clone, Copy)]
pub struct Field {
    name: &'static str,
    offset: usize,
    size: usize,
    encoding: Encoding,
    endianness: Endianness
}

impl Field {
    pub const fn new(name: &'static str, offset: usize, size: usize, encoding: Encoding, endianness: Endianness) -> Self {
        Self {
            name,
            offset,
            size,
            encoding,
            endianness
        }
    }

    pub const fn get_name(&self) -> &'static str {
        self.name
    }

    pub const fn get_offset_in_bits(&self) -> usize {
        self.offset
    }

    pub const fn get_size_in_bits(&self) -> usize {
        self.size
    }

    pub const fn get_encoding(&self) -> Encoding {
        self.encoding
    }

    pub const fn get_endianness(&self) -> Endianness {
        self.endianness
    }
}