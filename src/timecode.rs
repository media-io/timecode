
use std::string::ToString;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Timecode {
  pub hours: u8,
  pub minutes: u8,
  pub seconds: u8,
  pub frame: u8,
  pub drop_frame: bool,
  pub color_frame: bool,
}

impl ToString for Timecode {
  fn to_string(&self) -> String {
    let separator = match self.drop_frame {
      true => ';',
      false => ':',
    };

    format!(
      "{:02}:{:02}:{:02}{}{:02}",
      self.hours, self.minutes, self.seconds, separator, self.frame
    )
  }
}

impl From<u32> for Timecode {
  fn from(frames: u32) -> Self {
    let fps = 25;

    let hours = frames / (60 * 60 * fps);
    let minutes = (frames / (60 * fps)) - hours * 60;
    let seconds = (frames / fps) - minutes * 60 - hours * 60 * 60;
    let frame = frames - seconds * fps - minutes * 60 - hours * 60 * 60;
    let drop_frame = false;
    let color_frame = false;

    Timecode {
      hours: hours as u8,
      minutes: minutes as u8,
      seconds: seconds as u8,
      frame: frame as u8,
      drop_frame,
      color_frame,
    }
  }
}

#[test]
fn timecode_from_frame() {
  let t1 = Timecode::from(900000);
  assert_eq!(t1.hours, 10);
  assert_eq!(t1.minutes, 0);
  assert_eq!(t1.seconds, 0);
  assert_eq!(t1.frame, 0);

  let t2 = Timecode::from(10);
  assert_eq!(t2.hours, 0);
  assert_eq!(t2.minutes, 0);
  assert_eq!(t2.seconds, 0);
  assert_eq!(t2.frame, 10);

  let t3 = Timecode::from(25);
  assert_eq!(t3.hours, 0);
  assert_eq!(t3.minutes, 0);
  assert_eq!(t3.seconds, 1);
  assert_eq!(t3.frame, 0);
}

