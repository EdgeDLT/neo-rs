#[macro_use]
extern crate failure;

pub extern crate neo_core as neo_core;

#[cfg_attr(tarpaulin, skip)]
pub mod cli;
