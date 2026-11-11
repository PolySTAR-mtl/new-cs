use embedded_hal::serial::Read;
use rtic::Mutex;
use rtic_monotonics::Monotonic;
use stm32h7xx_hal::nb::Error;
use rtic_monotonics::fugit::ExtU32;

use super::{
    app::{
        input_watchdog,
        handle_remote,
        interrupt_uart5
    },
    Mono,
};

pub fn task_uart5_interrupt(_cx: interrupt_uart5::Context) -> () {
    match handle_remote::spawn() {
        Ok(_) => {
            defmt::info!("handle_remote spawned.")
        },
        Err(_e) => {
            defmt::warn!("handle_remote failed to spawn.");
        }
    }
}

pub async fn task_handle_remote(cx: &mut handle_remote::Context<'_>) -> () {
    loop {
        let result = cx.local.uart5_rx.read();
        match result {
            Ok(byte) => {
                cx.shared.remote.lock(|remote| {
                    let now = Mono::now();
                    let is_frame_full = remote.read(byte, now);
                    if is_frame_full {
                        cx.shared.input_map.lock(|input_map| {
                            input_map.update(remote, now);
                        });
                    }
                });
            },

            Err(Error::WouldBlock) => break,
            
            Err(Error::Other(_error)) => {
                defmt::warn!("UART receive error");
                break;
            },
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