use super::axis_value::AxisValue;
use crate::{input::{action_value::ActionValue::Boolean, switch_position::SwitchPosition}, math::vector2::Vector2};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionValue {
    Boolean(bool),
    Axis(AxisValue),
    DualAxis(Vector2<AxisValue>),
}

impl ActionValue {
    pub fn is_active(&self) -> bool {
        match *self {
            Self::Boolean(value) => value,
            Self::Axis(value)
                => value.is_active(),
            Self::DualAxis(vector)
                => vector.x().is_active() || vector.y().is_active(),
        }
    }

    pub fn normalize(&mut self) -> () {
        match self {
            Self::Axis(value) => {
                *value = value.clamp(-1.0, 1.0);
            },

            Self::DualAxis(value) => {
                *value = value.normalize()
            },
            _ => {},
        };
    }
}