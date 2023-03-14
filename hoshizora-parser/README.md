# hoshizora-parser

`hoshizora-parser` is a library for parsing osu! beatmaps, written in the `.osu`
file format.

## features

- [x] General
- [x] Editor
- [x] Metadata
- [x] Difficulty
- [ ] Events
- [x] TimingPoints
- [ ] Colours
- [x] HitObjects

## examples

an example of the library's usage can be found in the `[tests]` directory of
this crate.

```rust
use hoshizora_parser::parse;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let file = File::open("./tests/end_time.osu").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).unwrap();

    let beatmap = parse(&content);
	// do things with the given beatmap here
}
```
	
## license

licensed under either of

*   Apache License, Version 2.0  
    ([LICENSE-APACHE](../LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
*   MIT license  
	([LICENSE-MIT](../LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## contribution

unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
