use fugit;
use crate::clocks::SYSTICK_FREQUENCY_HZ;

pub type Duration = fugit::Duration<u32, 1, SYSTICK_FREQUENCY_HZ>;