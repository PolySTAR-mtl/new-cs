use embedded_hal::serial::Read;
use input::{action::Action, action_value::ActionValue};
use rtic::Mutex;
use rtic_monotonics::Monotonic;
use stm32h7xx_hal::nb::Error;
use rtic_monotonics::fugit::ExtU32;

use math::vector2;

use super::{
    app::{
        input_watchdog,
        handle_remote,
        interrupt_uart5
    },
    Mono,
};

pub fn task_uart5_interrupt(cx: &mut interrupt_uart5::Context) -> () {
    loop {
        let result = cx.local.uart5_rx.read();
        match result {
            Ok(byte) => {
                if let Err(_error) = cx.local.uart5_sender.try_send(byte) {
                    defmt::warn!("UART RX channel full");
                }
            },

            Err(Error::WouldBlock) => break,
            
            Err(Error::Other(_error)) => {
                defmt::warn!("UART receive error");
                break;
            },
        }
    }
}

pub async fn task_handle_remote(cx: &mut handle_remote::Context<'_>) -> ! {
    loop {
        let result = cx.local.uart5_receiver.recv().await;

        match result {
            Ok(byte) => {
                let now = Mono::now();

                let is_frame_ready = cx.shared.remote.lock(|remote| {
                    remote.read(byte, now)
                });

                if !is_frame_ready {
                    continue;
                }

                cx.shared.remote.lock(|remote| {
                    let now = Mono::now();
                    if remote.parse_frame().is_ok() {
                        remote.update_inputs(now);
                        cx.shared.input_map.lock(|input_map| {
                            input_map.update(remote.get_inputs(), now);
                        });
                    }
                });
            },

            Err(_) => continue,
        }
    }
}

pub async fn task_input_watchdog(cx: &mut input_watchdog::Context<'_>) -> ! {
    loop {
        Mono::delay(1000.millis()).await;
        cx.shared.remote.lock(|remote| {
            let now = Mono::now();
            remote.check_timeout(now);
            if !remote.is_connected() {
                defmt::println!("Controller was disconnected");
            }
        });
    }
}