#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
pub mod alloc;

#[cfg(feature = "std")]
mod std;

#[cfg(feature = "std")]
pub use std::*;
