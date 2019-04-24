
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Timecode {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub frame: u8,
    pub drop_frame: bool,
    pub color_frame: bool,
}

impl fmt::Display for Timecode
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let separator = match self.drop_frame {
            true => ';',
            false => ':',
        };

        write!(
            f,
            "{:02}:{:02}:{:02}{}{:02}",
            self.hours, self.minutes, self.seconds, separator, self.frame
        )
    }
}
