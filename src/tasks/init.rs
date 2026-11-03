use stm32h7xx_hal::{
    gpio::{NoPin, gpiob}, pac, prelude::*, serial::{Rx, config::Config},
};

use crate::{drivers::remote::Remote, tasks::app::input_watchdog};

use super::app::{
    init,
};

pub struct Hardware {
    pub system_clock_hz: u32,
    pub remote: Remote,
    pub uart5_rx: Rx<pac::UART5>,
}

fn configure_hardware(device: pac::Peripherals) -> Hardware {
    defmt::println!("Initializing STM32H723...");

    // Configure power.
    let power = device.PWR.constrain();
    defmt::println!("Before power freeze");
    let power_config = power.freeze();

    defmt::println!("Power configuration complete");

    // Configure clocks.
    let reset_and_clock_control = device.RCC.constrain();

    let high_speed_external_oscillator_frequency = 25.MHz();
    let system_clock_frequency = 64.MHz();
    let ccdr = reset_and_clock_control
        .use_hse(high_speed_external_oscillator_frequency)
        .sys_ck(system_clock_frequency)
        .freeze(power_config, &device.SYSCFG);

    defmt::println!("Clock configuration complete");

    // Configure GPIO.
    let gpiod = device.GPIOD.split(ccdr.peripheral.GPIOD);

    // let tx_pin = gpiod.pd5.into_alternate::<14>();
    let gpiob = device.GPIOB.split(ccdr.peripheral.GPIOB);
    let tx_pin = gpiob.pb6.into_alternate::<14>();
    let rx_pin = gpiod.pd2.into_alternate();

    // Configure UART5.
    let config = Config::new(100_000.bps())
        .parity_even();

    let uart5 = device.UART5
        .serial(
            (tx_pin, rx_pin),
            config,
            ccdr.peripheral.UART5,
            &ccdr.clocks,
        )
        .unwrap();

    let (_uart5_tx, mut uart5_rx) = uart5.split();

    // Enable receive interrupt.
    uart5_rx.listen();

    const SYSTEM_CLOCK_HZ: u32 = 64_000_000;

    Hardware {
        system_clock_hz: SYSTEM_CLOCK_HZ,
        remote: Remote::new(),
        uart5_rx
    }
}

pub fn task_init(cx: init::Context) -> Hardware {
    let hardware = configure_hardware(cx.device);

    super::Mono::start(cx.core.SYST, hardware.system_clock_hz);

    input_watchdog::spawn().expect("Failed to spawn watchdog");

    hardware
}