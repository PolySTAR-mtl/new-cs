use core::{
    ops::{
        Add,
        Sub,
    },
};

use crate::input::axis_value::AxisValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2<T> {
    x: T,
    y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }
}

impl<T> Add for Vector2<T>
where T: Add<Output = T>{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Vector2<T>
where T: Sub<Output = T>{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Vector2<AxisValue> {
    pub fn magnitude(&self) -> f32 {
        let x = self.x.value();
        let y = self.y.value();

        libm::hypotf(x, y)
    }


    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();

        if magnitude == 0.0 {
            return Self::new(AxisValue::ZERO, AxisValue::ZERO);
        }

        Self::new(
            AxisValue::new(self.x.value() / magnitude),
            AxisValue::new(self.y.value() / magnitude),
        )
    }
}