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

pub fn parse_hit_objects(line: &str, beatmap: &mut Beatmap) {
    let mut iter = line.trim().split(',');

    let position_x = iter.next().unwrap().parse::<i32>().unwrap();
    let position_y = iter.next().unwrap().parse::<i32>().unwrap();
    let time = iter.next().unwrap().parse::<u32>().unwrap();
    let object_type = iter.next().unwrap().parse::<u32>().unwrap();

    let new_combo = object_type & 0b00000100 != 0;
    let color_skip = (object_type >> 4) & 0b00000111;

    let hitsound = iter.next().unwrap().parse::<u32>().unwrap();

    let hit_object = match object_type & 0b10001011 {
        1 => HitObject::HitCircle(HitCircle {
            position_x,
            position_y,
            new_combo,
            color_skip,
            time,
            hitsound,
            hit_sample: match iter.next() {
                Some(t) => parse_hit_sample(t),
                None => HitSample::default(),
            },
        }),

        2 => {
            let (curve_type, curve_points) = match iter.next() {
                Some(t) => parse_curve(t),
                None => panic!(),
            };

            HitObject::Slider(Slider {
                position_x,
                position_y,
                new_combo,
                color_skip,
                time,
                hitsound,
                curve_type,
                curve_points,
                slides: iter.next().unwrap().parse::<u32>().unwrap(),
                length: iter.next().unwrap().parse::<f32>().unwrap(),
                edge_sounds: match iter.next() {
                    Some(t) => parse_slider_edge_sounds(t),
                    None => panic!(),
                },
                edge_additions: match iter.next() {
                    Some(t) => parse_slider_edge_sets(t),
                    None => panic!(),
                },
                hit_sample: match iter.next() {
                    Some(t) => parse_hit_sample(t),
                    None => HitSample::default(),
                },
            })
        }

        8 => HitObject::Spinner(Spinner {
            position_x,
            position_y,
            new_combo,
            color_skip,
            time,
            hitsound,
            end_time: iter.next().unwrap().parse::<u32>().unwrap(),
            hit_sample: match iter.next() {
                Some(t) => parse_hit_sample(t),
                None => HitSample::default(),
            },
        }),

        128 => {
            let (end_time, hit_sample) = iter
                .next()
                .unwrap()
                .split_once(':')
                .map(|(et, hs)| {
                    let et = et.parse::<u32>().unwrap();
                    let hs = parse_hit_sample(hs);
                    (et, hs)
                })
                .unwrap();
            HitObject::ManiaHold(ManiaHold {
                position_x,
                position_y,
                new_combo,
                color_skip,
                time,
                hitsound,
                end_time,
                hit_sample,
            })
        }
        _ => unimplemented!(),
    };

    beatmap.hit_objects.push(hit_object);
}

pub fn parse_hit_sample(line: &str) -> HitSample {
    let mut iter = line.trim().split(':');

    let normal_set = iter.next().unwrap().parse::<u32>().unwrap();
    let addition_set = iter.next().unwrap().parse::<u32>().unwrap();
    let index = iter.next().unwrap().parse::<u32>().unwrap();
    let volume = iter.next().unwrap().parse::<u32>().unwrap();
    let filename = iter.next().unwrap().to_string();

    HitSample {
        normal_set,
        addition_set,
        index,
        volume,
        filename,
    }
}

pub fn parse_curve(line: &str) -> (CurveType, Vec<(i32, i32)>) {
    let mut iter = line.trim().split('|');

    let curve_type = match iter.next().unwrap() {
        "B" => CurveType::Bezier,
        "C" => CurveType::Catmull,
        "L" => CurveType::Linear,
        "P" => CurveType::Perfect,
        _ => panic!(),
    };

    let mut curve_points: Vec<(i32, i32)> = vec![];
    for t in iter {
        let (x, y) = t.split_once(':').unwrap();
        curve_points.push((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
    }

    (curve_type, curve_points)
}

pub fn parse_slider_edge_sounds(line: &str) -> Vec<u32> {
    line.trim()
        .split('|')
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

pub fn parse_slider_edge_sets(line: &str) -> Vec<(u32, u32)> {
    line.trim()
        .split('|')
        .map(|s| {
            let (x, y) = s.split_once(':').unwrap();
            (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hit_objects() {
        let test_str = "256,192,11000,21,2
        256,192,11200,8,12,12000,3:0:0:80:
        100,100,12600,6,1,B|200:200|250:200|250:200|300:150,2,310.123,2|1|2,0:0|0:0|0:2,0:0:0:0:";
        let mut beatmap = Beatmap::default();
        for line in test_str.lines() {
            parse_hit_objects(line, &mut beatmap);
        }

        assert_eq!(
            beatmap.hit_objects[0],
            HitObject::HitCircle(HitCircle {
                position_x: 256,
                position_y: 192,
                new_combo: true,
                color_skip: 1,
                time: 11000,
                hitsound: 2,
                hit_sample: HitSample::default(),
            })
        );

        assert_eq!(
            beatmap.hit_objects[1],
            HitObject::Spinner(Spinner {
                position_x: 256,
                position_y: 192,
                new_combo: false,
                color_skip: 0,
                time: 11200,
                hitsound: 12,
                end_time: 12000,
                hit_sample: HitSample {
                    normal_set: 3,
                    addition_set: 0,
                    index: 0,
                    volume: 80,
                    filename: String::new(),
                }
            })
        );

        assert_eq!(
            beatmap.hit_objects[2],
            HitObject::Slider(Slider {
                position_x: 100,
                position_y: 100,
                new_combo: true,
                color_skip: 0,
                time: 12600,
                hitsound: 1,
                curve_type: CurveType::Bezier,
                curve_points: vec![(200, 200), (250, 200), (250, 200), (300, 150)],
                slides: 2,
                length: 310.123,
                edge_sounds: vec![2, 1, 2],
                edge_additions: vec![(0, 0), (0, 0), (0, 2)],
                hit_sample: HitSample::default(),
            })
        );
    }
}
