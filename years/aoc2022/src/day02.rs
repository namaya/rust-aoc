use std::collections::HashMap;
use std::{fs, path::Path};

pub fn solve() -> String {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let input_path = Path::new(project_root).join("inputs/day02/full.txt");

    let strategy_guide = fs::read_to_string(input_path)
        .expect("Error reading input file...");

    let opponent_action_encodings = HashMap::from([
        ("A", 1),
        ("B", 2),
        ("C", 3)
    ]);

    // from part 1
    // let player_action_encodings = HashMap::from([
    //     ("X", 1),
    //     ("Y", 2),
    //     ("Z", 3)
    // ]);

    let player_action_encodings = HashMap::from([
        ("X", [0, 3, 1, 2]),
        ("Y", [0, 1, 2, 3]),
        ("Z", [0, 2, 3, 1])
    ]);


    let action_rules = [
        [0, 0, 0, 0],
        [0, 3, 0, 6],
        [0, 6, 3, 0],
        [0, 0, 6, 3],
    ];

    let mut overall_score = 0;

    for line in strategy_guide.split("\n") {
        let inputs: Vec<&str> = line.split(" ").collect();

        let desired_round_outcome = inputs[1];
        let opp_action_score = opponent_action_encodings[inputs[0]];

        let player_action_score = player_action_encodings[desired_round_outcome][opp_action_score];

        let round_outcome = action_rules[player_action_score][opp_action_score];
        let player_score = player_action_score + round_outcome;

        overall_score += player_score;
    }

    return overall_score.to_string();
}

inventory::submit! {
    crate::AocEntry2022 {
        year: 2022,
        day: 2,
        solve: solve,
    }
}
