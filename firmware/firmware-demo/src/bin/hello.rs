#![no_main]
#![no_std]

use firmware_demo as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    firmware_demo::exit()
}
