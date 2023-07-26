#![no_main]
#![no_std]

use demo_share::One;
use firmware_demo as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");
    let _x = One { a: 0, b: 0 };

    firmware_demo::exit()
}
