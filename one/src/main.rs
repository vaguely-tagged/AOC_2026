use std::fs;

use error::DayOneError;

mod error;

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

fn main() -> Result<(), DayOneError> {
    let _ = part_one()?;
    let _ = part_two()?;
    Ok(())
}

fn part_two() -> Result<(), DayOneError> {
    let mut counter = 0;
    let dial = Vec::from_iter(0..100);

    let input = fs::read_to_string("input2.txt")?;
    let _ = input
        .lines()
        .map(|line| {
            let characters = line.as_bytes();

            let direction = match characters[0] {
                b'R' => Direction::R,
                b'L' => Direction::L,
                _ => panic!("invalid direction {:?}", characters[0]),
            };

            let amount = str::from_utf8(&characters[1..])?;
            let amount = amount.parse()?;

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

    println!("PART TWO dial has hit zero {counter:?} times");
    Ok(())
}

fn part_one() -> Result<(), DayOneError> {
    let mut counter = 0;
    let input = fs::read_to_string("input.txt")?;
    let _ = input
        .lines()
        .map(|line| {
            let characters = line.as_bytes();

            let direction = match characters[0] {
                b'R' => Direction::R,
                b'L' => Direction::L,
                _ => panic!("invalid direction {:?}", characters[0]),
            };

            let amount = str::from_utf8(&characters[1..])?;
            let amount = amount.parse()?;

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

    println!("PART ONE: dial has hit zero {counter:?} times");
    Ok(())
}


