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

pub fn get_section(line: &str) -> FileSections {
    let trimmed_line = line.trim().trim_start_matches('[').trim_end_matches(']');
    match FileSections::from_str(trimmed_line) {
        Ok(t) => t,
        Err(_) => FileSections::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_success() {
        let test_str = "[Difficulty]";
        assert_eq!(get_section(test_str), FileSections::Difficulty);
    }

    #[test]
    fn test_parse_fail() {
        let test_str = "[Garbage]";
        assert_eq!(get_section(test_str), FileSections::None);
    }
}
