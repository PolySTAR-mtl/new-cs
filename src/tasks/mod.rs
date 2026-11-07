use rtic::app;
use rtic_monotonics::systick::prelude::*;

use crate::config::clocks::SYSTICK_FREQUENCY_HZ;

mod control;
mod init;
mod input;
mod idle;
systick_monotonic!(Mono, SYSTICK_FREQUENCY_HZ);

defmt::timestamp!("{=u32}", {
    Mono::now().ticks()
});

#[app(
    device = stm32h7xx_hal::pac,
    peripherals = true,
    dispatchers = [
        ETH,
        ETH_WKUP
    ]
)]
mod app {
    use stm32h7xx_hal::{
        pac,
        serial::Rx
    };
    use crate::{
        drivers::remote::Remote,
        input::input_map::InputMap,
    };

    #[shared]
    struct Shared {
        // Shared state between tasks (if needed)
        // Add action states to decouple the input task from the main control loop
        remote: Remote, // Remote driver
    }

    #[local]
    struct Local {
        // Task-local state
        // Example: wheel_motors: [MotorDriver; 4],
        // Example: timer: Timer<pac::TIM2>,
        // Add subsystems states (data, speed, position, etc.)
        // Add powermanagement states (battery, power supply, etc.)
        uart5_rx: Rx<pac::UART5>, // Remote uart rx buffer
        input_map: InputMap
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let context = super::init::task_init(cx);

        (
            Shared {
                remote: context.hardware.remote
            }, 
            Local {
                uart5_rx: context.hardware.uart5_rx,
                input_map: context.input_map
            },
        )
    }

    #[idle]
    fn idle(cx: idle::Context) -> ! {
        super::idle::task_idle(cx)
    }

    #[task(
        binds = UART5,
        priority = 2,
        shared = [
            remote
        ],
        local = [
            uart5_rx
        ]
    )]
    fn input(mut cx: input::Context) -> () {
        super::input::task_input(&mut cx);
    }

    #[task(
        priority = 1,
        shared = [
            remote
        ],
    )]
    async fn input_watchdog(mut cx: input_watchdog::Context) -> ! {
        super::input::task_input_watchdog(&mut cx).await;
    }

    #[task(
        priority = 1,
        shared = [
            remote
        ],
        local = [
            input_map
        ]
    )]
    async fn control(mut cx: control::Context) -> ! {
        super::control::task_control(&mut cx).await;
    }
}
