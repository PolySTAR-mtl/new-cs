use embedded_hal::serial::Read;
use rtic::Mutex;
use rtic_monotonics::{Monotonic};
use stm32h7xx_hal::nb::Error;
use rtic_monotonics::fugit::ExtU32;

use super::{
    app::{
        input,
        input_watchdog,
    },
    Mono,
};

use crate::drivers::remote::TimeMilliseconds;

pub fn task_input(cx: &mut input::Context) -> () {
    loop {
        let result = cx.local.uart5_rx.read();
        match result {
            Ok(byte) => {
                cx.shared.remote.lock(|remote| {
                    remote.read(byte, Mono::now().ticks() as TimeMilliseconds);
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
            remote.check_timeout(Mono::now().ticks() as u64);
            if !remote.is_connected() {
                defmt::println!("Controller was disconnected");
            }
        });
    }
}