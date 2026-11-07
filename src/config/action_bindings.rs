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
    trigger_data_output::{
        Direction,
        TriggerDataOutput
    },
};

pub const BINDINGS: &'static [Binding] = &[
    Binding {
        trigger: Trigger::Pressed(Button::Key(Key::W)),
        trigger_data_output: TriggerDataOutput::Direction(Direction::Up),
        action: Action::Move,
    },
    Binding {
        trigger: Trigger::Pressed(Button::Key(Key::S)),
        trigger_data_output: TriggerDataOutput::Direction(Direction::Down),
        action: Action::Move,
    },
    Binding {
        trigger: Trigger::DualAxis(DualAxis::LeftStick),
        trigger_data_output: TriggerDataOutput::Default,
        action: Action::Move,
    },
    Binding {
        trigger: Trigger::Axis(Axis::Wheel),
        trigger_data_output: TriggerDataOutput::Default,
        action: Action::Spin,
    },
];