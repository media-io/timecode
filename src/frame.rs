pub use frame_rate::FrameRate;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Frame {
  frames: u8,
  drop_frame: bool,
  color_frame: bool,
  frame_rate: FrameRate,
}

impl Frame {
  pub fn new(frames: u8, drop_frame: bool, color_frame: bool, frame_rate: FrameRate) -> Self {
    Self {
      frames,
      drop_frame,
      color_frame,
      frame_rate,
    }
  }
  pub fn drop_frame(&self) -> bool {
    self.drop_frame
  }
  pub fn color_frame(&self) -> bool {
    self.color_frame
  }
  pub fn frame_rate(&self) -> FrameRate {
    self.frame_rate
  }
  pub fn frames(&self) -> u8 {
    self.frames
  }
}
