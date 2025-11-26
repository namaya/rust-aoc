// prompt: https://adventofcode.com/2024/day/1

use std::fs;
use std::io::Error;
use std::path::Path;

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

pub fn solve1(full: bool) -> String {
    let contents = read_input(1, full).expect("Failed to read input file");

    let (mut left_list, mut right_list): (Vec<i32>, Vec<i32>) = contents
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            if nums.len() >= 2 {
                Some((nums[0], nums[1]))
            } else {
                None
            }
        })
        .unzip();

    left_list.sort();
    right_list.sort();

    let total: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    total.to_string()
}

pub fn solve2() -> String {
    // Placeholder for part 2 solution
    "Not implemented".to_string()
}

inventory::submit! {
    crate::AocEntry2024 {
        year: 2024,
        day: 1,
        solve: solve1,
    }
}
