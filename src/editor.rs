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

pub fn parse_editor(line: &str, beatmap: &mut Beatmap) {
    let (k, v) = line.split_once(':').unwrap();
    match k.trim() {
        "Bookmarks" => {
            beatmap.editor.bookmarks = v
                .trim()
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        }
        "DistanceSpacing" => beatmap.editor.distance_spacing = v.trim().parse::<f32>().unwrap(),
        "BeatDivisor" => beatmap.editor.beat_divisor = v.trim().parse::<u8>().unwrap(),
        "GridSize" => beatmap.editor.grid_size = v.trim().parse::<u8>().unwrap(),
        "TimelineZoom" => beatmap.editor.timeline_zoom = v.trim().parse::<f32>().unwrap(),
        _ => {}
    }
}

