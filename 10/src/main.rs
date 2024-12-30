mod grid;

use grid::Grid;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt::Display;
use std::fs;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
struct Height(u8);

#[derive(Debug)]
enum ParseHeightError {
    NotDigit,
    TooLarge,
}

impl TryFrom<char> for Height {
    type Error = ParseHeightError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let v = value
            .to_digit(10)
            .ok_or(ParseHeightError::NotDigit)?
            .try_into()
            .map_err(|_| ParseHeightError::TooLarge)?;
        Ok(Height(v))
    }
}

impl Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

enum Step {
    Up,
    Down,
    Left,
    Right,
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();

    let topographic_map: Grid<Height> = input.parse().unwrap();
    log::debug!("{}", topographic_map);

    let trailhead_scores = find_trailheads(&topographic_map);
    let combined_trailhead_scores: u32 = trailhead_scores
        .iter()
        .map(|score| score.len() as u32)
        .sum();
    log::debug!("{}", combined_trailhead_scores);
}

fn find_trailheads(map: &Grid<Height>) -> Vec<HashSet<Position>> {
    let mut cache: HashMap<Position, HashSet<Position>> = HashMap::new();

    map.iter()
        .filter(|(_, height)| **height == Height(0))
        .map(|((x, y), _)| Position { x, y })
        .map(|start| find_trailhead(map, &mut cache, start))
        .collect()
}

fn find_trailhead(
    map: &Grid<Height>,
    cache: &mut HashMap<Position, HashSet<Position>>,
    start: Position,
) -> HashSet<Position> {
    if let Some(positions) = cache.get(&start) {
        return positions.clone();
    }

    if let Some(Height(9)) = map.get(start.x, start.y) {
        let mut results = HashSet::new();

        results.insert(start);
        return results;
    }

    let results: HashSet<Position> = [Step::Up, Step::Down, Step::Left, Step::Right]
        .into_iter()
        .flat_map(|step| {
            if let Some(next_position) = try_take_step(map, start, step) {
                find_trailhead(map, cache, next_position)
            } else {
                HashSet::new()
            }
        })
        .collect();

    cache.insert(start, results.clone());
    results
}

fn try_take_step(map: &Grid<Height>, position: Position, step: Step) -> Option<Position> {
    let current_height = map.get(position.x, position.y)?;

    let next_position = match step {
        Step::Up => Position {
            x: position.x,
            y: position.y - 1,
        },
        Step::Down => Position {
            x: position.x,
            y: position.y + 1,
        },
        Step::Left => Position {
            x: position.x - 1,
            y: position.y,
        },
        Step::Right => Position {
            x: position.x + 1,
            y: position.y,
        },
    };

    let next_height = map.get(next_position.x, next_position.y)?;

    if current_height.0 > next_height.0 {
        return None;
    }

    let height_diff = next_height.0 - current_height.0;
    if height_diff != 1 {
        return None;
    }

    Some(next_position)
}
