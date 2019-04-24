

use std::marker;
use std::string::ToString;

pub trait FrameRate {
  const FPS: u32;
}

#[derive(Debug, PartialEq)]
pub struct FrameRate2400;

impl FrameRate for FrameRate2400 {
    const FPS: u32 = 24;
}

#[derive(Debug, PartialEq)]
pub struct FrameRate2500;

impl FrameRate for FrameRate2500 {
    const FPS: u32 = 25;
}

#[derive(Debug, PartialEq)]
pub struct FrameRate3000;

impl FrameRate for FrameRate3000 {
    const FPS: u32 = 30;
}

#[derive(Debug, PartialEq)]
pub struct FrameRate5000;

impl FrameRate for FrameRate5000 {
    const FPS: u32 = 50;
}

#[derive(Debug, PartialEq)]
pub struct FrameRate6000;

impl FrameRate for FrameRate6000 {
    const FPS: u32 = 60;
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Timecode<FrameRate> {
  pub hours: u8,
  pub minutes: u8,
  pub seconds: u8,
  pub frame: u8,
  pub drop_frame: bool,
  pub color_frame: bool,
  frame_rate: marker::PhantomData::<FrameRate>,
}

impl<FrameRate> ToString for Timecode<FrameRate> {
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

impl<FrameRate> From<u32> for Timecode<FrameRate> {
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
      frame_rate: marker::PhantomData
    }
  }
}

impl<FrameRate> Timecode<FrameRate> {
  pub fn parse_smpte_331m(data: &[u8]) -> Option<Self> {
    if data.len() != 17 {
      return None;
    }
    match data[0] {
      0x81 => Timecode::<FrameRate>::parse_smpte_12m(&data[1..]),
      _ => None,
    }
  }

  pub fn parse_smpte_12m(data: &[u8]) -> Option<Self> {
    if data.len() < 4 {
      return None;
    }

    let mask_tens_2 = 0b0011_0000;
    let mask_tens_3 = 0b0111_0000;

    let frame = Timecode::<FrameRate>::get_number(data[0], mask_tens_2);
    let seconds = Timecode::<FrameRate>::get_number(data[1], mask_tens_3);
    let minutes = Timecode::<FrameRate>::get_number(data[2], mask_tens_3);
    let hours = Timecode::<FrameRate>::get_number(data[3], mask_tens_2);

    let color_frame = (data[0] & 0b1000_0000) != 0;
    let drop_frame = (data[0] & 0b0100_0000) != 0;

    Some(Timecode::<FrameRate> {
      hours: hours,
      minutes: minutes,
      seconds: seconds,
      frame: frame,
      drop_frame: drop_frame,
      color_frame: color_frame,
      frame_rate: marker::PhantomData::<FrameRate>,
    })
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
  let t1 = Timecode::<FrameRate2500>::from(900000);
  assert_eq!(t1.hours, 10);
  assert_eq!(t1.minutes, 0);
  assert_eq!(t1.seconds, 0);
  assert_eq!(t1.frame, 0);

  let t2 = Timecode::<FrameRate2500>::from(10);
  assert_eq!(t2.hours, 0);
  assert_eq!(t2.minutes, 0);
  assert_eq!(t2.seconds, 0);
  assert_eq!(t2.frame, 10);

  let t3 = Timecode::<FrameRate2500>::from(25);
  assert_eq!(t3.hours, 0);
  assert_eq!(t3.minutes, 0);
  assert_eq!(t3.seconds, 1);
  assert_eq!(t3.frame, 0);
}

