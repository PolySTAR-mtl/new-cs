#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    InvalidFrameLength {
        expected: usize,
        actual: usize,
    },
}