#[derive(Clone, Copy, Debug)]
pub enum Action {
    Aim = 0,
    Move,
    Shoot,
    Feed,
    Spin,
}

impl Action {
    pub const COUNT: usize = 5;

    #[inline]
    pub const fn index(self) -> usize {
        self as usize
    }
}