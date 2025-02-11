#![cfg_attr(not(feature = "std"), no_std)]

mod vec;

pub mod collections {
    pub use crate::vec::Vec;
}
