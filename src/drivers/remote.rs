const REMOTE_BUFFER_SIZE: usize = 18;

pub type TimeMilliseconds = u64;
pub type BufferData = u8;
pub type Wheel = i16;

pub enum Switch {
    LeftSwitch,
    RightSwitch,
}

pub enum SwitchState {
    Unknown,
    Up,
    Middle,
    Down,
}

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

pub enum KeyState {
    Unknown,
    Pressed,
    Released,
    Held, // Consider removing this state if unnecessary, as it may not be needed for the current implementation
}

#[derive(Clone, Copy, Default)]
pub struct JoyStick {
    x: i16,
    y: i16,
}

#[derive(Clone, Copy, Default)]
pub struct Mouse {
    x: i16,
    y: i16,
    z: i16,
    left_button: bool,
    right_button: bool,
}

pub struct Remote<UART> {
    left_joystick: JoyStick,
    right_joystick: JoyStick,
    wheel: Wheel,
    left_switch: SwitchState,
    right_switch: SwitchState,
    mouse: Mouse,
    uart: UART,
    is_connected: bool,
    current_buffer_index: usize,
    buffer: [BufferData; REMOTE_BUFFER_SIZE],
    last_read_time_ms: TimeMilliseconds,
}

impl<UART> Remote<UART>
where
    UART: serial::Read<BufferData>,
{

pub fn new(uart: UART) -> Self {
    Remote {
        left_joystick: JoyStick { x: 0, y: 0 },
        right_joystick: JoyStick { x: 0, y: 0 },
        wheel: 0,
        uart,
        left_switch: SwitchState::Unknown,
        right_switch: SwitchState::Unknown,
        mouse: Mouse {
            x: 0,
            y: 0,
            z: 0,
            left_button: false,
            right_button: false,
        },
        current_buffer_index: 0,
        buffer: [0; REMOTE_BUFFER_SIZE],
        is_connected: false,
        last_read_time_ms: get_time_milliseconds(),
    }
}

pub fn update(&mut self) {
    const READ_BUFFER_SIZE: usize = 1;
    const REMOTE_DISCONNECT_TIMEOUT: TimeMilliseconds = 1000; // 1 second timeout for remote disconnect
    const REMOTE_READ_TIMEOUT: TimeMilliseconds = 500; // 0.5 second timeout for read operation

    let now = get_time_milliseconds();

    if now - self.last_read_time_ms > REMOTE_DISCONNECT_TIMEOUT {
        self.is_connected = false;
        self.reset();
    }

    let data: [BufferData; READ_BUFFER_SIZE] = [0; READ_BUFFER_SIZE];

    while self.uart.read(&mut data).is_ok() && self.current_buffer_index < REMOTE_BUFFER_SIZE {
        match self.uart.read() {
            Ok(byte) => {
                self.buffer[self.current_buffer_index] = byte;
                self.current_buffer_index += 1;
                self.last_read_time_ms = get_time_milliseconds();
            }
            Err(_) => break,
        }
    }

    if self.current_buffer_index > 0
        && get_time_milliseconds() - self.last_read_time_ms > REMOTE_READ_TIMEOUT {
        self.clear_rx_buffer();
    }

    // Parse the buffer if all 18 bytes have been received.
    if self.current_buffer_index >= REMOTE_BUFFER_SIZE {
        self.is_connected = true;
        self.parse_read_buffer();
        self.clear_rx_buffer();
    }
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

pub fn get_switch_state(&self, switch: Switch) -> SwitchState {
    match switch {
        Switch::LeftSwitch => self.left_switch,
        Switch::RightSwitch => self.right_switch,
    }
}

pub fn get_mouse(&self) -> Mouse {
    Mouse {
        x: 0,
        y: 0,
        z: 0,
        left_button: false,
        right_button: false,
    }
}

pub fn get_key_state(&self, key: Key) -> KeyState {
    KeyState::Unknown
}

fn reset(&mut self) {
    self.left_joystick = JoyStick { x: 0, y: 0 };
    self.right_joystick = JoyStick { x: 0, y: 0 };
    self.wheel = 0;
    self.is_connected = false;
    self.last_read_time_ms = get_time_milliseconds();
}

pub fn is_connected(&self) -> bool {
    self.is_connected
}

fn clear_rx_buffer(&mut self) {
    self.current_buffer_index = 0;
    self.buffer = [0; REMOTE_BUFFER_SIZE];

    // Drain bytes currently pending in the UART.
    while self.uart.read().is_ok() {}
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

fn parse_read_buffer(&mut self) {
    // Placeholder implementation, replace with actual parsing logic
    // this is a wonky encoding implemented by our "good pal Li Qingzhi" as stated by a chinese
    // translated document our team acquired a while back; refer to this document for the protocol
    // encoding: https://drive.google.com/file/d/1a5kaTsDvG89KQwy3fkLVkxKaQJfJCsnu/view?usp=sharing

    // // remote joystick information
    // remote.rightHorizontal = (rxBuffer[0] | rxBuffer[1] << 8) & 0x07FF;
    // remote.rightHorizontal -= 1024;
    // remote.rightVertical = (rxBuffer[1] >> 3 | rxBuffer[2] << 5) & 0x07FF;
    // remote.rightVertical -= 1024;
    // remote.leftHorizontal = (rxBuffer[2] >> 6 | rxBuffer[3] << 2 | rxBuffer[4] << 10) & 0x07FF;
    // remote.leftHorizontal -= 1024;
    // remote.leftVertical = (rxBuffer[4] >> 1 | rxBuffer[5] << 7) & 0x07FF;
    // remote.leftVertical -= 1024;

    // remote.leftSwitch = static_cast<Remote::SwitchState>((rxBuffer[5] >> 6) & 0x03);
    // remote.rightSwitch = static_cast<Remote::SwitchState>((rxBuffer[5] >> 4) & 0x03);

    // // mouse input
    // remote.mouse.x = rxBuffer[6] | (rxBuffer[7] << 8);    // x axis
    // remote.mouse.y = rxBuffer[8] | (rxBuffer[9] << 8);    // y axis
    // remote.mouse.z = rxBuffer[10] | (rxBuffer[11] << 8);  // z axis
    // remote.mouse.l = static_cast<bool>(rxBuffer[12]);     // left button click
    // remote.mouse.r = static_cast<bool>(rxBuffer[13]);     // right button click

    // // keyboard capture
    // remote.key = rxBuffer[14] | rxBuffer[15] << 8;
    // // remote wheel
    // remote.wheel = (rxBuffer[16] | rxBuffer[17] << 8) - 1024;

    // // the remote joystick and wheel values must be <= abs(660)
    // if ((abs(remote.rightHorizontal) > ANALOG_MAX_VALUE) ||
    //     (abs(remote.rightVertical) > ANALOG_MAX_VALUE) ||
    //     (abs(remote.leftHorizontal) > ANALOG_MAX_VALUE) ||
    //     (abs(remote.leftVertical) > ANALOG_MAX_VALUE) || (abs(remote.wheel) > ANALOG_MAX_VALUE))
    // {
    //     RAISE_ERROR(drivers, "invalid remote joystick values");
    // }

    // drivers->commandMapper.handleKeyStateChange(
    //     remote.key,
    //     remote.leftSwitch,
    //     remote.rightSwitch,
    //     remote.mouse.l,
    //     remote.mouse.r);

    // remote.updateCounter++;

    self.joystick.left.x = (self.buffer[2] >> 6 | self.buffer[3] << 2 | self.buffer[4] << 10) & 0x07FF - 1024;
    self.joystick.left.y = (self.buffer[4] >> 1 | self.buffer[5] << 7) & 0x07FF - 1024;
    self.joystick.right.x = (self.buffer[0] | self.buffer[1] << 8) & 0x07FF - 1024;
    self.joystick.right.y = (self.buffer[1] >> 3 | self.buffer[2] << 5) & 0x07FF - 1024;

    self.left_switch = match (self.buffer[5] >> 6) & 0x03 {
        0 => SwitchState::Unknown,
        1 => SwitchState::Up,
        2 => SwitchState::Middle,
        3 => SwitchState::Down,
        _ => SwitchState::Unknown,
    };

    self.right_switch = match (self.buffer[5] >> 4) & 0x03 {
        0 => SwitchState::Unknown,
        1 => SwitchState::Up,
        2 => SwitchState::Middle,
        3 => SwitchState::Down,
        _ => SwitchState::Unknown,
    };

    self.mouse.x = self.buffer[6] | (self.buffer[7] << 8);
    self.mouse.y = self.buffer[8] | (self.buffer[9] << 8);
    self.mouse.z = self.buffer[10] | (self.buffer[11] << 8);
    self.mouse.left_button = self.buffer[12] != 0;
    self.mouse.right_button = self.buffer[13] != 0;
    self.key_state = self.buffer[14] | (self.buffer[15] << 8);
    self.wheel = (self.buffer[16] | (self.buffer[17] << 8)) - 1024;

    // TODO: Implement validation logic for joystick and wheel values.

    // TODO: Implement logic to handle key state changes, switch states, and mouse button states.
}

}

/**
 * TODO: Move this method to a timer object or time utility module, as it is not directly related to the Remote struct.
 */
fn get_time_milliseconds() -> TimeMilliseconds {
    // Placeholder implementation, replace with actual time retrieval logic
    0
}