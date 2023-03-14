use hoshizora_parser::{parse, Beatmap};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

#[test]
fn test_parse_from_file() {
    let file = File::open("./tests/end_time.osu").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).unwrap();

    let beatmap = parse(&content);

    assert_eq!(beatmap.metadata.title, String::from("End Time"));
}
