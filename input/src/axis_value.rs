use core::ops::{
    Add,
    Sub,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct AxisValue {
    value: f32
}

impl AxisValue {
    pub const TRESHOLD: Self = Self::new(0.02);
    pub const ZERO: Self = Self::new(0.0);
    pub const POSITIVE: Self = Self::new(1.0);
    pub const NEGATIVE: Self = Self::new(-1.0);

    pub const fn new(value: f32) -> Self {
        Self {
            value
        }
    }

    pub const fn clamp(self, min: f32, max: f32) -> Self {
        Self::new(self.value.clamp(min, max))
    }

    pub const fn value(&self) -> f32 {
        self.value
    }

    pub fn is_active(self) -> bool {
        // Consider moving to trigger.
        self.value.abs() > Self::TRESHOLD.value.abs()
    }
}

impl Add for AxisValue {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
}

impl Sub for AxisValue {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
}
