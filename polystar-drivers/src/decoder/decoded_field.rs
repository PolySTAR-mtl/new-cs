use super::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedField {
    pub name: &'static str,
    pub value: Value,
}