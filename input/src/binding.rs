use super::{
    trigger::Trigger,
    trigger_data_output::TriggerDataOutput,
    action::Action,
};

pub struct Binding {
    pub trigger: Trigger,
    pub trigger_data_output: TriggerDataOutput,
    pub action: Action,
}