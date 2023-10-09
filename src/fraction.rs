use super::TimecodeFrames;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub enum Fraction {
  Frames(TimecodeFrames),
  MilliSeconds(u16),
}

impl ToString for Fraction {
  fn to_string(&self) -> String {
    match self {
      Self::Frames(timecode_frame) => timecode_frame.to_string(),
      Self::MilliSeconds(milliseconds) => format!(".{:03}", milliseconds),
    }
  }
}
