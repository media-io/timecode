pub use frame_rate::FrameRate;
use num_traits::cast::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::{string::ToString, time::Duration};
use Fraction::Frame;
use Fraction::MilliSeconds;

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Timecode {
  hours: u8,
  minutes: u8,
  seconds: u8,
  fraction: Fraction,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub enum Fraction {
  Frame {
    frames: u8,
    drop_frame: bool,
    color_frame: bool,
    frame_rate: FrameRate,
  },
  MilliSeconds(u16),
}

impl ToString for Timecode {
  fn to_string(&self) -> String {
    let fraction = match self.fraction {
      Frame {
        frames, drop_frame, ..
      } => {
        let separator = if drop_frame { ';' } else { ':' };

        format!("{}{:02}", separator, frames)
      }
      MilliSeconds(milliseconds) => format!(".{:03}", milliseconds),
    };

    format!(
      "{:02}:{:02}:{:02}{}",
      self.hours, self.minutes, self.seconds, fraction
    )
  }
}

impl From<Duration> for Timecode {
  fn from(duration: Duration) -> Self {
    let remaining = duration.as_secs();
    let hours = remaining / 3600;
    let remaining = remaining % 3600;
    let minutes = remaining / 60;
    let seconds = remaining % 60;
    let ms = duration.as_millis() as u64
      - hours * 36e5 as u64
      - minutes * 6e4 as u64
      - seconds * 1e3 as u64;
    let fraction = Fraction::MilliSeconds(ms as u16);

    Timecode {
      hours: hours as u8,
      minutes: minutes as u8,
      seconds: seconds as u8,
      fraction,
    }
  }
}

impl From<(u32, FrameRate)> for Timecode {
  fn from((frames, frame_rate): (u32, FrameRate)) -> Self {
    let fps: f32 = {
      let frame_rate: frame_rate::Ratio<u32> = frame_rate.into();
      frame_rate.to_f32().unwrap_or_default()
    };

    let hours = frames / (60 * 60 * fps as u32);
    let minutes = frames / (60 * fps as u32) - hours * 60;
    let seconds = (frames / fps as u32) - minutes * 60 - hours * 60 * 60;
    let frames = frames
      - seconds * (fps as u32)
      - minutes * 60 * (fps as u32)
      - hours * 60 * 60 * (fps as u32);
    let color_frame = false;
    let drop_frame = false;

    let fraction = Fraction::Frame {
      frames: frames as u8,
      color_frame,
      drop_frame,
      frame_rate,
    };

    Timecode {
      hours: hours as u8,
      minutes: minutes as u8,
      seconds: seconds as u8,
      fraction,
    }
  }
}

impl From<&Timecode> for f64 {
  fn from(timecode: &Timecode) -> Self {
    (timecode.hours as f64) * 3600.0 + (timecode.minutes as f64) * 60.0 + (timecode.seconds as f64)
  }
}

impl Timecode {
  pub fn frame_rate(&self) -> Option<FrameRate> {
    match self.fraction {
      Frame { frame_rate, .. } => Some(frame_rate),
      _ => None,
    }
  }
  pub fn hours(&self) -> u8 {
    self.hours
  }
  pub fn minutes(&self) -> u8 {
    self.minutes
  }
  pub fn seconds(&self) -> u8 {
    self.seconds
  }

  pub fn fraction(&self) -> Fraction {
    match self.fraction {
      Frame {
        frames,
        drop_frame,
        color_frame,
        frame_rate,
      } => Frame {
        frames,
        drop_frame,
        color_frame,
        frame_rate,
      },
      MilliSeconds(ms) => MilliSeconds(ms),
    }
  }

  pub fn parse_smpte_331m(data: &[u8], frame_rate: FrameRate) -> Option<Self> {
    if data.len() != 17 {
      return None;
    }
    match data[0] {
      0x81 => Timecode::parse_smpte_12m(&data[1..], frame_rate),
      _ => None,
    }
  }

  pub fn parse_smpte_12m(data: &[u8], frame_rate: FrameRate) -> Option<Self> {
    if data.len() < 4 {
      return None;
    }

    let mask_tens_2 = 0b0011_0000;
    let mask_tens_3 = 0b0111_0000;
    let color_frame = (data[0] & 0b1000_0000) != 0;
    let drop_frame = (data[0] & 0b0100_0000) != 0;

    let fraction = Fraction::Frame {
      frames: (Timecode::get_number(data[0], mask_tens_2)),
      drop_frame,
      color_frame,
      frame_rate,
    };
    let seconds = Timecode::get_number(data[1], mask_tens_3);
    let minutes = Timecode::get_number(data[2], mask_tens_3);
    let hours = Timecode::get_number(data[3], mask_tens_2);

    Some(Timecode {
      hours,
      minutes,
      seconds,
      fraction,
    })
  }

  // used in STL format (EBU Tech 3264)
  pub fn from_ebu_smpte_time_and_control(data: &[u8; 4], frame_rate: FrameRate) -> Timecode {
    let fraction = Fraction::Frame {
      frames: data[3],
      drop_frame: false,
      color_frame: false,
      frame_rate,
    };
    Timecode {
      hours: data[0],
      minutes: data[1],
      seconds: data[2],
      fraction,
    }
  }

  fn get_number(data: u8, mask_tens: u8) -> u8 {
    let mask_unit = 0x0F;

    let tens = (data & mask_tens) >> 4;
    let unit = data & mask_unit;

    (10 * tens) + unit
  }
}

#[test]
fn timecode_from_frame() {
  let timecode = Timecode::from((24 * 60 * 60 * 10, FrameRate::_24_00));
  assert_eq!(timecode.hours(), 10);
  assert_eq!(timecode.minutes(), 0);
  assert_eq!(timecode.seconds(), 0);
  assert_eq!(
    timecode.fraction(),
    Frame {
      frames: 0,
      drop_frame: false,
      color_frame: false,
      frame_rate: FrameRate::_24_00
    }
  );

  let timecode = Timecode::from((24 * 60 * 60, FrameRate::_24_00));
  assert_eq!(timecode.hours(), 1);
  assert_eq!(timecode.minutes(), 0);
  assert_eq!(timecode.seconds(), 0);
  assert_eq!(
    timecode.fraction(),
    Frame {
      frames: 0,
      drop_frame: false,
      color_frame: false,
      frame_rate: FrameRate::_24_00
    }
  );

  let timecode = Timecode::from((24 * 60 * 60 - 1, FrameRate::_24_00));
  assert_eq!(timecode.hours(), 0);
  assert_eq!(timecode.minutes(), 59);
  assert_eq!(timecode.seconds(), 59);
  assert_eq!(
    timecode.fraction(),
    Frame {
      frames: 23,
      drop_frame: false,
      color_frame: false,
      frame_rate: FrameRate::_24_00
    }
  );

  let timecode = Timecode::from((24 * 60, FrameRate::_24_00));
  assert_eq!(timecode.hours(), 0);
  assert_eq!(timecode.minutes(), 1);
  assert_eq!(timecode.seconds(), 0);
  assert_eq!(
    timecode.fraction(),
    Frame {
      frames: 0,
      drop_frame: false,
      color_frame: false,
      frame_rate: FrameRate::_24_00
    }
  );

  let timecode = Timecode::from((24 * 60 - 1, FrameRate::_24_00));
  assert_eq!(timecode.hours(), 0);
  assert_eq!(timecode.minutes(), 0);
  assert_eq!(timecode.seconds(), 59);
  assert_eq!(
    timecode.fraction(),
    Frame {
      frames: 23,
      drop_frame: false,
      color_frame: false,
      frame_rate: FrameRate::_24_00
    }
  );

  let timecode = Timecode::from((24, FrameRate::_24_00));
  assert_eq!(timecode.hours(), 0);
  assert_eq!(timecode.minutes(), 0);
  assert_eq!(timecode.seconds(), 1);
  assert_eq!(
    timecode.fraction(),
    Frame {
      frames: 0,
      drop_frame: false,
      color_frame: false,
      frame_rate: FrameRate::_24_00
    }
  );

  let timecode = Timecode::from((23, FrameRate::_24_00));
  assert_eq!(timecode.hours(), 0);
  assert_eq!(timecode.minutes(), 0);
  assert_eq!(timecode.seconds(), 0);
  assert_eq!(
    timecode.fraction(),
    Frame {
      frames: 23,
      drop_frame: false,
      color_frame: false,
      frame_rate: FrameRate::_24_00
    }
  );

  let timecode = Timecode::from((25, FrameRate::_25_00));
  assert_eq!(timecode.hours(), 0);
  assert_eq!(timecode.minutes(), 0);
  assert_eq!(timecode.seconds(), 1);
  assert_eq!(
    timecode.fraction(),
    Frame {
      frames: 0,
      drop_frame: false,
      color_frame: false,
      frame_rate: FrameRate::_25_00
    }
  );

  let timecode = Timecode::from((24, FrameRate::_25_00));
  assert_eq!(timecode.hours(), 0);
  assert_eq!(timecode.minutes(), 0);
  assert_eq!(timecode.seconds(), 0);
  assert_eq!(
    timecode.fraction(),
    Frame {
      frames: 24,
      drop_frame: false,
      color_frame: false,
      frame_rate: FrameRate::_25_00
    }
  );

  let timecode = Timecode::from_ebu_smpte_time_and_control(&[10, 0, 0, 0], FrameRate::_25_00);
  assert_eq!(timecode.hours(), 10);
  assert_eq!(timecode.minutes(), 0);
  assert_eq!(timecode.seconds(), 0);
  assert_eq!(
    timecode.fraction(),
    Frame {
      frames: 0,
      drop_frame: false,
      color_frame: false,
      frame_rate: FrameRate::_25_00
    }
  );

  let timecode = Timecode::from_ebu_smpte_time_and_control(&[0, 10, 0, 0], FrameRate::_25_00);
  assert_eq!(timecode.hours(), 0);
  assert_eq!(timecode.minutes(), 10);
  assert_eq!(timecode.seconds(), 0);
  assert_eq!(
    timecode.fraction(),
    Frame {
      frames: 0,
      drop_frame: false,
      color_frame: false,
      frame_rate: FrameRate::_25_00
    }
  );

  let timecode = Timecode::from_ebu_smpte_time_and_control(&[0, 0, 10, 0], FrameRate::_25_00);
  assert_eq!(timecode.hours(), 0);
  assert_eq!(timecode.minutes(), 0);
  assert_eq!(timecode.seconds(), 10);
  assert_eq!(
    timecode.fraction(),
    Frame {
      frames: 0,
      drop_frame: false,
      color_frame: false,
      frame_rate: FrameRate::_25_00
    }
  );

  let timecode = Timecode::from_ebu_smpte_time_and_control(&[0, 0, 0, 10], FrameRate::_25_00);
  assert_eq!(timecode.hours(), 0);
  assert_eq!(timecode.minutes(), 0);
  assert_eq!(timecode.seconds(), 0);
  assert_eq!(
    timecode.fraction(),
    Frame {
      frames: 10,
      drop_frame: false,
      color_frame: false,
      frame_rate: FrameRate::_25_00
    }
  );
}

#[test]
fn timecode_from_fram2() {
  let timecode = Timecode::from((Duration::from_millis(2 * 60 * 1000 + 5 * 1000 + 66)));
  assert_eq!(timecode.hours(), 0);
  assert_eq!(timecode.minutes(), 2);
  assert_eq!(timecode.seconds(), 5);
  assert_eq!(timecode.fraction(), MilliSeconds(66));
}
