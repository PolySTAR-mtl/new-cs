
use remote::{
    Switch,
    SwitchState,
    JoyStick,
    Wheel,
    Mouse,
    Key,
    KeyState
};

pub enum Input {
    LeftSwitch,
    RightSwitch,
    LeftJoyStick,
    RightJoyStick,
    Wheel,
    Mouse,
    Key(Key)
}