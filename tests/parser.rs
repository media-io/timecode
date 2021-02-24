extern crate timecode;
use timecode::*;

#[test]
fn test_smpte_12m_bad_length() {
  let data = vec![0, 0, 0];
  let tc = Timecode::<FrameRate2500>::parse_smpte_12m(&data);
  assert!(tc.is_none());
}

#[test]
fn test_smpte_12m_zero() {
  let data = vec![0, 0, 0, 0, 0, 0, 0, 0];
  let value = Timecode::<FrameRate2500>::parse_smpte_12m(&data);

  assert!(value.is_some());
  let tc = value.unwrap();
  assert_eq!(tc.hours, 0);
  assert_eq!(tc.minutes, 0);
  assert_eq!(tc.seconds, 0);
  assert_eq!(tc.frame, 0);
  assert!(!tc.drop_frame);
  assert!(!tc.color_frame);
}

#[test]
fn test_smpte_12m_full_range() {
  let data = vec![
    0b0011_1111,
    0b0111_1111,
    0b0111_1111,
    0b0011_1111,
    0,
    0,
    0,
    0,
  ];
  let value = Timecode::<FrameRate2500>::parse_smpte_12m(&data);

  assert!(value.is_some());
  let tc = value.unwrap();
  assert_eq!(tc.hours, 45);
  assert_eq!(tc.minutes, 85);
  assert_eq!(tc.seconds, 85);
  assert_eq!(tc.frame, 45);
  assert!(!tc.drop_frame);
  assert!(!tc.color_frame);
}

#[test]
fn test_smpte_12m_10_hours() {
  let data = vec![0, 0, 0, 0b0001_0000, 0, 0, 0, 0];
  let value = Timecode::<FrameRate2500>::parse_smpte_12m(&data);

  assert!(value.is_some());
  let tc = value.unwrap();
  assert_eq!(tc.hours, 10);
  assert_eq!(tc.minutes, 0);
  assert_eq!(tc.seconds, 0);
  assert_eq!(tc.frame, 0);
  assert!(!tc.drop_frame);
  assert!(!tc.color_frame);
}

#[test]
fn test_smpte_12m_drop_frame_and_color_frame() {
  let data = vec![0b1100_0000, 0, 0, 0, 0, 0, 0, 0];
  let value = Timecode::<FrameRate2500>::parse_smpte_12m(&data);

  assert!(value.is_some());
  let tc = value.unwrap();
  assert_eq!(tc.hours, 0);
  assert_eq!(tc.minutes, 0);
  assert_eq!(tc.seconds, 0);
  assert_eq!(tc.frame, 0);
  assert!(tc.drop_frame);
  assert!(tc.color_frame);
}

#[test]
fn test_smpte_331m_bad_length() {
  let data = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  let value = Timecode::<FrameRate2500>::parse_smpte_331m(&data);

  assert!(value.is_none());
}

#[test]
fn test_smpte_331m_bad_code() {
  let data = vec![
    0x00,
    0,
    0,
    0,
    0b0001_0000,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
  ];
  let value = Timecode::<FrameRate2500>::parse_smpte_331m(&data);

  assert!(value.is_none());
}

#[test]
fn test_smpte_331m_smpte_12m_content() {
  let data = vec![
    0x81,
    0,
    0,
    0,
    0b0001_0000,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
  ];
  let value = Timecode::<FrameRate2500>::parse_smpte_331m(&data);

  assert!(value.is_some());
  let tc = value.unwrap();
  assert_eq!(tc.hours, 10);
  assert_eq!(tc.minutes, 0);
  assert_eq!(tc.seconds, 0);
  assert_eq!(tc.frame, 0);
  assert!(!tc.drop_frame);
  assert!(!tc.color_frame);
}
