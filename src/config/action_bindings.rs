use crate::input::{
    action::Action,
    binding::Binding,
    input::{
        Axis,
        Button,
        DualAxis,
        Key,
    },
    trigger::Trigger,
};

pub const BINDINGS: &'static [Binding] = &[
    Binding {
        trigger: Trigger::Pressed(Button::Key(Key::W)),
        action: Action::Move,
    },
    Binding {
        trigger: Trigger::Pressed(Button::Key(Key::S)),
        action: Action::Move,
    },
    Binding {
        trigger: Trigger::DualAxis(DualAxis::LeftStick),
        action: Action::Move,
    },
    Binding {
        trigger: Trigger::Axis(Axis::Wheel),
        action: Action::Spin,
    },
];