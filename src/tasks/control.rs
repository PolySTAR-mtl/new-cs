use rtic::Mutex;
use rtic_monotonics::{
    Monotonic,
    fugit::ExtU32,
};

use super::{
    app::{
        control,
    },
    Mono,
};

pub async fn task_control(cx: &mut control::Context<'_>) -> ! {
    loop {
        // delay
        const DELAY_MS: u32 = 10;
        Mono::delay(DELAY_MS.millis()).await; // 100Hz
        // parse actions
        cx.shared.remote.lock(|remote| {
            let now = Mono::now();
            cx.local.input_map.update(remote, now);
        });

        // execute current actions
    }
}