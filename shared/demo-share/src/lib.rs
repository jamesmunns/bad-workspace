#![cfg_attr(not(feature = "use-std"), no_std)]

pub struct One {
    pub a: u32,
    pub b: u16,
}

#[cfg(feature = "use-std")]
pub struct Two {
    pub x: Vec<u8>,
    pub y: String,
}
