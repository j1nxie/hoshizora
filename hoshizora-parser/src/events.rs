use crate::Beatmap;

#[derive(Debug, PartialEq)]
pub enum EventType {
    Background,
    Video,
    Break,
    Storyboard,
}

#[derive(Debug, PartialEq)]
pub struct Event {
    pub event_type: EventType,
    pub start_time: u32,
    pub event_params: Vec<String>,
}

impl Event {
    fn new(event_type: EventType, start_time: u32, event_params: Vec<String>) -> Self {
        Self {
            event_type,
            start_time,
            event_params,
        }
    }
}

impl Default for Event {
    fn default() -> Self {
        Self {
            event_type: EventType::Background,
            start_time: 0,
            event_params: Vec::new(),
        }
    }
}

pub fn parse_events(line: &str, beatmap: &mut Beatmap) {
    let parsed: Vec<&str> = line.split(',').collect();
    let event = Event::default();
    event.event_type = match parsed[0].trim() {
        "0" => EventType::Background,
        "1" | "Video" => EventType::Video,
        "2" | "Break" => EventType::Break,
        _ => todo!(),
    }
}
