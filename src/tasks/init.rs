use stm32h7xx_hal::{
    pac,
    prelude::*,
};

use super::app::{
    init,
};

pub struct Hardware {
    pub system_clock_hz: u32,
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
    let _ccdr = reset_and_clock_control
        .use_hse(high_speed_external_oscillator_frequency)
        .sys_ck(system_clock_frequency)
        .freeze(power_config, &device.SYSCFG);

    defmt::println!("Clock configuration complete");

    const SYSTEM_CLOCK_HZ: u32 = 64_000_000;
    // Initialize GPIO, FDCAN, timers, and other drivers here.
    Hardware { 
        system_clock_hz: SYSTEM_CLOCK_HZ,
    }
}

pub fn task_init(cx: init::Context) -> Hardware {
    let hardware = configure_hardware(cx.device);

    super::Mono::start(cx.core.SYST, hardware.system_clock_hz);

    hardware
}