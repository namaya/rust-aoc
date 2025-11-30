// prompt: https://adventofcode.com/2024/day/1

use crate::read_input;

pub fn solve(full: bool, part: Option<u8>) -> String {
    match part {
        Some(1) => solve1(full),
        Some(2) => solve2(full),
        None => {
            let part1 = solve1(full);
            let part2 = solve2(full);
            format!("Part 1: {}\tPart 2: {}", part1, part2)
        }
        _ => "Invalid part".to_string(),
    }
}

fn solve1(full: bool) -> String {
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

fn solve2(full: bool) -> String {
    let contents = read_input(1, full).expect("Failed to read input file");

    let (left_list, right_list): (Vec<i32>, Vec<i32>) = contents
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

    let total: i32 = left_list
        .iter()
        .map(|left_val| {
            let count = right_list
                .iter()
                .filter(|&right_val| right_val == left_val)
                .count() as i32;
            left_val * count
        })
        .sum();

    total.to_string()
}

inventory::submit! {
    crate::AocEntry2024 {
        year: 2024,
        day: 1,
        solve: solve,
    }
}
