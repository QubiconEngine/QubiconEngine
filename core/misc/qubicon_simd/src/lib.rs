#![cfg_attr(not(test), no_std)]

pub use _impl::*;

#[cfg_attr(target_arch = "x86_64", path = "x86_64/mod.rs")]
mod _impl;