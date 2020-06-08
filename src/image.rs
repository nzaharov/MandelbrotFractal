use std::str::FromStr;

pub struct ImageSize {
    pub width: u32,
    pub height: u32,
}

impl FromStr for ImageSize {
    type Err = SizeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dimensions: Vec<&str> = s.split("x").collect();

        let width = dimensions
            .get(0)
            .ok_or(SizeParseError)?
            .parse::<u32>()
            .map_err(|_| SizeParseError)?;
        let height = dimensions
            .get(1)
            .ok_or(SizeParseError)?
            .parse::<u32>()
            .map_err(|_| SizeParseError)?;

        Ok(Self { width, height })
    }
}

#[derive(Debug, Clone)]
pub struct SizeParseError;

impl std::fmt::Display for SizeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Could not parse size.")
    }
}
