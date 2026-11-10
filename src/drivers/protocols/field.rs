#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Bitmask,
    Boolean,
    Centered(i16),
    Signed,
    Unsigned,
}

#[derive(Debug, Clone, Copy)]
pub struct Field {
    name: &'static str,
    offset: usize,
    size: usize,
    encoding: Encoding,
}

impl Field {
    pub const fn new(name: &'static str, offset: usize, size: usize, encoding: Encoding) -> Self {
        Self { name, offset, size, encoding }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_offset_in_bits(&self) -> usize {
        self.offset
    }

    pub fn get_size_in_bits(&self) -> usize {
        self.size
    }

    pub fn get_encoding(&self) -> Encoding {
        self.encoding
    }
}

pub const CENTER: i16 = 1024;