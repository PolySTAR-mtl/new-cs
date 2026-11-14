use crate::{
    protocol::{
        Encoding,
        Endianness,
        Field,
        Protocol,
    },
    units::Bytes,
};

const CENTER: i16 = 1024;

const FRAME_SIZE: usize = 18; // bytes

pub const PROTOCOL_FIELDS_COUNT: usize = 13;
const PROTOCOL_FIELDS: &'static [Field; PROTOCOL_FIELDS_COUNT] = &[
    Field::new(
        "right_joystick_x",
        0,
        11,
        Encoding::Centered(CENTER),
        Endianness::Little
    ),
    Field::new(
        "right_joystick_y",
        11,
        11,
        Encoding::Centered(CENTER),
        Endianness::Little
    ),
    Field::new(
        "left_joystick_x",
        22,
        11,
        Encoding::Centered(CENTER),
        Endianness::Little
    ),
    Field::new(
        "left_joystick_y",
        33,
        11,
        Encoding::Centered(CENTER),
        Endianness::Little
    ),
    Field::new(
        "right_switch",
        44,
        2,
        Encoding::Unsigned,
        Endianness::Little
    ),
    Field::new(
        "left_switch",
        46,
        2,
        Encoding::Unsigned,
        Endianness::Little
    ),
    Field::new(
        "mouse_x",
        48,
        16,
        Encoding::Signed,
        Endianness::Little
    ),
    Field::new(
        "mouse_y",
        64,
        16,
        Encoding::Signed,
        Endianness::Little
    ),
    Field::new(
        "mouse_z",
        80,
        16,
        Encoding::Signed,
        Endianness::Little
    ),
    Field::new(
        "mouse_left_button",
        96,
        8,
        Encoding::Boolean,
        Endianness::Little
    ),
    Field::new(
        "mouse_right_button",
        104,
        8,
        Encoding::Boolean,
        Endianness::Little
    ),
    Field::new(
        "keyboard",
        112,
        16,
        Encoding::Bitmask,
        Endianness::Little
    ),
    Field::new(
        "wheel",
        128,
        16,
        Encoding::Centered(CENTER),
        Endianness::Little
    ),
];

pub enum ProtocolFieldIndex {
    RightJoyStickX = 0,
    RightJoyStickY,
    LeftJoyStickX,
    LeftJoyStickY,
    RightSwitch,
    LeftSwitch,
    MouseX,
    MouseY,
    MouseZ,
    MouseLeftButton,
    MouseRightButton,
    Keyboard,
    Wheel,
}

pub const DR16_PROTOCOL: Protocol<PROTOCOL_FIELDS_COUNT> =
    Protocol::new(PROTOCOL_FIELDS, Bytes::new(FRAME_SIZE));