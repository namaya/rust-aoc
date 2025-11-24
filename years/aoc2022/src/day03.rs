use std::{fs, path::Path};

pub fn solve() -> String {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let file_path = Path::new(project_root).join("inputs/day03/full.txt");

    let rucksacks = fs::read_to_string(file_path)
                                    .unwrap();

    let mut total_priority: u32 = 0;

    let mut groups = ["", "", ""];
    let mut i = 0;

    for rucksack in rucksacks.split("\n") {
        groups[i] = rucksack;
        i = (i + 1) % 3;

        if i != 0 || groups[0].len() == 0 {
            continue;
        };

        let mut special_item = 0u8;

        for item in groups[0].as_bytes() {
            if groups[1].as_bytes().contains(item) && groups[2].as_bytes().contains(item) {
                special_item = *item;
                break;
            }
        }

        let priority = if special_item >= b'a' {
            special_item - b'a' + 1
        } else {
            special_item - b'A' + 27
        };

        total_priority += u32::from(priority);
    }

    total_priority.to_string()
}

inventory::submit! {
    crate::AocEntry2022 {
        year: 2022,
        day: 3,
        solve: solve,
    }
}
