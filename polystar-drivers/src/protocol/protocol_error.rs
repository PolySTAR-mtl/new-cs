#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolError {
    InvalidFrameSize {
        size: usize,
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
    OverlappingFields {
        first: &'static str,
        second: &'static str,
    },
}