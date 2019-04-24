#![crate_name = "timecode"]
#![crate_type = "lib"]

extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod timecode;
pub mod parser;

pub use crate::timecode::Timecode;
