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
    pub fn new(
        bookmarks: Vec<u32>,
        distance_spacing: f32,
        beat_divisor: u8,
        grid_size: u8,
        timeline_zoom: f32,
    ) -> Self {
        Self {
            bookmarks,
            distance_spacing,
            beat_divisor,
            grid_size,
            timeline_zoom,
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            bookmarks: Vec::new(),
            distance_spacing: 1.0,
            beat_divisor: 4,
            grid_size: 4,
            timeline_zoom: 1.0,
        }
    }
}

pub fn parse_editor(line: &str, beatmap: &mut Beatmap) {
    let (k, v) = line.trim().split_once(':').unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Difficulty, General, Metadata};

    #[test]
    fn test_parse_editor() {
        let test_str = "DistanceSpacing: 0.8
        BeatDivisor: 4
        GridSize: 32
        TimelineZoom: 3.2";
        let mut beatmap = Beatmap::default();
        for line in test_str.lines() {
            parse_editor(line, &mut beatmap);
        }

        assert_eq!(
            beatmap,
            Beatmap {
                general: General::default(),
                editor: Editor {
                    bookmarks: Vec::new(),
                    distance_spacing: 0.8,
                    beat_divisor: 4,
                    grid_size: 32,
                    timeline_zoom: 3.2,
                },
                metadata: Metadata::default(),
                difficulty: Difficulty::default(),
                timing_points: Vec::new(),
                hit_objects: Vec::new(),
            }
        )
    }
}
