use math::vector2::Vector2;

use time::Instant;

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
    actions: [ActionState; Action::COUNT],
}

impl InputMap {
    pub fn new(bindings: &'static [Binding]) -> Self {
        let empty_action = ActionState::new(ActionValue::Boolean(false));
        Self {
            bindings,
            actions: [empty_action; Action::COUNT],
        }
    }

    pub fn update(&mut self, inputs: &[InputState; Input::COUNT], now: Instant) {
        self.update_actions(inputs, now);
    }

    pub fn get_action_state(&self, action: Action) -> ActionState {
        self.actions[action.index()]
    }

    fn update_actions(&mut self, inputs: &[InputState; Input::COUNT], now: Instant) {
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

            if !binding.trigger.evaluate(inputs, now) {
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
                    let source = inputs[
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
                    let source = inputs[
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
                    let is_pressed = inputs[
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