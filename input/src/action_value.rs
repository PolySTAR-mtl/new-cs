use math::vector2::Vector2;
use super::axis_value::AxisValue;


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
                let vector2 = Vector2::new(value.x().value(), value.y().value());
                if vector2.magnitude() > 1.0 {
                    let normalized = vector2.normalize();
                    *value = Vector2::new(AxisValue::new(*normalized.x()), AxisValue::new(*normalized.y()))
                }
            },
            _ => {},
        };
    }
}