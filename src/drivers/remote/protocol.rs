#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Bitmask,
    Boolean,
    Centered(i16),
    Signed,
    Unsigned,
}

#[derive(Debug, Clone, Copy)]
pub struct Field {
    name: &'static str,
    offset: usize,
    size: usize,
    encoding: Encoding,
}

impl Field {
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_offset_in_bits(&self) -> usize {
        self.offset
    }

    pub fn get_size_in_bits(&self) -> usize {
        self.size
    }

    pub fn get_encoding(&self) -> Encoding {
        self.encoding
    }
}

const CENTER: i16 = 1024;

pub const FRAME_SIZE: usize = 18; // bytes

pub const PROTOCOL_FIELDS: [Field; 13] = [
    Field {
        name: "right_joystick_x",
        offset: 0,
        size: 11,
        encoding: Encoding::Centered(CENTER),
    },
    Field {
        name: "right_joystick_y",
        offset: 11,
        size: 11,
        encoding: Encoding::Centered(CENTER),
    },
    Field {
        name: "left_joystick_x",
        offset: 22,
        size: 11,
        encoding: Encoding::Centered(CENTER),
    },
    Field {
        name: "left_joystick_y",
        offset: 33,
        size: 11,
        encoding: Encoding::Centered(CENTER),
    },
    Field {
        name: "right_switch",
        offset: 44,
        size: 2,
        encoding: Encoding::Unsigned,
    },
    Field {
        name: "left_switch",
        offset: 46,
        size: 2,
        encoding: Encoding::Unsigned,
    },
    Field {
        name: "mouse_x",
        offset: 48,
        size: 16,
        encoding: Encoding::Signed,
    },
    Field {
        name: "mouse_y",
        offset: 64,
        size: 16,
        encoding: Encoding::Signed,
    },
    Field {
        name: "mouse_z",
        offset: 80,
        size: 16,
        encoding: Encoding::Signed,
    },
    Field {
        name: "mouse_left_button",
        offset: 96,
        size: 8,
        encoding: Encoding::Boolean,
    },
    Field {
        name: "mouse_right_button",
        offset: 104,
        size: 8,
        encoding: Encoding::Boolean,
    },
    Field {
        name: "keyboard",
        offset: 112,
        size: 16,
        encoding: Encoding::Bitmask,
    },
    Field {
        name: "wheel",
        offset: 128,
        size: 16,
        encoding: Encoding::Centered(CENTER),
    },
];

// const FRAME_BITS: usize = FRAME_SIZE * 8;

// const _: () = assert!(
//     PROTOCOL_FIELDS
//         .iter()
//         .all(|field| field.get_offset_in_bits() + field.get_size_in_bits() <= FRAME_BITS)
// );