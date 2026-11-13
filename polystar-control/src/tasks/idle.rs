use super::app::idle;

pub fn task_idle(_: idle::Context) -> ! {
    loop {
        // Perform background tasks or enter low-power mode
        // For example, you can put the MCU to sleep:
        cortex_m::asm::wfi();
    }
}