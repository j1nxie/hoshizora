use crate::file_sections::{get_section, FileSections};

mod file_sections;

#[derive(Debug, PartialEq)]
pub struct Difficulty {
    hp: f32,
    cs: f32,
    od: f32,
    ar: f32,
    slider_multiplier: f32,
    slider_tickrate: f32,
}

impl Difficulty {
    fn new() -> Difficulty {
        Difficulty {
            hp: 5.0,
            cs: 5.0,
            od: 5.0,
            ar: 5.0,
            slider_multiplier: 1.4,
            slider_tickrate: 1.0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Beatmap {
    difficulty: Difficulty,
}

impl Beatmap {
    fn new() -> Beatmap {
        Beatmap {
            difficulty: Difficulty::new(),
        }
    }
}

pub fn parse(text: &str) -> Beatmap {
    let mut beatmap = Beatmap::new();
    let mut current_section = FileSections::Format;

    for line in text.lines() {
        if line.trim().is_empty() && !line.starts_with("//") {
            if get_section(line) != FileSections::None(line.to_string()) {
                current_section = get_section(line);
            } else {
                match current_section {
                    FileSections::Difficulty => parse_difficulty(line, &mut beatmap),
                    _ => todo!(),
                }
            }
        }
    }

    beatmap
}

fn parse_difficulty(line: &str, beatmap: &mut Beatmap) {
    let (k, v) = line.split_once(':').unwrap();
    match k.trim() {
        "HPDrainRate" => beatmap.difficulty.hp = v.trim().parse::<f32>().unwrap(),
        "CircleSize" => beatmap.difficulty.cs = v.trim().parse::<f32>().unwrap(),
        "OverallDifficulty" => beatmap.difficulty.od = v.trim().parse::<f32>().unwrap(),
        "ApproachRate" => beatmap.difficulty.ar = v.trim().parse::<f32>().unwrap(),
        "SliderMultiplier" => {
            beatmap.difficulty.slider_multiplier = v.trim().parse::<f32>().unwrap()
        }
        "SliderTickRate" => beatmap.difficulty.slider_tickrate = v.trim().parse::<f32>().unwrap(),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_difficulty() {
        let test_str = "HPDrainRate:6.0
            CircleSize:4.2
            OverallDifficulty:8.5
            ApproachRate:9.8
            SliderMultiplier:1.4
            SliderTickRate:2.0";
        let mut beatmap = Beatmap::new();
        for line in test_str.lines() {
            parse_difficulty(line, &mut beatmap);
        }

        assert_eq!(
            beatmap,
            Beatmap {
                difficulty: Difficulty {
                    hp: 6.0,
                    cs: 4.2,
                    od: 8.5,
                    ar: 9.8,
                    slider_multiplier: 1.4,
                    slider_tickrate: 2.0,
                }
            }
        );
    }
}
