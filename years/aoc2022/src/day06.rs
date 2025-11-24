
use std::{fs, path::Path};
use std::collections::HashSet;

pub fn solve() -> String {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let input_path = Path::new(project_root).join("inputs/day06/full.txt");

    let stream = fs::read_to_string(input_path)
        .expect("Error reading file...");

    let lookback_window = 14;
    let mut i = lookback_window;

    while i < stream.len() {
        let characters = &stream[i-lookback_window..i];

        let mut character_set = HashSet::new();

        for character in characters.chars() {
            character_set.insert(character);
        }

        if character_set.len() == lookback_window {
            break;
        }

        i += 1;
    }

    return i.to_string();
}

inventory::submit! {
    crate::AocEntry2022 {
        year: 2022,
        day: 6,
        solve: solve,
    }
}
