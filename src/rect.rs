use std::fmt;
use std::str::FromStr;

pub struct Rect {
    a1: f32,
    a2: f32,
    b1: f32,
    b2: f32,
}

impl FromStr for Rect {
    type Err = RectParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct RectParseError;

impl fmt::Display for RectParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse rect bounds")
    }
}
