
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Timecode {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub frame: u8,
    pub drop_frame: bool,
    pub color_frame: bool,
}
