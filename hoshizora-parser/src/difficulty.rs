use crate::Beatmap;

#[derive(Debug, PartialEq)]
pub struct Difficulty {
    pub hp: f32,
    pub cs: f32,
    pub od: f32,
    pub ar: f32,
    pub slider_multiplier: f32,
    pub slider_tickrate: f32,
}

impl Difficulty {
    pub fn new(
        hp: f32,
        cs: f32,
        od: f32,
        ar: f32,
        slider_multiplier: f32,
        slider_tickrate: f32,
    ) -> Self {
        Self {
            hp,
            cs,
            od,
            ar,
            slider_multiplier,
            slider_tickrate,
        }
    }
}

impl Default for Difficulty {
    fn default() -> Self {
        Self {
            hp: 5.0,
            cs: 5.0,
            od: 5.0,
            ar: 5.0,
            slider_multiplier: 1.0,
            slider_tickrate: 1.4,
        }
    }
}

pub fn parse_difficulty(line: &str, beatmap: &mut Beatmap) {
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
    use crate::{Editor, Format, General, Metadata};

    #[test]
    fn test_parse_difficulty() {
        let test_str = "HPDrainRate:6.0
            CircleSize:4.2
            OverallDifficulty:8.5
            ApproachRate:9.8
            SliderMultiplier:1.4
            SliderTickRate:2.0";
        let mut beatmap = Beatmap::default();
        for line in test_str.lines() {
            parse_difficulty(line, &mut beatmap);
        }

        assert_eq!(
            beatmap,
            Beatmap {
                format: Format::default(),
                general: General::default(),
                editor: Editor::default(),
                metadata: Metadata::default(),
                difficulty: Difficulty {
                    hp: 6.0,
                    cs: 4.2,
                    od: 8.5,
                    ar: 9.8,
                    slider_multiplier: 1.4,
                    slider_tickrate: 2.0,
                },
                timing_points: Vec::new(),
                hit_objects: Vec::new(),
            }
        );
    }
}
