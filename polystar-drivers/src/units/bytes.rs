#[derive(Copy, Clone)]
pub struct Bytes(usize);

impl Bytes {
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    pub const fn value(&self) -> usize {
        self.0
    }
}