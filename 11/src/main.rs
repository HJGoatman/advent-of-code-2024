use core::num;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::ops::Div;

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    let stones = input
        .split(" ")
        .map(|s| s.parse())
        .collect::<Result<Vec<u64>, ParseIntError>>()
        .unwrap();

    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();
    const NUM_BLINKS: u64 = 25;
    let num_stones = stones.iter().fold(0, |acc, stone| {
        acc + count_stones_after_blinking(&mut cache, *stone, NUM_BLINKS)
    });
    println!("{}", num_stones);
}

fn count_stones_after_blinking(
    cache: &mut HashMap<(u64, u64), u64>,
    stone: u64,
    blinks_remaining: u64,
) -> u64 {
    if let Some(result) = cache.get(&(stone, blinks_remaining)) {
        return *result;
    }

    if blinks_remaining == 0 {
        return 1;
    }

    if stone == 0 {
        let result = count_stones_after_blinking(cache, 1, blinks_remaining - 1);
        cache.insert((stone, blinks_remaining), result);
        return result;
    }

    let num_digits = stone.ilog10() + 1;
    let has_even_number_of_digits = num_digits % 2 == 0;
    if has_even_number_of_digits {
        let (first_stone, second_stone) = split_integer(stone, num_digits);
        let result = count_stones_after_blinking(cache, first_stone, blinks_remaining - 1)
            + count_stones_after_blinking(cache, second_stone, blinks_remaining - 1);
        cache.insert((stone, blinks_remaining), result);
        return result;
    }

    let result = count_stones_after_blinking(cache, stone * 2024, blinks_remaining - 1);
    cache.insert((stone, blinks_remaining), result);
    return result;
}

fn split_integer(a: u64, num_digits: u32) -> (u64, u64) {
    assert!(num_digits % 2 == 0);
    const TEN: u64 = 10;
    let divisor = TEN.pow(num_digits.div(2));
    let first_part = a.div(divisor);
    let second_part = a % divisor;

    return (first_part, second_part);
}

#[cfg(test)]
mod tests {
    use crate::split_integer;

    #[test]
    fn test_split() {
        assert_eq!((1, 2), split_integer(12, 2));
        assert_eq!((12345, 67890), split_integer(1234567890, 10));
    }
}
