use crate::Beatmap;

#[derive(Debug, PartialEq)]
pub struct Format {
    version: u32,
}

impl Format {
    pub fn new(version: u32) -> Self {
        Self { version }
    }
}

impl Default for Format {
    fn default() -> Self {
        Self { version: 14 }
    }
}

pub fn parse_format(line: &str, beatmap: &mut Beatmap) {
    beatmap.format.version = line
        .trim()
        .trim_start_matches("osu file format v")
        .parse::<u32>()
        .unwrap();
}
