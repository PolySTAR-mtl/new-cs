use math::vector2::Vector2;

use super::{
    axis_value::AxisValue,
    switch_position::SwitchPosition
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InputValue {
    Boolean(bool),
    Axis(AxisValue),
    DualAxis(Vector2<AxisValue>),
    Position(SwitchPosition)
}