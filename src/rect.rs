use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Rect {
    a1: f32,
    a2: f32,
    b1: f32,
    b2: f32,
}

impl FromStr for Rect {
    type Err = RectParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bounds: Vec<&str> = s.split(":").collect();
        if bounds.len() != 4 {
            return Err(RectParseError);
        }
        let bounds = bounds
            .iter()
            .map(|b| b.parse::<f32>())
            .collect::<Result<Vec<f32>, std::num::ParseFloatError>>()
            .map_err(|_| RectParseError)?;

        Ok(Self {
            a1: bounds[0],
            a2: bounds[1],
            b1: bounds[2],
            b2: bounds[3],
        })
    }
}

#[derive(Debug, Clone)]
pub struct RectParseError;

impl fmt::Display for RectParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse rect bounds")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {
        let bounds = "-3:1.0:4:-3.5";
        let parsed = Rect::from_str(bounds).unwrap();

        assert_eq!(
            parsed,
            Rect {
                a1: -3.0,
                a2: 1.0,
                b1: 4.0,
                b2: -3.5
            }
        );
    }

    #[test]
    fn test_invalid_parse() {
        let some_str = "abc:asbas";
        match Rect::from_str(some_str) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
    }
}
