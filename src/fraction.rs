use crate::frame;
use frame::Frame;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub enum Fraction {
  Frame(Frame),
  MilliSeconds(u16),
}

impl ToString for Fraction {
  fn to_string(&self) -> String {
    match self {
      Fraction::Frame(frame) => {
        let separator = if frame.drop_frame() { ';' } else { ':' };

        format!("{}{:02}", separator, frame.frames())
      }
      Fraction::MilliSeconds(milliseconds) => format!(".{:03}", milliseconds),
    }
  }
}
