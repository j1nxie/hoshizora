use crate::Beatmap;

#[derive(Debug, PartialEq)]
pub enum HitObject {
    HitCircle(HitCircle),
    Slider(Slider),
    Spinner(Spinner),
    ManiaHold(ManiaHold),
}

#[derive(Debug, PartialEq)]
pub struct HitCircle {
    pub position_x: i32,
    pub position_y: i32,
    pub new_combo: bool,
    pub color_skip: u32,
    pub time: u32,
    pub hitsound: u32,
    pub hit_sample: HitSample,
}

#[derive(Debug, PartialEq)]
pub struct Slider {
    pub position_x: i32,
    pub position_y: i32,
    pub new_combo: bool,
    pub color_skip: u32,
    pub time: u32,
    pub hitsound: u32,
    pub curve_type: CurveType,
    pub curve_points: Vec<(i32, i32)>,
    pub slides: u32,
    pub length: f32,
    pub edge_sounds: Vec<u32>,
    pub edge_additions: Vec<(u32, u32)>,
    pub hit_sample: HitSample,
}

#[derive(Debug, PartialEq)]
pub enum CurveType {
    Linear,
    Bezier,
    Perfect,
    Catmull,
}

#[derive(Debug, PartialEq)]
pub struct Spinner {
    pub position_x: i32,
    pub position_y: i32,
    pub new_combo: bool,
    pub color_skip: u32,
    pub time: u32,
    pub hitsound: u32,
    pub end_time: u32,
    pub hit_sample: HitSample,
}

#[derive(Debug, PartialEq)]
pub struct ManiaHold {
    pub position_x: i32,
    pub position_y: i32,
    pub new_combo: bool,
    pub color_skip: u32,
    pub time: u32,
    pub hitsound: u32,
    pub end_time: u32,
    pub hit_sample: HitSample,
}

#[derive(Default, Debug, PartialEq)]
pub struct HitSample {
    pub normal_set: u32,
    pub addition_set: u32,
    pub index: u32,
    pub volume: u32,
    pub filename: String,
}
