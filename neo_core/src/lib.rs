//! # Model
//!
//! A neo_core for cryptocurrency wallets.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(unused_extern_crates, dead_code)]
#![forbid(unsafe_code)]

#[cfg(not(feature="std"))]
#[allow(unused_imports)]
#[doc(hidden)]
#[macro_use]
extern crate alloc;

#[macro_use]
extern crate failure;

pub mod no_std;

pub mod utilities;
pub use self::utilities::*;

pub mod nep2;
pub use self::nep2::*;

pub mod neo_type;
pub use self::neo_type::*;

pub mod key_pair;
pub use self::key_pair::*;