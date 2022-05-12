#![crate_name = "timecode"]
#![crate_type = "lib"]

#[macro_use]
extern crate serde_derive;

mod timecode;

pub use crate::timecode::{FrameRate, Timecode};
