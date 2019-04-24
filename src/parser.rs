use crate::timecode::Timecode;

fn get_number(data: u8, mask_tens: u8) -> u8 {
  let mask_unit = 0x0F;

  let tens = (data & mask_tens) >> 4;
  let unit = data & mask_unit;

  (10 * tens) + unit
}

pub fn smpte_331m(data: &[u8]) -> Option<Timecode> {
  if data.len() != 17 {
    return None;
  }
  match data[0] {
    0x81 => smpte_12m(&data[1..]),
    _ => None,
  }
}

pub fn smpte_12m(data: &[u8]) -> Option<Timecode> {
  if data.len() < 4 {
    return None;
  }

  let mask_tens_2 = 0b0011_0000;
  let mask_tens_3 = 0b0111_0000;

  let frame = get_number(data[0], mask_tens_2);
  let seconds = get_number(data[1], mask_tens_3);
  let minutes = get_number(data[2], mask_tens_3);
  let hours = get_number(data[3], mask_tens_2);

  let color_frame = (data[0] & 0b1000_0000) != 0;
  let drop_frame = (data[0] & 0b0100_0000) != 0;

  Some(Timecode {
    hours: hours,
    minutes: minutes,
    seconds: seconds,
    frame: frame,
    drop_frame: drop_frame,
    color_frame: color_frame,
  })
}
