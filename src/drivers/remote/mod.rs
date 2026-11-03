mod decoder;
pub mod joystick;
pub mod keyboard;
pub mod switch;
pub mod wheel;
pub mod mouse;
mod protocol;

use decoder::{Dr16Decoder, Value, create_dr16_decoder};
use joystick::JoyStick;
use keyboard::{
    Key,
    KeyState,
};
use switch::SwitchState;
use wheel::Wheel;
use mouse::Mouse;

use crate::drivers::remote::protocol::ProtocolFieldIndex;

const REMOTE_BUFFER_SIZE: usize = 18;

type BufferData = u8;
pub type TimeMilliseconds = u64;

enum RemoteReadError {
    InvalidData,
}

pub struct Remote {
    left_joystick: JoyStick,
    right_joystick: JoyStick,
    wheel: Wheel,
    left_switch: SwitchState,
    right_switch: SwitchState,
    mouse: Mouse,
    key_bits: u16,
    is_connected: bool,
    current_buffer_index: usize,
    buffer: [BufferData; REMOTE_BUFFER_SIZE],
    last_read_time: TimeMilliseconds,
    decoder: Dr16Decoder,
}

impl Remote {

pub fn new() -> Self {
    Remote {
        left_joystick: JoyStick { x: 0, y: 0 },
        right_joystick: JoyStick { x: 0, y: 0 },
        wheel: 0,
        left_switch: SwitchState::Unknown,
        right_switch: SwitchState::Unknown,
        mouse: Mouse {
            x: 0,
            y: 0,
            z: 0,
            left_button: false,
            right_button: false,
        },
        key_bits: 0,
        current_buffer_index: 0,
        buffer: [0; REMOTE_BUFFER_SIZE],
        is_connected: false,
        last_read_time: 0,
        decoder: create_dr16_decoder()
    }
}

pub fn read(&mut self, byte: u8, now: TimeMilliseconds) -> () {
    self.buffer[self.current_buffer_index] = byte;
    self.current_buffer_index += 1;
    self.last_read_time = now;

    // Parse the buffer if all 18 bytes have been received.
    if self.current_buffer_index >= REMOTE_BUFFER_SIZE {
        if self.parse_read_buffer().is_err() {
            defmt::println!("Failed to parse remote buffer");
            self.reset();
        } else {
            self.is_connected = true;
        }
        self.clear_rx_buffer();
    }
}

pub fn get_left_switch(&self) -> SwitchState {
    self.left_switch
}

pub fn get_right_switch(&self) -> SwitchState {
    self.right_switch
}


pub fn get_left_joystick(&self) -> JoyStick {
    self.left_joystick
}

pub fn get_right_joystick(&self) -> JoyStick {
    self.right_joystick
}

pub fn get_wheel(&self) -> Wheel {
    self.wheel
}

pub fn get_mouse(&self) -> Mouse {
    self.mouse
}

pub fn get_key_state(&self, key: Key) -> KeyState {
    if !self.is_connected {
        return KeyState::Unknown;
    }
    let key_index = key as u8;
    let mask = 1u16 << key_index;

    if self.key_bits & mask != 0 {
        KeyState::Pressed
    } else {
        KeyState::Released
    }
}

pub fn is_connected(&self) -> bool {
    self.is_connected
}

pub fn check_timeout(&mut self, now: TimeMilliseconds) -> () {
    const REMOTE_DISCONNECT_TIMEOUT: TimeMilliseconds = 1000; // 1 second timeout for remote disconnect
    if now - self.last_read_time > REMOTE_DISCONNECT_TIMEOUT {
        self.reset();
    }
}

fn reset(&mut self) {
    self.left_joystick = JoyStick { x: 0, y: 0 };
    self.right_joystick = JoyStick { x: 0, y: 0 };
    self.wheel = 0;
    self.left_switch = SwitchState::Unknown;
    self.right_switch = SwitchState::Unknown;
    self.mouse = Mouse::default();
    self.key_bits = 0;

    self.is_connected = false;

    self.clear_rx_buffer();
}

fn clear_rx_buffer(&mut self) {
    self.current_buffer_index = 0;
    self.buffer = [0; REMOTE_BUFFER_SIZE];
}

fn byte_to_switch_state(value: u32) -> SwitchState {
    match value {
        0 => SwitchState::Unknown,
        1 => SwitchState::Up,
        2 => SwitchState::Middle,
        3 => SwitchState::Down,
        _ => SwitchState::Unknown,
    }
}

fn parse_read_buffer(&mut self) -> Result<(), RemoteReadError> {
    // Decode the 18-byte DBUS frame.
    let decoded = self.decoder
        .decode(&self.buffer)
        .map_err(|_| RemoteReadError::InvalidData)?;

    // Extract joystick and wheel values.
    let get_centered = |index: usize| -> Result<i16, RemoteReadError> {
        match decoded[index].value {
            Value::Centered(value) => {
                i16::try_from(value).map_err(|_| RemoteReadError::InvalidData)
            }
            _ => Err(RemoteReadError::InvalidData),
        }
    };

    let right_x = get_centered(ProtocolFieldIndex::RightJoyStickX as usize)?;
    let right_y = get_centered(ProtocolFieldIndex::RightJoyStickY as usize)?;
    let left_x = get_centered(ProtocolFieldIndex::LeftJoyStickX as usize)?;
    let left_y = get_centered(ProtocolFieldIndex::LeftJoyStickY as usize)?;

    let wheel = get_centered(ProtocolFieldIndex::Wheel as usize)?;

    // Extract switches.
    let get_switch = |index: usize| -> Result<SwitchState, RemoteReadError> {
        match decoded[index].value {
            Value::Unsigned(value) => {
                let value = u32::try_from(value)
                    .map_err(|_| RemoteReadError::InvalidData)?;

                Ok(Self::byte_to_switch_state(value))
            }
            _ => Err(RemoteReadError::InvalidData),
        }
    };

    let right_switch = get_switch(ProtocolFieldIndex::RightSwitch as usize)?;
    let left_switch = get_switch(ProtocolFieldIndex::LeftSwitch as usize)?;

    // Extract mouse information.
    let get_signed = |index: usize| -> Result<i16, RemoteReadError> {
        match decoded[index].value {
            Value::Signed(value) => {
                i16::try_from(value).map_err(|_| RemoteReadError::InvalidData)
            }
            _ => Err(RemoteReadError::InvalidData),
        }
    };

    let mouse_x = get_signed(ProtocolFieldIndex::MouseX as usize)?;
    let mouse_y = get_signed(ProtocolFieldIndex::MouseY as usize)?;
    let mouse_z = get_signed(ProtocolFieldIndex::MouseZ as usize)?;

    // Extract mouse buttons.
    let get_boolean = |index: usize| -> Result<bool, RemoteReadError> {
        match decoded[index].value {
            Value::Boolean(value) => Ok(value),
            _ => Err(RemoteReadError::InvalidData),
        }
    };

    let mouse_left = get_boolean(ProtocolFieldIndex::MouseLeftButton as usize)?;
    let mouse_right = get_boolean(ProtocolFieldIndex::MouseRightButton as usize)?;

    // Extract keyboard bitmask.
    let key_bits = match decoded[ProtocolFieldIndex::Keyboard as usize].value {
        Value::Bitmask(value) => {
            u16::try_from(value).map_err(|_| RemoteReadError::InvalidData)?
        }
        _ => return Err(RemoteReadError::InvalidData),
    };

    // Validate analog values.
    if !validate_analog(right_x)
        || !validate_analog(right_y)
        || !validate_analog(left_x)
        || !validate_analog(left_y)
        || !validate_analog(wheel)
    {
        return Err(RemoteReadError::InvalidData);
    }

    // Commit parsed values.
    self.right_joystick = JoyStick {
        x: right_x,
        y: right_y,
    };

    defmt::println!("Right Joystick x:{}, y:{}", right_x, right_y);

    self.left_joystick = JoyStick {
        x: left_x,
        y: left_y,
    };

    defmt::println!("Left Joystick x:{}, y:{}", left_x, left_y);

    self.right_switch = right_switch;
    self.left_switch = left_switch;

    defmt::println!("Right switch: {}", right_switch as u32);
    defmt::println!("Left switch: {}", left_switch as u32);

    self.mouse = Mouse {
        x: mouse_x,
        y: mouse_y,
        z: mouse_z,
        left_button: mouse_left,
        right_button: mouse_right,
    };

    defmt::println!("Mouse x: {}, y: {}, z: {}, left_button: {}, right_button: {}",
        mouse_x,
        mouse_y,
        mouse_z,
        mouse_left,
        mouse_right,
    );

    self.key_bits = key_bits;

    defmt::println!("Key bits: {}", key_bits);

    self.wheel = wheel;

    defmt::println!("Wheel: {}", wheel);

    Ok(())
}

}

fn validate_analog(value: i16) -> bool {
    const ANALOG_MAX_VALUE: u16 = 660;
    value.unsigned_abs() <= ANALOG_MAX_VALUE
}