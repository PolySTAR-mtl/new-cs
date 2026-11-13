#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TriggerDataOutput {
    Default,
    Direction(Direction)
}