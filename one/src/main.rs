use error::DayOneError;
use std::fs;

mod error;

const INPUT_FILE: &str = "input2.txt";

const DIAL_START: i128 = 50;
const DIAL_MAX: i128 = 99;

const DIAL_START_INDEX: usize = 50;

#[derive(Debug)]
enum Direction {
    L,
    R,
}

#[derive(Debug)]
struct Combo {
    direction: Direction,
    amount: i128,
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect(&format!("file {INPUT_FILE} not found!"));

    let _ = match part_one(&input) {
        Ok(counter) => {
            println!("part one completed with {counter}");
        }
        Err(error) => {
            eprintln!("part one failed with {error}");
        }
    };

    let _ = match part_two(&input) {
        Ok(counter) => {
            println!("part two completed with {counter}");
        }
        Err(error) => {
            eprintln!("part two failed with {error}");
        }
    };
}

fn part_two(input: &str) -> Result<i32, DayOneError> {
    let mut counter = 0;
    let dial = Vec::from_iter(0..100);

    let _ = input
        .lines()
        .map(|line| {
            let characters = line.as_bytes();

            let direction = match characters
                .get(0)
                .ok_or_else(|| DayOneError::LineNotLongEnough(line.into()))?
            {
                b'R' => Direction::R,
                b'L' => Direction::L,
                _ => panic!("invalid direction {:?}", characters[0]),
            };

            if characters.len() < 2 {
                return Err(DayOneError::LineNotLongEnough(line.into()));
            }

            let amount = str::from_utf8(&characters[1..])
                .map_err(|e| DayOneError::Utf8Error(e, line.into()))?;
            let amount = amount
                .parse()
                .map_err(|e| DayOneError::ParseIntError(e, line.into()))?;

            Ok(Combo { direction, amount })
        })
        .collect::<Result<Vec<Combo>, DayOneError>>()?
        .iter()
        .fold(DIAL_START_INDEX, |dial_index, combo| {
            let new_index = match combo.direction {
                Direction::L => {
                    let mut curr_index = dial_index;

                    for _ in 0..combo.amount {
                        if curr_index == 0 {
                            curr_index = DIAL_MAX as usize;
                        } else {
                            curr_index -= 1;
                        }

                        if dial[curr_index as usize] == 0 {
                            counter += 1;
                        }
                    }

                    curr_index
                }
                Direction::R => {
                    let mut curr_index = dial_index;

                    for _ in 0..combo.amount {
                        curr_index += 1;

                        if curr_index > DIAL_MAX as usize {
                            curr_index = 0;
                        }

                        if dial[curr_index as usize] == 0 {
                            counter += 1;
                        }
                    }

                    curr_index
                }
            };

            new_index
        });

    Ok(counter)
}

fn part_one(input: &str) -> Result<i32, DayOneError> {
    let mut counter = 0;
    let _ = input
        .lines()
        .map(|line| {
            let characters = line.as_bytes();

            let direction = match characters
                .get(0)
                .ok_or_else(|| DayOneError::LineNotLongEnough(line.into()))?
            {
                b'R' => Direction::R,
                b'L' => Direction::L,
                _ => panic!("invalid direction {:?}", characters[0]),
            };

            if characters.len() < 2 {
                return Err(DayOneError::LineNotLongEnough(line.into()));
            }

            let amount = str::from_utf8(&characters[1..])
                .map_err(|e| DayOneError::Utf8Error(e, line.into()))?;

            let amount = amount
                .parse()
                .map_err(|e| DayOneError::ParseIntError(e, line.into()))?;

            Ok(Combo { direction, amount })
        })
        .collect::<Result<Vec<Combo>, DayOneError>>()?
        .iter()
        .fold(DIAL_START, |dial, combo| {
            let new_dial = match combo.direction {
                Direction::L => (dial - combo.amount).rem_euclid(DIAL_MAX + 1),
                Direction::R => (dial + combo.amount).rem_euclid(DIAL_MAX + 1),
            };

            if new_dial == 0 {
                counter += 1;
            }

            new_dial
        });

    Ok(counter)
}
