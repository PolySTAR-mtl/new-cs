use super::{
    trigger::Trigger,
    action::Action,
};

pub struct Binding {
    pub trigger: Trigger,
    pub action: Action,
}