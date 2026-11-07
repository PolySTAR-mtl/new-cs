#[derive(Clone, Copy, Debug)]
pub enum Axis {
    Wheel = 0,
}

impl Axis {
    pub const COUNT: usize = Self::Wheel as usize + 1;

    pub const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DualAxis {
    LeftStick,
    RightStick,
}

impl DualAxis {
    pub const COUNT: usize = Self::RightStick as usize + 1;
    pub const fn index(self) -> usize {
        Axis::COUNT + self as usize
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Key {
    W = 0,
    S,
    A,
    D,
    SHIFT,
    CTRL,
    Q,
    E,
    R,
    F,
    G,
    Z,
    X,
    C,
    V,
    B
}

impl Key {
    pub const COUNT: usize = Key::B as usize + 1;

    pub const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Button {
    Key(Key),
    MouseLeft,
    MouseRight,
}

impl Button {
    pub const COUNT: usize = Key::COUNT + 2;

    pub const fn index(self) -> usize {
        const OFFSET: usize = Axis::COUNT + DualAxis::COUNT;

        OFFSET + match self {
            Self::Key(key) => key.index(),
            Self::MouseLeft => Key::COUNT,
            Self::MouseRight => Key::COUNT + 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Switch {
    Left,
    Right
}

impl Switch {
    pub const COUNT: usize = Self::Right as usize + 1;

    pub const fn index(self) -> usize {
        const OFFSET: usize = Axis::COUNT + DualAxis::COUNT + Button::COUNT;
        self as usize + OFFSET
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Input {
    Axis(Axis),
    DualAxis(DualAxis),
    Button(Button),
    Switch(Switch),
}

impl Input {
    pub const COUNT: usize = Axis::COUNT
                            + DualAxis::COUNT
                            + Button::COUNT
                            + Switch::COUNT;

    pub const fn index(self) -> usize {
        match self {
            Self::Axis(axis) => axis.index(),
            Self::DualAxis(axis) => axis.index(),
            Self::Button(button) => button.index(),
            Self::Switch(switch) => switch.index(),
        }
    }
}