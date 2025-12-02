// prompt: https://adventofcode.com/2025/day/1

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

    let mut value = 50;
    let mut count = 0;

    contents.lines().for_each(|line| {
        let delta = line
            .trim_start_matches('L')
            .trim_start_matches('R')
            .parse::<i32>()
            .unwrap_or(0);

        if line.starts_with('L') {
            value = ((value - delta) % 100 + 100) % 100;
        } else if line.starts_with('R') {
            value = (value + delta) % 100;
        }

        if value == 0 {
            count += 1;
        }
    });

    count.to_string()
}

fn solve2(full: bool) -> String {
    let contents = read_input(1, full).expect("Failed to read input file");

    let mut value = 50;
    let mut count = 0;

    contents.lines().for_each(|line| {
        let delta = line
            .trim_start_matches('L')
            .trim_start_matches('R')
            .parse::<i32>()
            .unwrap_or(0);

        if line.starts_with('L') {
            count += ((100 - value) + delta) / 100;
            if value == 0 {
                count -= 1;
            };
            value = ((value - delta) % 100 + 100) % 100;
        } else if line.starts_with('R') {
            count += (value + delta) / 100;
            value = (value + delta) % 100;
        };

        // println!("Value: {}, Count: {}", value, count);

        // if value == 0 {
        //     count -= 1;
        // }
    });

    count.to_string()
}

inventory::submit! {
    crate::AocEntry2025 {
        year: 2025,
        day: 1,
        solve: solve,
    }
}
