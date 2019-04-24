#![crate_name = "timecode"]
#![crate_type = "lib"]

extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod parser;
pub mod timecode;

pub use crate::timecode::Timecode;
