pub mod mouse;
mod dr16_protocol;
mod dr16_fields;

use crate::{
    config::time::{
        Duration,
        Instant,
    }, 
    input::{
        axis_value::AxisValue,
        input::Key,
        switch_position::SwitchPosition,
    },
    math::vector2::Vector2,
    drivers::{
        protocols::{
            decoder::Value,
            protocol_codec::DecoderProtocol,
        },
        remote::dr16_fields::ProtocolFieldIndex,
    }
};

use mouse::Mouse;
use dr16_protocol::Dr16Protocol;


const REMOTE_BUFFER_SIZE: usize = 18;
const ANALOG_MAX_VALUE: u16 = 660;

type BufferData = u8;
type Joystick = Vector2<AxisValue>;
type Wheel = AxisValue;


enum RemoteReadError {
    InvalidData,
}

pub struct Remote {
    left_joystick: Joystick,
    right_joystick: Joystick,
    wheel: Wheel,
    left_switch: Option<SwitchPosition>,
    right_switch: Option<SwitchPosition>,
    mouse: Mouse,
    key_bits: u16,
    is_connected: bool,
    current_buffer_index: usize,
    buffer: [BufferData; REMOTE_BUFFER_SIZE],
    last_read_time: Instant,
    protocol: Dr16Protocol,
}

impl Remote {

pub fn new() -> Self {
    Remote {
        left_joystick: Joystick::new(AxisValue::ZERO, AxisValue::ZERO),
        right_joystick: Joystick::new(AxisValue::ZERO, AxisValue::ZERO),
        wheel: AxisValue::ZERO,
        left_switch: None,
        right_switch: None,
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
        last_read_time: Instant::from_ticks(0),
        protocol: Dr16Protocol::new().expect("Échec de l'initialisation du protocole DR16")
    }
}

pub fn read(&mut self, byte: u8, now: Instant) -> () {
    const INTER_BYTE_GAP: Duration = Duration::from_ticks(200);
    let gap_detected = now
        .checked_duration_since(self.last_read_time)
        .map_or(false, |gap| gap >= INTER_BYTE_GAP);

    if gap_detected {
        self.clear_rx_buffer();
    }

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

pub fn get_left_switch(&self) -> Option<SwitchPosition> {
    self.left_switch
}

pub fn get_right_switch(&self) -> Option<SwitchPosition> {
    self.right_switch
}


pub fn get_left_joystick(&self) -> Joystick {
    self.left_joystick
}

pub fn get_right_joystick(&self) -> Joystick {
    self.right_joystick
}

pub fn get_wheel(&self) -> Wheel {
    self.wheel
}

pub fn get_mouse(&self) -> Mouse {
    self.mouse
}

pub fn is_key_pressed(&self, key: Key) -> Option<bool> {
    if !self.is_connected {
        return None;
    }
    let key_index = key as u8;
    let mask = 1u16 << key_index;

    if self.key_bits & mask != 0 {
        Some(true)
    } else {
        Some(false)
    }
}

pub fn is_connected(&self) -> bool {
    self.is_connected
}

pub fn check_timeout(&mut self, now: Instant) -> () {
    const REMOTE_DISCONNECT_TIMEOUT: Duration = Duration::from_ticks(1000);
    let stale = now
        .checked_duration_since(self.last_read_time)
        .map_or(false, |elapsed| elapsed > REMOTE_DISCONNECT_TIMEOUT);
    if stale {
        self.reset();
    }
}

fn reset(&mut self) {
    self.left_joystick = Joystick::new(AxisValue::ZERO, AxisValue::ZERO);
    self.right_joystick = Joystick::new(AxisValue::ZERO, AxisValue::ZERO);
    self.wheel = AxisValue::ZERO;
    self.left_switch = None;
    self.right_switch = None;
    self.mouse = Mouse::default();
    self.key_bits = 0;

    self.is_connected = false;

    self.clear_rx_buffer();
}

fn clear_rx_buffer(&mut self) {
    self.current_buffer_index = 0;
    self.buffer = [0; REMOTE_BUFFER_SIZE];
}

fn byte_to_switch_state(value: u32) -> Option<SwitchPosition> {
    match value {
        0 => None,
        1 => Some(SwitchPosition::Up),
        2 => Some(SwitchPosition::Middle),
        3 => Some(SwitchPosition::Down),
        _ => None,
    }
}

fn parse_read_buffer(&mut self) -> Result<(), RemoteReadError> {
    // Decode the 18-byte DBUS frame.
    let decoded = self.protocol
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
    let get_switch = |index: usize| -> Result<Option<SwitchPosition>, RemoteReadError> {
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
    self.right_joystick = Joystick::new(
        AxisValue::new(right_x as f32 / ANALOG_MAX_VALUE as f32),
        AxisValue::new(right_y as f32 / ANALOG_MAX_VALUE as f32),
    );

    defmt::println!("Right Joystick x:{}, y:{}", right_x, right_y);

    self.left_joystick = Joystick::new(
        AxisValue::new(left_x as f32 / ANALOG_MAX_VALUE as f32),
        AxisValue::new(left_y as f32 / ANALOG_MAX_VALUE as f32),
    );

    defmt::println!("Left Joystick x:{}, y:{}", left_x, left_y);

    self.right_switch = right_switch;
    self.left_switch = left_switch;

    let switch_to_u32 = |state: Option<SwitchPosition>| -> u32 {
        if let Some(state) = state {
            return match state {
                SwitchPosition::Up => 1,
                SwitchPosition::Middle => 2,
                SwitchPosition::Down => 3,
            };
        }
        0
    };

    defmt::println!("Right switch: {}", switch_to_u32(right_switch));
    defmt::println!("Left switch: {}", switch_to_u32(left_switch));

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

    self.wheel = AxisValue::new(wheel as f32 / ANALOG_MAX_VALUE as f32);

    defmt::println!("Wheel: {}", wheel);

    Ok(())
}

}

fn validate_analog(value: i16) -> bool {
    value.unsigned_abs() <= ANALOG_MAX_VALUE
}