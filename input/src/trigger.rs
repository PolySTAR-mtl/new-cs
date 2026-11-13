use time::Instant;

use super::{
    input::{
        Axis,
        Button,
        DualAxis,
    },
    switch_position::SwitchPosition,
    input_state::InputState,
    input::{
        Input,
        Switch,
    },
    input_value::InputValue
};


#[derive(Clone, Copy, Debug)]
pub enum Trigger {
    Pressed(Button),
    Released(Button),
    Held(Button),
    JustPressed(Button),
    JustReleased(Button),
    SwitchAt{
        switch: Switch,
        position: SwitchPosition,
    },
    Axis(Axis),
    DualAxis(DualAxis),
}


impl Trigger {
    pub fn evaluate(
        &self,
        inputs: &[InputState],
        now: Instant,
    ) -> bool {
        // Evaluate trigger against input states.
        match *self {
            Trigger::Pressed(button) => {
                let index = Input::Button(button).index();
                inputs[index].is_pressed()
            },

            Trigger::Released(button) => {
                let index = Input::Button(button).index();
                inputs[index].is_released()
            },

            Trigger::Held(button) => {
                let index = Input::Button(button).index();
                inputs[index].is_held()
            },

            Trigger::JustPressed(button) => {
                let index = Input::Button(button).index();
                inputs[index].is_just_pressed()
            },

            Trigger::JustReleased(button) => {
                let index = Input::Button(button).index();
                inputs[index].is_just_released()
            },

            Trigger::SwitchAt {
                switch,
                position
            } => {
                let index = Input::Switch(switch).index();
                let current = inputs[index].value();

                match current {
                    InputValue::Position(pos) => pos == position,
                    _ => false,
                }
            },

            Trigger::Axis(axis) => {
                let index = Input::Axis(axis).index();
                inputs[index].is_pressed()
            },

            Trigger::DualAxis(dual_axis) => {
                let index = Input::DualAxis(dual_axis).index();
                inputs[index].is_pressed()
            },
        }
    }
}