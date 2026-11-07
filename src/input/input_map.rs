use crate::{
    config::time::Instant,
    drivers::remote::Remote,
    math::vector2::Vector2,
};

use super::{
    action_state::ActionState,
    action_value::ActionValue,
    axis_value::AxisValue,
    input_state::InputState,
    input_value::InputValue,
    switch_position::SwitchPosition,
    input::{
        Switch,
        Input,
        Key,
        Button,
        DualAxis,
        Axis
    },
    action::Action,
    binding::Binding,
    trigger::Trigger,
    trigger_data_output::{
        Direction,
        TriggerDataOutput,
    }
};

pub struct InputMap {
    bindings: &'static [Binding],
    inputs: [InputState; Input::COUNT],
    actions: [ActionState; Action::COUNT],
}

impl InputMap {
    pub fn new(bindings: &'static [Binding]) -> Self {
        let empty_action = ActionState::new(ActionValue::Boolean(false));
        let empty_input = InputState::new(InputValue::Boolean(false));
        Self {
            bindings,
            inputs: [empty_input; Input::COUNT],
            actions: [empty_action; Action::COUNT],
        }
    }

    pub fn update(&mut self, remote: &Remote, now: Instant) {
        self.update_inputs(remote, now);
        self.update_actions(now);
    }

    pub fn get_action_state(&self, action: Action) -> ActionState {
        self.actions[action.index()]
    }

    fn update_inputs(&mut self, remote: &Remote, now: Instant) {
        // Update keyboard
        const KEYS: [Key; Key::COUNT] = [
            Key::W,
            Key::S,
            Key::A,
            Key::D,
            Key::SHIFT,
            Key::CTRL,
            Key::Q,
            Key::E,
            Key::R,
            Key::F,
            Key::G,
            Key::Z,
            Key::X,
            Key::C,
            Key::V,
            Key::B,
        ];

        for key in KEYS {
            let pressed = remote.is_key_pressed(key).unwrap_or(false);

            self.inputs[
                Input::Button(Button::Key(key)).index()
            ].update(
                InputValue::Boolean(pressed),
                now
            );
        }
        // TODO: Update mouse

        // Update joysticks
        let left_joystick = remote.get_left_joystick();


        self.inputs[
            Input::DualAxis(DualAxis::LeftStick).index()
        ].update(
            InputValue::DualAxis(left_joystick),
            now,
        );

        let right_joystick = remote.get_right_joystick();

        self.inputs[
            Input::DualAxis(DualAxis::RightStick).index()
        ].update(
            InputValue::DualAxis(right_joystick),
            now,
        );

        // Update switches
        let left_switch = remote.get_left_switch().unwrap_or(SwitchPosition::Middle);
        self.inputs[
            Input::Switch(Switch::Left).index()
        ].update(
            InputValue::Position(left_switch),
            now
        );

        let right_switch = remote.get_right_switch().unwrap_or(SwitchPosition::Middle);
        self.inputs[
            Input::Switch(Switch::Right).index()
        ].update(
            InputValue::Position(right_switch),
            now
        );

        // Update wheel
        let wheel = remote.get_wheel();

        self.inputs[
            Input::Axis(Axis::Wheel).index()
        ].update(
            InputValue::Axis(wheel),
            now,
        );
    }

    fn update_actions(&mut self, now: Instant) {
        let mut values = [
            // Aim
            ActionValue::DualAxis(Vector2::new(
                AxisValue::ZERO,
                AxisValue::ZERO,
            )),
            // Move
            ActionValue::DualAxis(Vector2::new(
                AxisValue::ZERO,
                AxisValue::ZERO,
            )),
            // Shoot
            ActionValue::Boolean(false),
            // Feed
            ActionValue::Boolean(false),
            // Spin
            ActionValue::Axis(AxisValue::ZERO),
        ];

        for binding in self.bindings {
            // Evaluate binding and accumulate its value.
            let index = binding.action.index();

            if !binding.trigger.evaluate(&self.inputs, now) {
                continue;
            }

            values[index] = match (values[index], binding.trigger, binding.trigger_data_output) {
                (
                    ActionValue::Boolean(current),
                    _, // Any Trigger
                    TriggerDataOutput::Default,
                ) => {
                    ActionValue::Boolean(current || true )
                },

                (
                    ActionValue::Axis(current),
                    Trigger::Axis(axis),
                    TriggerDataOutput::Default
                ) => {
                    let source = self.inputs[
                        Input::Axis(axis).index()
                    ].value();

                    match source {
                        InputValue::Axis(value) => {
                            ActionValue::Axis(value + current)
                        },
                        _ => ActionValue::Axis(current),
                    }
                },

                (
                    ActionValue::DualAxis(current),
                    Trigger::DualAxis(dual_axis),
                    TriggerDataOutput::Default
                ) => {
                    let source = self.inputs[
                        Input::DualAxis(dual_axis).index()
                    ].value();

                    match source {
                        InputValue::DualAxis(value) => {
                            ActionValue::DualAxis(current + value)
                        },
                        _ => ActionValue::DualAxis(current),
                    }
                },

                (
                    ActionValue::DualAxis(current),
                    Trigger::Pressed(button),
                    TriggerDataOutput::Direction(direction)
                ) => {
                    let is_pressed = self.inputs[
                        Input::Button(button).index()
                    ].is_pressed();

                    if is_pressed {
                        ActionValue::DualAxis(current + match direction {
                            Direction::Up => Vector2::new(AxisValue::ZERO, AxisValue::POSITIVE),
                            Direction::Right => Vector2::new(AxisValue::POSITIVE, AxisValue::ZERO),
                            Direction::Left => Vector2::new(AxisValue::NEGATIVE, AxisValue::ZERO),
                            Direction::Down => Vector2::new(AxisValue::ZERO, AxisValue::NEGATIVE),
                        })
                    } else {
                        ActionValue::DualAxis(current)
                    }
                },

                // Mismatched value types
                (
                    current,
                    _, // Any Trigger
                    _,
                ) => {
                    current
                },
            };
        }

        for value in &mut values {
            value.normalize();
        }

        for (action, value) in self.actions.iter_mut().zip(values) {
            action.update(value, now);
        }
    }
}