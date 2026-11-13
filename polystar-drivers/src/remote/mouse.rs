// TODO: Convert output in remote to standard axis and button input values.
#[derive(Clone, Copy, Default)]
pub struct Mouse {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub left_button: bool,
    pub right_button: bool,
}
