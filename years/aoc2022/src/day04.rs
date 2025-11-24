use std::{fs, path::Path};

pub fn solve() -> String {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let input_path = Path::new(project_root).join("inputs/day04/full.txt");
    let section_assignments = fs::read_to_string(input_path).unwrap();

    let mut count = 0;

    for pair in section_assignments.split("\n") {
        let assigments: Vec<&str> = pair.split(",").collect();

        let section_range1: Vec<u16> = assigments[0].split("-").map(|x| x.parse::<u16>().unwrap()).collect();
        let section_range2: Vec<u16> = assigments[1].split("-").map(|x| x.parse::<u16>().unwrap()).collect();

        if section_range1[0] < section_range2[0] && section_range1[1] >= section_range2[0] {
            count += 1;
        } else if section_range2[0] < section_range1[0] && section_range2[1] >= section_range1[0] {
            count += 1;
        } else if section_range1[0] == section_range2[0] {
            count += 1;
        }

        // part 1
        // if section_range1[0] < section_range2[0] && section_range1[1] >= section_range2[1] {
        //     count += 1;
        // } else if section_range2[0] < section_range1[0] && section_range2[1] >= section_range1[1] {
        //     count += 1;
        // } else if section_range1[0] == section_range2[0] {
        //     count += 1;
        // }
    }

    count.to_string()
}

inventory::submit! {
    crate::AocEntry2022 {
        year: 2022,
        day: 4,
        solve: solve,
    }
}
