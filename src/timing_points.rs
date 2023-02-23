use crate::{general::SampleSet, Beatmap};
use std::str::FromStr;
use strum::ParseError::VariantNotFound;

#[derive(Debug, PartialEq)]
pub enum Effects {
    None,
    Kiai,
    OmitFirstBarline,
    All,
}

impl FromStr for Effects {
    type Err = strum::ParseError;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        match val {
            "0" => Ok(Self::None),
            "1" => Ok(Self::Kiai),
            "8" => Ok(Self::OmitFirstBarline),
            "9" => Ok(Self::All),
            _ => Err(VariantNotFound),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TimingPoint {
    pub time: u32,
    pub beat_length: f32,
    pub meter: u32,
    pub sample_set: SampleSet,
    pub sample_index: u32,
    pub volume: u32,
    pub uninherited: bool,
    pub effects: Effects,
}

#[allow(clippy::too_many_arguments)]
impl TimingPoint {
    pub fn new(
        time: u32,
        beat_length: f32,
        meter: u32,
        sample_set: SampleSet,
        sample_index: u32,
        volume: u32,
        uninherited: bool,
        effects: Effects,
    ) -> Self {
        Self {
            time,
            beat_length,
            meter,
            sample_set,
            sample_index,
            volume,
            uninherited,
            effects,
        }
    }
}

impl Default for TimingPoint {
    fn default() -> Self {
        TimingPoint::new(0, 500.0, 4, SampleSet::Default, 0, 100, true, Effects::None)
    }
}

pub fn parse_timing_points(line: &str, beatmap: &mut Beatmap) {
    let params: Vec<&str> = line.split(',').map(|x| x.trim()).collect();

    let timing_point = TimingPoint {
        time: params[0].parse::<u32>().unwrap(),
        beat_length: params[1].parse::<f32>().unwrap(),
        meter: params[2].parse::<u32>().unwrap(),
        sample_set: SampleSet::from_str(params[3]).unwrap(),
        sample_index: params[4].parse::<u32>().unwrap(),
        volume: params[5].parse::<u32>().unwrap(),
        uninherited: params[6].parse::<u8>().unwrap() != 0,
        effects: Effects::from_str(params[7]).unwrap(),
    };

    beatmap.timing_points.push(timing_point)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_timing_points() {
        let test_str = "1342, 333.33, 4, 2, 1, 35, 1, 0";
        let mut beatmap = Beatmap::default();
        parse_timing_points(test_str, &mut beatmap);

        assert_eq!(beatmap.timing_points.len(), 1);
        assert_eq!(
            beatmap.timing_points[0],
            TimingPoint {
                time: 1342,
                beat_length: 333.33,
                meter: 4,
                sample_set: SampleSet::Soft,
                sample_index: 1,
                volume: 35,
                uninherited: true,
                effects: Effects::None,
            }
        );
    }
}
