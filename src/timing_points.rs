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

