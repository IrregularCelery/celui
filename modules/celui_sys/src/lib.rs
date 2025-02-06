#![cfg_attr(not(feature = "std"), no_std)]

pub mod alloc;

#[cfg(feature = "std")]
mod std;
