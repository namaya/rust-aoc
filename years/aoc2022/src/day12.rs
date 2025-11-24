
use std::{fs, path::Path};

pub fn solve() -> String {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let input_path = Path::new(project_root).join("inputs/day12/full.txt");

    let _contents = fs::read_to_string(input_path)
        .expect("Error reading file...");

    return "Not implemented".to_string();
}

inventory::submit! {
    crate::AocEntry2022 {
        year: 2022,
        day: 12,
        solve: solve,
    }
}
