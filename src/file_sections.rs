use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
pub enum FileSections {
    None,
    Format,
    General,
    Editor,
    Metadata,
    Difficulty,
    Events,
    TimingPoints,
    Colours,
    HitObjects,
}

impl FileSections {
    pub fn get_section(line: &str) -> Self {
        let trimmed_line = line.trim().trim_start_matches('[').trim_end_matches(']');
        match Self::from_str(trimmed_line) {
            Ok(t) => t,
            Err(_) => Self::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_success() {
        let test_str = "[Difficulty]";
        assert_eq!(
            FileSections::get_section(test_str),
            FileSections::Difficulty
        );
    }

    #[test]
    fn test_parse_fail() {
        let test_str = "[Garbage]";
        assert_eq!(FileSections::get_section(test_str), FileSections::None);
    }
}
