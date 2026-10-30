use rtic::app;
use rtic_monotonics::systick::prelude::*;

mod init;
mod input;
mod idle;

const SYSTICK_FREQUENCY_HZ: u32 = 1_000;
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
    #[shared]
    struct Shared {
        // Shared state between tasks (if needed)
        // Add action states to decouple the input task from the main control loop
    }

    #[local]
    struct Local {
        // Task-local state
        // Example: wheel_motors: [MotorDriver; 4],
        // Example: timer: Timer<pac::TIM2>,
        remote: u32, // Remote driver
        // Add subsystems states (data, speed, position, etc.)
        // Add powermanagement states (battery, power supply, etc.)
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let _ = super::init::task_init(cx);

        defmt::println!("RTIC init reached");

        (
            Shared {
            }, 
            Local {
                remote: 0,
            },
        )
    }

    #[idle]
    fn idle(cx: idle::Context) -> ! {
        super::idle::task_idle(cx)
    }

    #[task(
        binds = UART4,
        priority = 1,
        shared = [

        ],
        local = [
            remote
        ]
    )]
    fn input(mut cx: input::Context) {
        super::input::task_input(&mut cx);
    }
}
