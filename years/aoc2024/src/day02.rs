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

fn is_safe_report(report: &[u32]) -> bool {
    if report.len() < 2 {
        return false;
    }

    let is_ascending = report
        .windows(2)
        .all(|w| w[1] > w[0] && w[1] - w[0] >= 1 && w[1] - w[0] <= 3);

    let is_descending = report
        .windows(2)
        .all(|w| w[0] > w[1] && w[0] - w[1] >= 1 && w[0] - w[1] <= 3);

    is_ascending || is_descending
}

fn is_safe_report_with_tolerance(report: &[u32]) -> bool {
    if report.len() < 2 {
        return false;
    }

    if is_safe_report(report) {
        return true;
    }

    for i in 0..report.len() {
        let modified_report: Vec<u32> = report
            .iter()
            .enumerate()
            .filter_map(|(idx, &val)| if idx != i { Some(val) } else { None })
            .collect();

        if is_safe_report(&modified_report) {
            return true;
        }
    }

    false
}

pub fn solve1(full: bool) -> String {
    let contents = read_input(2, full).expect("Failed to read input file");

    let safe_count = contents
        .lines()
        .filter(|line| {
            let report: Vec<u32> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();

            is_safe_report(&report)
        })
        .count();

    safe_count.to_string()
}

pub fn solve2(full: bool) -> String {
    let contents = read_input(2, full).expect("Failed to read input file");

    let safe_count = contents
        .lines()
        .filter(|line| {
            let report: Vec<u32> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();

            is_safe_report_with_tolerance(&report)
        })
        .count();

    safe_count.to_string()
}

inventory::submit! {
    crate::AocEntry2024 {
        year: 2024,
        day: 2,
        solve: solve,
    }
}
