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

#[derive(Clone, Copy)]
pub enum KeyState {
    Unknown,
    Pressed,
    Released,
}