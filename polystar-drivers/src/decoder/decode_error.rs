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