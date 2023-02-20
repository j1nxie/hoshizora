use crate::Beatmap;

#[derive(Debug, PartialEq)]
pub struct General {
    pub audio_filename: String,
    pub audio_lead_in: u32,
    pub audio_hash: String,
    pub preview_time: i32,
    pub countdown: u32,     // TODO: this should be an enum
    pub sample_set: String, // TODO: this should also be an enum
    pub stack_leniency: f32,
    pub mode: u32, // TODO: think about whether this should be an enum
    pub letterbox_in_breaks: bool,
    pub story_fire_in_front: bool,
    pub use_skin_sprites: bool,
    pub always_show_playfield: bool,
    pub overlay_position: String, // TODO: this probably should be an enum
    pub skin_preference: String,
    pub epilepsy_warning: bool,
    pub countdown_offset: u32,
    pub special_style: bool,
    pub widescreen_storyboard: bool,
    pub samples_match_playback_rate: bool,
}

impl General {
    pub fn new() -> General {
        General {
            audio_filename: String::new(),
            audio_lead_in: 0,
            audio_hash: String::new(),
            preview_time: -1,
            countdown: 1,
            sample_set: String::from("Normal"),
            stack_leniency: 0.7,
            mode: 0,
            letterbox_in_breaks: false,
            story_fire_in_front: true,
            use_skin_sprites: false,
            always_show_playfield: false,
            overlay_position: String::from("NoChange"),
            skin_preference: String::new(),
            epilepsy_warning: false,
            countdown_offset: 0,
            special_style: false,
            widescreen_storyboard: false,
            samples_match_playback_rate: false,
        }
    }
}

pub fn parse_general(line: &str, beatmap: &mut Beatmap) {
    let (k, v) = line.split_once(':').unwrap();
    match k.trim() {
        "AudioFilename" => beatmap.general.audio_filename = String::from(v.trim()),
        "AudioLeadIn" => beatmap.general.audio_lead_in = v.trim().parse::<u32>().unwrap(),
        "AudioHash" => beatmap.general.audio_hash = String::from(v.trim()),
        "PreviewTime" => beatmap.general.preview_time = v.trim().parse::<i32>().unwrap(),
        "Countdown" => beatmap.general.countdown = v.trim().parse::<u32>().unwrap(),
        "SampleSet" => beatmap.general.sample_set = String::from(v.trim()),
        "StackLeniency" => beatmap.general.stack_leniency = v.trim().parse::<f32>().unwrap(),
        "Mode" => beatmap.general.mode = v.trim().parse::<u32>().unwrap(),
        "LetterboxInBreaks" => {
            beatmap.general.letterbox_in_breaks = v.trim().parse::<u8>().unwrap() != 0
        }
        "StoryFireInFront" => {
            beatmap.general.story_fire_in_front = v.trim().parse::<u8>().unwrap() != 0
        }
        "UseSkinSprites" => beatmap.general.use_skin_sprites = v.trim().parse::<u8>().unwrap() != 0,
        "AlwaysShowPlayfield" => {
            beatmap.general.always_show_playfield = v.trim().parse::<u8>().unwrap() != 0
        }
        "OverlayPosition" => beatmap.general.overlay_position = String::from(v.trim()),
        "SkinPreference" => beatmap.general.skin_preference = String::from(v.trim()),
        "EpilepsyWarning" => {
            beatmap.general.epilepsy_warning = v.trim().parse::<u8>().unwrap() != 0
        }
        "CountdownOffset" => beatmap.general.countdown_offset = v.trim().parse::<u32>().unwrap(),
        "SpecialStyle" => beatmap.general.special_style = v.trim().parse::<u8>().unwrap() != 0,
        "WidescreenStoryboard" => {
            beatmap.general.widescreen_storyboard = v.trim().parse::<u8>().unwrap() != 0
        }
        "SamplesMatchPlaybackRate" => {
            beatmap.general.samples_match_playback_rate = v.trim().parse::<u8>().unwrap() != 0
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Difficulty, Editor};

    #[test]
    fn test_parse_general() {
        let test_str = "AudioFilename: audio.mp3
            AudioLeadIn: 0
            AudioHash: afjskldfjaldksfjklasf
            PreviewTime: 10
            Countdown: 0
            SampleSet: Drum
            StackLeniency: 0.75
            Mode: 1
            LetterboxInBreaks: 1
            StoryFireInFront: 0
            UseSkinSprites: 1
            AlwaysShowPlayfield: 1
            OverlayPosition: Below
            SkinPreference: Seoul v10
            EpilepsyWarning: 1
            CountdownOffset: 1
            SpecialStyle: 1
            WidescreenStoryboard: 1
            SamplesMatchPlaybackRate: 1";
        let mut beatmap = Beatmap::new();
        for line in test_str.lines() {
            parse_general(line, &mut beatmap)
        }

        assert_eq!(
            beatmap,
            Beatmap {
                general: General {
                    audio_filename: String::from("audio.mp3"),
                    audio_lead_in: 0,
                    audio_hash: String::from("afjskldfjaldksfjklasf"),
                    preview_time: 10,
                    countdown: 0,
                    sample_set: String::from("Drum"),
                    stack_leniency: 0.75,
                    mode: 1,
                    letterbox_in_breaks: true,
                    story_fire_in_front: false,
                    use_skin_sprites: true,
                    always_show_playfield: true,
                    overlay_position: String::from("Below"),
                    skin_preference: String::from("Seoul v10"),
                    epilepsy_warning: true,
                    countdown_offset: 1,
                    special_style: true,
                    widescreen_storyboard: true,
                    samples_match_playback_rate: true,
                },
                editor: Editor::new(),
                difficulty: Difficulty::new(),
            }
        );
    }
}
