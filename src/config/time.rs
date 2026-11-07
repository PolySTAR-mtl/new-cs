use rtic_monotonics::fugit;

use super::clocks::SYSTICK_FREQUENCY_HZ;

pub type Instant = fugit::Instant<u32, 1, SYSTICK_FREQUENCY_HZ>; 
pub type Duration = fugit::Duration<u32, 1, SYSTICK_FREQUENCY_HZ>;
