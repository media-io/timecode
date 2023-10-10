use super::TimecodeFrames;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub enum Fraction {
  Frames(TimecodeFrames),
  MilliSeconds(u16),
}

impl Fraction {
  pub fn number_of_digits(&self) -> usize {
    match self {
      Self::Frames(timecode_frame) => timecode_frame.number_of_digits(),
      Self::MilliSeconds(_) => 3,
    }
  }

  pub fn separator(&self) -> char {
    match self {
      Self::Frames(timecode_frame) => timecode_frame.separator(),
      Self::MilliSeconds(_) => '.',
    }
  }
}

impl ToString for Fraction {
  fn to_string(&self) -> String {
    match self {
      Self::Frames(timecode_frame) => timecode_frame.to_string(),
      Self::MilliSeconds(milliseconds) => format!(".{:03}", milliseconds),
    }
  }
}
