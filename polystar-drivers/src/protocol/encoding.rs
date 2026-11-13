#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Bitmask,
    Boolean,
    Centered(i16),
    Signed,
    Unsigned,
}