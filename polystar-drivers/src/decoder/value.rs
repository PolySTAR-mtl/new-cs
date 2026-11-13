#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Unsigned(u64),
    Signed(i64),
    Centered(i64),
    Boolean(bool),
    Bitmask(u64),
}