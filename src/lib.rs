#![crate_name = "timecode"]
#![crate_type = "lib"]

#[macro_use]
extern crate serde_derive;

mod timecode;

pub use crate::timecode::{
  FrameRate2400, FrameRate2500, FrameRate3000, FrameRate5000, FrameRate6000, Timecode,
};
