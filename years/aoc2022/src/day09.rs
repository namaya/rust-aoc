
use std::{fs, path::Path};
use std::collections::HashSet;

enum Motion {
    Right(u32),
    Left(u32),
    Up(u32),
    Down(u32)
}

pub fn solve() -> String {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let input_path = Path::new(project_root).join("inputs/day09/full.txt");

    let contents = fs::read_to_string(input_path)
        .expect("Error reading file...");

    const N: u32 = 10;

    let mut knots_pos: Vec<(i32, i32)> = (0..N).map(|_| (0, 0)).collect(); 
    let mut visited = HashSet::from([(0,0)]);

    contents
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|motion| match &motion.split_whitespace().collect::<Vec<&str>>()[..] {
                &[first, second, ..] => match first {
                    "U" => Motion::Up(second.parse::<u32>().expect("nsteps not uint32")),
                    "R" => Motion::Right(second.parse::<u32>().expect("nsteps not uint32")),
                    "D" => Motion::Down(second.parse::<u32>().expect("nsteps not uint32")),
                    "L" => Motion::Left(second.parse::<u32>().expect("nsteps not uint32")),
                    _ => unreachable!()
                },
                _ => unreachable!()
        })
        .for_each(|motion| {
            match motion {
                Motion::Up(nsteps) => {
                    for _i in 0..nsteps {
                        let len = knots_pos.len();
                        knots_pos[len-1].1 += 1;

                        for i in (0..knots_pos.len()-1).rev() {
                            if knots_pos[i+1].1 - knots_pos[i].1 >= 2 {
                                knots_pos[i].1 += 1;
                                knots_pos[i].0 += knots_pos[i+1].0 - knots_pos[i].0;
                            }
                        }

                        visited.insert(knots_pos[0]);
                    }
                }
                Motion::Right(nsteps) => {
                    for _i in 0..nsteps {
                        let len = knots_pos.len();
                        knots_pos[len-1].0 += 1;

                        for i in (0..knots_pos.len()-1).rev() {
                            if knots_pos[i+1].0 - knots_pos[i].0 >= 2 {
                                knots_pos[i].0 += 1;
                                knots_pos[i].1 += knots_pos[i+1].1 - knots_pos[i].1;
                            }
                        }

                        visited.insert(knots_pos[0]);
                    }
                },
                Motion::Down(nsteps) => {
                    for _i in 0..nsteps {
                        let len = knots_pos.len();
                        knots_pos[len-1].1 -= 1;

                        for i in (0..knots_pos.len()-1).rev() {
                            if knots_pos[i+1].1 - knots_pos[i].1 <= -2 {
                                knots_pos[i].1 -= 1;
                                knots_pos[i].0 += knots_pos[i+1].0 - knots_pos[i].0;
                            }
                        }

                        visited.insert(knots_pos[0]);
                    }
                },
                Motion::Left(nsteps) => {
                    for _i in 0..nsteps {
                        let len = knots_pos.len();
                        knots_pos[len-1].0 -= 1;

                        for i in (0..knots_pos.len()-1).rev() {
                            if knots_pos[i+1].0 - knots_pos[i].0 <= -2 {
                                knots_pos[i].0 -= 1;
                                knots_pos[i].1 += knots_pos[i+1].1 - knots_pos[i].1;
                            }
                        }

                        visited.insert(knots_pos[0]);
                    }
                }
            }

        });
    
    return visited.len().to_string();
}

inventory::submit! {
    crate::AocEntry2022 {
        year: 2022,
        day: 9,
        solve: solve,
    }
}
