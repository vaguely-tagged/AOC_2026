use std::{error, fs};

fn main() -> Result<(), Box<dyn error::Error>> {
    let _ = part_one()?;
    let _ = part_two()?;
    Ok(())
}

fn part_one() -> Result<(), Box<dyn error::Error>> {
    let intput = fs::read_to_string("input.txt")?;

    let id_list: u64 = intput
        .trim()
        .split(',')
        .map(|id_range| {
            let id_pair: Vec<&str> = id_range.split('-').collect();

            let starting_id: u64 = id_pair[0].parse().unwrap();
            let ending_id: u64 = id_pair[1].parse().unwrap();

            let mut sum = 0;

            for curr_number in starting_id..=ending_id {
                let curr_number_as_str = curr_number.to_string();

                let first_half = &curr_number_as_str[0..curr_number_as_str.len() / 2];
                let seccond_half =
                    &curr_number_as_str[curr_number_as_str.len() / 2..curr_number_as_str.len()];

                if first_half == seccond_half {
                    sum += curr_number;
                }
            }

            sum
        })
        .collect::<Vec<u64>>()
        .iter()
        .sum();

    println!("PART ONE: {id_list}");
    Ok(())
}

fn part_two() -> Result<(), Box<dyn error::Error>> {
    let intput = fs::read_to_string("input.txt")?;

    let id_list: u64 = intput
        .trim()
        .split(',')
        .map(|id_range| {
            let id_pair: Vec<&str> = id_range.split('-').collect();

            let starting_id: u64 = id_pair[0].parse().unwrap();
            let ending_id: u64 = id_pair[1].parse().unwrap();

            let mut sum = 0;

            for curr_number in starting_id..=ending_id {
                let num_bytes = curr_number.to_string();

                let num_bytes = num_bytes.as_bytes();

                for chunk_size in 1..num_bytes.len() {
                    let chunks: Vec<&str> = num_bytes
                        .chunks(chunk_size)
                        .map(|chunk| str::from_utf8(chunk).unwrap())
                        .collect();

                    let first = chunks[0];
                    let is_equal = chunks.iter().all(|&item| item == first);

                    if is_equal {
                        sum += curr_number;
                        break;
                    }
                }
            }

            sum
        })
        .collect::<Vec<u64>>()
        .iter()
        .sum();

    println!("PART TWO: {id_list}");
    Ok(())
}
