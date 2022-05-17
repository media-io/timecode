use std::string::ToString;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum FrameRate {
  _24_00,
  _25_00,
  _30_00,
  _50_00,
  _60_00,
}

impl From<FrameRate> for f32 {
  fn from(fr: FrameRate) -> Self {
    match fr {
      FrameRate::_24_00 => 24.0,
      FrameRate::_25_00 => 25.0,
      FrameRate::_30_00 => 30.0,
      FrameRate::_50_00 => 50.0,
      FrameRate::_60_00 => 60.0,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Timecode {
  pub hours: u8,
  pub minutes: u8,
  pub seconds: u8,
  pub frame: u8,
  pub drop_frame: bool,
  pub color_frame: bool,
  frame_rate: FrameRate,
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

impl From<(u32, FrameRate)> for Timecode {
  fn from((frames, frame_rate): (u32, FrameRate)) -> Self {
    let fps: f32 = frame_rate.clone().into();

    let hours = frames / (60 * 60 * fps as u32);
    let minutes = frames / (60 * fps as u32) - hours * 60;
    let seconds = (frames / fps as u32) - minutes * 60 - hours * 60 * 60;
    let frame = frames
      - seconds * (fps as u32)
      - minutes * 60 * (fps as u32)
      - hours * 60 * 60 * (fps as u32);
    let drop_frame = false;
    let color_frame = false;

    log::trace!("{} {} {} {}", hours, minutes, seconds, frame);

    Timecode {
      hours: hours as u8,
      minutes: minutes as u8,
      seconds: seconds as u8,
      frame: frame as u8,
      drop_frame,
      color_frame,
      frame_rate,
    }
  }
}

impl From<&Timecode> for f64 {
  fn from(timecode: &Timecode) -> Self {
    (timecode.hours as f64) * 3600.0 + (timecode.minutes as f64) * 60.0 + (timecode.seconds as f64)
  }
}

impl Timecode {
  pub fn frame_rate(&self) -> &FrameRate {
    &self.frame_rate
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

    let frame = Timecode::get_number(data[0], mask_tens_2);
    let seconds = Timecode::get_number(data[1], mask_tens_3);
    let minutes = Timecode::get_number(data[2], mask_tens_3);
    let hours = Timecode::get_number(data[3], mask_tens_2);

    let color_frame = (data[0] & 0b1000_0000) != 0;
    let drop_frame = (data[0] & 0b0100_0000) != 0;

    Some(Timecode {
      hours,
      minutes,
      seconds,
      frame,
      drop_frame,
      color_frame,
      frame_rate,
    })
  }

  // used in STL format (EBU Tech 3264)
  pub fn from_ebu_smpte_time_and_control(data: &[u8; 4], frame_rate: FrameRate) -> Timecode {
    Timecode {
      hours: data[0],
      minutes: data[1],
      seconds: data[2],
      frame: data[3],
      drop_frame: false,
      color_frame: false,
      frame_rate,
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
  assert_eq!(timecode.hours, 10);
  assert_eq!(timecode.minutes, 0);
  assert_eq!(timecode.seconds, 0);
  assert_eq!(timecode.frame, 0);

  let timecode = Timecode::from((24 * 60 * 60, FrameRate::_24_00));
  assert_eq!(timecode.hours, 1);
  assert_eq!(timecode.minutes, 0);
  assert_eq!(timecode.seconds, 0);
  assert_eq!(timecode.frame, 0);

  let timecode = Timecode::from((24 * 60 * 60 - 1, FrameRate::_24_00));
  assert_eq!(timecode.hours, 0);
  assert_eq!(timecode.minutes, 59);
  assert_eq!(timecode.seconds, 59);
  assert_eq!(timecode.frame, 23);

  let timecode = Timecode::from((24 * 60, FrameRate::_24_00));
  assert_eq!(timecode.hours, 0);
  assert_eq!(timecode.minutes, 1);
  assert_eq!(timecode.seconds, 0);
  assert_eq!(timecode.frame, 0);

  let timecode = Timecode::from((24 * 60 - 1, FrameRate::_24_00));
  assert_eq!(timecode.hours, 0);
  assert_eq!(timecode.minutes, 0);
  assert_eq!(timecode.seconds, 59);
  assert_eq!(timecode.frame, 23);

  let timecode = Timecode::from((24, FrameRate::_24_00));
  assert_eq!(timecode.hours, 0);
  assert_eq!(timecode.minutes, 0);
  assert_eq!(timecode.seconds, 1);
  assert_eq!(timecode.frame, 0);

  let timecode = Timecode::from((23, FrameRate::_24_00));
  assert_eq!(timecode.hours, 0);
  assert_eq!(timecode.minutes, 0);
  assert_eq!(timecode.seconds, 0);
  assert_eq!(timecode.frame, 23);

  let timecode = Timecode::from((25, FrameRate::_25_00));
  assert_eq!(timecode.hours, 0);
  assert_eq!(timecode.minutes, 0);
  assert_eq!(timecode.seconds, 1);
  assert_eq!(timecode.frame, 0);

  let timecode = Timecode::from((24, FrameRate::_25_00));
  assert_eq!(timecode.hours, 0);
  assert_eq!(timecode.minutes, 0);
  assert_eq!(timecode.seconds, 0);
  assert_eq!(timecode.frame, 24);

  let timecode = Timecode::from_ebu_smpte_time_and_control(&[10, 0, 0, 0], FrameRate::_25_00);
  assert_eq!(timecode.hours, 10);
  assert_eq!(timecode.minutes, 0);
  assert_eq!(timecode.seconds, 0);
  assert_eq!(timecode.frame, 0);

  let timecode = Timecode::from_ebu_smpte_time_and_control(&[0, 10, 0, 0], FrameRate::_25_00);
  assert_eq!(timecode.hours, 0);
  assert_eq!(timecode.minutes, 10);
  assert_eq!(timecode.seconds, 0);
  assert_eq!(timecode.frame, 0);

  let timecode = Timecode::from_ebu_smpte_time_and_control(&[0, 0, 10, 0], FrameRate::_25_00);
  assert_eq!(timecode.hours, 0);
  assert_eq!(timecode.minutes, 0);
  assert_eq!(timecode.seconds, 10);
  assert_eq!(timecode.frame, 0);

  let timecode = Timecode::from_ebu_smpte_time_and_control(&[0, 0, 0, 10], FrameRate::_25_00);
  assert_eq!(timecode.hours, 0);
  assert_eq!(timecode.minutes, 0);
  assert_eq!(timecode.seconds, 0);
  assert_eq!(timecode.frame, 10);
}
