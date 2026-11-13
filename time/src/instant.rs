use fugit;
use crate::clocks::SYSTICK_FREQUENCY_HZ;

pub type Instant = fugit::Instant<u32, 1, SYSTICK_FREQUENCY_HZ>; 