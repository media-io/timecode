#![crate_name = "timecode"]
#![crate_type = "lib"]

extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod timecode;

pub use crate::timecode::Timecode;
pub use crate::timecode::FrameRate2400;
pub use crate::timecode::FrameRate2500;
pub use crate::timecode::FrameRate3000;
pub use crate::timecode::FrameRate5000;
pub use crate::timecode::FrameRate6000;