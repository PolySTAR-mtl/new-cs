#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

mod config;
mod drivers;
mod input;
mod math;
mod tasks;