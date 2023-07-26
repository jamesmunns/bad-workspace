#![cfg_attr(not(feature = "use-std"), no_std)]

#[cfg(feature = "ex-a")]
pub const LOL: u32 = 0;

#[cfg(feature = "ex-b")]
pub const LOL: u16 = 1;

pub struct One {
    pub a: u32,
    pub b: u16,
}

#[cfg(feature = "use-std")]
pub struct Two {
    pub x: Vec<u8>,
    pub y: String,
}
