use crate::Beatmap;

#[derive(Debug, PartialEq)]
pub struct Editor {
    pub bookmarks: Vec<u32>,
    pub distance_spacing: f32,
    pub beat_divisor: u8,
    pub grid_size: u8,
    pub timeline_zoom: f32,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            bookmarks: Vec::new(),
            distance_spacing: 1.0,
            beat_divisor: 4,
            grid_size: 4,
            timeline_zoom: 1.0,
        }
    }
}
