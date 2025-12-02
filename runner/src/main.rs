use anyhow::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    /// year to run (e.g., 2024)
    year: u32,
    /// day to run (e.g., 1). If omitted, run all days.
    #[structopt(short, long)]
    day: Option<u32>,
    /// part to run (1 or 2). If omitted, run both parts.
    #[structopt(short, long)]
    part: Option<u8>,
    /// run on full input
    #[structopt(short, long)]
    full: bool,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::from_args();
    let entries2022 = aoc2022::get_entries();
    let entries2023 = aoc2023::get_entries();
    let entries2024 = aoc2024::get_entries();
    let entries2025 = aoc2025::get_entries();

    let mut results = Vec::new();

    match args.year {
        2022 => {
            for entry in entries2022.iter() {
                if args.day.is_none() || Some(entry.day) == args.day {
                    results.push((entry.day, (entry.solve)()));
                }
            }
        }
        2023 => {
            for entry in entries2023.iter() {
                if args.day.is_none() || Some(entry.day) == args.day {
                    results.push((entry.day, (entry.solve)()));
                }
            }
        }
        2024 => {
            for entry in entries2024.iter() {
                if args.day.is_none() || Some(entry.day) == args.day {
                    results.push((entry.day, (entry.solve)(args.full, args.part)));
                }
            }
        }
        2025 => {
            for entry in entries2025.iter() {
                if args.day.is_none() || Some(entry.day) == args.day {
                    results.push((entry.day, (entry.solve)(args.full, args.part)));
                }
            }
        }
        _ => {}
    }

    results.sort_by_key(|&(day, _)| day);

    for (day, result) in results {
        println!("Day {}: {}", day, result);
    }

    Ok(())
}
