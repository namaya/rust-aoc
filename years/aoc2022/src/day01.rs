
use std::{fs, path::Path};

pub fn solve() -> String {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let input_path = Path::new(project_root).join("inputs/day01/full.txt");

    let contents = fs::read_to_string(input_path)
        .expect("Error reading file...");

    let mut elf_calories : Vec<u32> = vec![0];

    for line in contents.split("\n") {
        if let Ok(num_calories) = line.parse::<u32>() {
            let cur_calories = elf_calories.last_mut()
                .expect("something went wrong..");

            *cur_calories += num_calories;
        } else {
            elf_calories.push(0);
        }
    }

    // get max value
    let mut max_values: [u32; 3] = [0, 0, 0];

    for num_calories in elf_calories {
        for value in max_values {
            if num_calories > value {
                // replace smallest value
                let mut min_value = u32::MAX;
                let mut min_i = 0;

                for (i, value) in max_values.iter().enumerate() {
                    if *value < min_value {
                        min_value = *value;
                        min_i = i;
                    }
                }

                max_values[min_i] = num_calories;
                break;
            }

        }
    }

    let top3: u32 = max_values.iter().sum();

    return top3.to_string();
}

inventory::submit! {
    crate::AocEntry2022 {
        year: 2022,
        day: 1,
        solve: solve,
    }
}
