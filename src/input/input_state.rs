use super::input_value::InputValue;
use crate::{
    config::time::{
        Duration,
        Instant,
    }, input::switch_position::SwitchPosition, 
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputState {
    value: InputValue,

    is_just_pressed: bool,
    is_just_released: bool,

    pressed_at: Option<Instant>,
}

impl InputState {
    pub fn new(value: InputValue) -> Self {
        Self {
            value,
            is_just_pressed: false,
            is_just_released: false,
            pressed_at: None,
        }
    }

    pub fn update(&mut self, value: InputValue, now: Instant) {
        let was_pressed = self.is_pressed();
        self.value = value;

        if self.is_pressed() && !was_pressed {
            self.is_just_pressed = true;
            self.pressed_at = Some(now);
        } else {
            self.is_just_pressed = false;
        }

        if self.is_released() && was_pressed {
            self.is_just_released = true;
            self.pressed_at = None;
        } else {
            self.is_just_released = false;
        }
    }

    pub fn is_pressed(&self) -> bool {
        match self.value {
            InputValue::Boolean(b) => b,
            InputValue::Axis(a) => a.is_active(),
            InputValue::DualAxis(v) => v.x().is_active() || v.y().is_active(),
            InputValue::Position(_) => false,
        }
    }

    pub fn is_released(&self) -> bool {
        !self.is_pressed()
    }

    pub fn is_just_pressed(&self) -> bool {
        self.is_just_pressed
    }

    pub fn is_just_released(&self) -> bool {
        self.is_just_released
    }

    pub fn is_held(&self) -> bool {
        self.is_pressed() && !self.is_just_pressed
    }

    pub fn held_for(&self, now: Instant) -> Option<Duration> {
        now.checked_duration_since(self.pressed_at?)
    }

    pub fn value(&self) -> InputValue {
        self.value
    }
}