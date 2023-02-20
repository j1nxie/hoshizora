use crate::{
    difficulty::{parse_difficulty, Difficulty},
    editor::{parse_editor, Editor},
    file_sections::{get_section, FileSections},
    general::{parse_general, General},
    metadata::{parse_metadata, Metadata},
};

mod difficulty;
mod editor;
mod file_sections;
mod general;
mod metadata;

#[derive(Debug, PartialEq)]
pub struct Beatmap {
    pub general: General,
    pub editor: Editor,
    pub metadata: Metadata,
    pub difficulty: Difficulty,
}

impl Beatmap {
    fn new() -> Beatmap {
        Beatmap {
            general: General::new(),
            editor: Editor::new(),
            metadata: Metadata::new(),
            difficulty: Difficulty::new(),
        }
    }
}

pub fn parse(text: &str) -> Beatmap {
    let mut beatmap = Beatmap::new();
    let mut current_section = FileSections::None;

    for line in text.lines() {
        if !line.trim().is_empty() && !line.starts_with("//") {
            if get_section(line) != FileSections::None {
                current_section = get_section(line);
            } else {
                match current_section {
                    FileSections::General => parse_general(line, &mut beatmap),
                    FileSections::Editor => parse_editor(line, &mut beatmap),
                    FileSections::Difficulty => parse_difficulty(line, &mut beatmap),
                    FileSections::Metadata => parse_metadata(line, &mut beatmap),
                    _ => todo!(),
                }
            }
        }
    }

    beatmap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let test_str = "[General]
            AudioFilename: audio.mp3
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
            SamplesMatchPlaybackRate: 1

            [Editor]
            DistanceSpacing: 0.8
            BeatDivisor: 4
            GridSize: 32
            TimelineZoom: 3.2

            [Difficulty]
            HPDrainRate:6.0
            CircleSize:4.2
            OverallDifficulty:8.5
            ApproachRate:9.8
            SliderMultiplier:1.4
            SliderTickRate:2.0";
        let beatmap = parse(test_str);

        assert_eq!(beatmap.general.audio_filename, String::from("audio.mp3"));
        assert_eq!(beatmap.difficulty.cs, 4.2);
        assert_eq!(beatmap.editor.beat_divisor, 4);
        assert_eq!(beatmap.editor.bookmarks, Vec::new());
    }
}
