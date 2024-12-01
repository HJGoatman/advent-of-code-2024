use env_logger;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::num::ParseIntError;

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() -> Result<(), ParseIntError> {
    env_logger::init();

    let input = load_input();
    let location_ids = input
        .split('\n')
        .filter(|line| line.len() > 0)
        .flat_map(|line| line.split("   ").map(|val| val.parse::<u64>()))
        .collect::<Result<Vec<u64>, ParseIntError>>()?;

    let mut left_list: Vec<u64> = location_ids
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, v)| v)
        .copied()
        .collect();

    let mut right_list: Vec<u64> = location_ids
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 != 0)
        .map(|(_, v)| v)
        .copied()
        .collect();

    left_list.sort();
    right_list.sort();

    let total_distance = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| left.max(right) - left.min(right))
        .sum::<u64>();

    println!("{}", total_distance);

    let occurances = build_occurances(&left_list, &right_list);

    let similarity_score: u64 = left_list
        .iter()
        .map(|location_id| location_id * occurances.get(location_id).unwrap())
        .sum();

    println!("{}", similarity_score);

    Ok(())
}

fn build_occurances(sorted_left_list: &[u64], sorted_right_list: &[u64]) -> HashMap<u64, u64> {
    let mut occurances = HashMap::new();

    let num_location_ids = sorted_left_list.len();
    let mut i = 0;
    let mut j = 0;
    let mut location_occuances = 0;
    while i < num_location_ids && j < num_location_ids {
        let left_location_id = sorted_left_list[i];
        let right_location_id = sorted_right_list[j];

        // Already scanned
        if i > 0 && left_location_id == sorted_left_list[i - 1] {
            i += 1;
            continue;
        }

        // Store value
        if left_location_id < right_location_id {
            occurances.insert(left_location_id, location_occuances);
            location_occuances = 0;
            i += 1;
            continue;
        }

        if left_location_id > right_location_id {
            j += 1;
            continue;
        }

        if left_location_id == right_location_id {
            location_occuances += 1;
            j += 1;
        }
    }

    for remaining_index in i..num_location_ids {
        occurances.insert(sorted_left_list[remaining_index], 0);
    }

    occurances
}
