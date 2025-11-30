use std::fs;
use std::io::Error;
use std::path::Path;

pub struct AocEntry2024 {
    pub year: u32,
    pub day: u32,
    pub solve: fn(full: bool, part: Option<u8>) -> String,
}

inventory::collect!(AocEntry2024);

pub fn get_entries() -> Vec<&'static AocEntry2024> {
    inventory::iter::<AocEntry2024>.into_iter().collect()
}

pub fn read_input_full(day: u8) -> Result<String, Error> {
    return read_input(day, true);
}

pub fn read_input_example(day: u8) -> Result<String, Error> {
    return read_input(day, false);
}

pub fn read_input(day: u8, full: bool) -> Result<String, Error> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = if full {
        root.join(format!("inputs/day{:02}/full.txt", day))
    } else {
        root.join(format!("inputs/day{:02}/example.txt", day))
    };

    log::info!("Reading input file: {:?}", path);

    let contents = fs::read_to_string(path);

    if let Ok(ref s) = contents {
        log::info!("Read {} characters from input file", s.len());
        log::debug!(
            "First 100 characters:\n{}",
            &s.chars().take(100).collect::<String>()
        );
    }

    contents
}

pub mod day01;
pub mod day02;

