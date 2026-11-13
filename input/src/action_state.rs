use time::{
    Duration,
    Instant,
};

use super::action_value::ActionValue;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActionState {
    value: ActionValue,

    activated_at: Option<Instant>,
}

impl ActionState {
    pub fn new(value: ActionValue) -> Self {
        Self {
            value,
            activated_at: None,
        }
    }

    pub fn update(&mut self, value: ActionValue, now: Instant) {
        let was_active = self.is_active();

        self.value = value;
        let is_active = self.is_active();

        if !was_active && is_active {
            self.activated_at = Some(now);
        }

        if was_active && !is_active {
            self.activated_at = None
        }
    }

    pub fn is_active(&self) -> bool {
        self.value.is_active()
    }

    pub fn value(&self) -> ActionValue {
        self.value
    }

    pub fn active_for(&self, now: Instant) -> Option<Duration> {
        now.checked_duration_since(self.activated_at?)
    }
}