use std::{fs, path::Path};

pub fn solve() -> String {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let file_path = Path::new(project_root).join("inputs/day05/full.txt");

    let puzzle_input = fs::read_to_string(file_path).unwrap();

    let (stacks_input, steps) = match &puzzle_input.split("\n\n").collect::<Vec<&str>>()[..] {
        &[first, second, ..] => (first, second),
        _ => unreachable!(),
    };


    let mut stacks = parse_stacks(stacks_input);

    // run steps
    for step in steps.split("\n") {
        let (num_items, from_stack_id, to_stack_id) = match &step.split_whitespace().collect::<Vec<&str>>()[..] {
            &[_, second, _, fourth, _, sixth] => (second.parse::<usize>().unwrap(), fourth.parse::<usize>().unwrap(), sixth.parse::<usize>().unwrap()),
            _ => unreachable!(),
        };

        let mut items: Vec<u8> = Vec::new();

        for _ in 0..num_items {
            let item = stacks[from_stack_id-1].pop().expect("not enough items in stack.");
            items.push(item);
        }

        let mut i = items.len()-1;

        while i < items.len() {
            stacks[to_stack_id-1].push(items[i]);
            i = i.wrapping_sub(1);
        }
    }

    let mut result = String::new();
    for stack in stacks {
        let top_item = char::from(stack[stack.len()-1]);
        result.push(top_item);
    }

    result
}

fn parse_stacks(stacks_input: &str) -> Vec<Vec<u8>> {
    let stack_layers = stacks_input.split("\n")
        .collect::<Vec<&str>>();

    let num_stacks = stack_layers
        .last().unwrap()
        .split_whitespace()
        .last().unwrap()
        .parse::<usize>().unwrap();


    let mut stacks: Vec<Vec<u8>> = Vec::new();

    for _ in 0..num_stacks {
        stacks.push(Vec::new())
    }

    let mut i = num_stacks - 1;

    while i < num_stacks {
        let layer = stack_layers[i].as_bytes();

        let mut j = 3usize;
        let mut stack_i = 0;

        while j <= layer.len() {
            if layer[j-2] != b' ' {
                stacks[stack_i].push(layer[j-2]);
            }

            stack_i += 1;
            j += 4;
        }

        i = i.wrapping_sub(1);
    }

    stacks
}

inventory::submit! {
    crate::AocEntry2022 {
        year: 2022,
        day: 5,
        solve: solve,
    }
}
