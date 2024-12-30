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

trait TrailheadScore: Default + Clone + From<Position> {
    fn combine(&mut self, other: Self) -> Self;
    fn get_score(&self) -> u32;
}

#[derive(Default, Clone)]
struct OriginalScoring {
    set: HashSet<Position>,
}

impl TrailheadScore for OriginalScoring {
    fn combine(&mut self, other: Self) -> Self {
        let combined: HashSet<Position> = self.set.union(&other.set).copied().collect();
        OriginalScoring { set: combined }
    }

    fn get_score(&self) -> u32 {
        self.set.len() as u32
    }
}

impl From<Position> for OriginalScoring {
    fn from(value: Position) -> Self {
        let mut results = HashSet::new();

        results.insert(value);
        OriginalScoring { set: results }
    }
}

#[derive(Default, Clone)]
struct TrailheadRating {
    value: u32,
}

impl TrailheadScore for TrailheadRating {
    fn combine(&mut self, other: Self) -> Self {
        TrailheadRating {
            value: self.value + other.value,
        }
    }

    fn get_score(&self) -> u32 {
        self.value
    }
}

impl From<Position> for TrailheadRating {
    fn from(_: Position) -> Self {
        TrailheadRating { value: 1 }
    }
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

    let trailhead_scores: Vec<OriginalScoring> = find_trailheads(&topographic_map);
    let combined_trailhead_scores: u32 = trailhead_scores
        .iter()
        .map(|scoring| scoring.get_score())
        .sum();
    println!("{}", combined_trailhead_scores);

    let trailhead_scores: Vec<TrailheadRating> = find_trailheads(&topographic_map);
    let combined_trailhead_scores: u32 = trailhead_scores
        .iter()
        .map(|scoring| scoring.get_score())
        .sum();
    println!("{}", combined_trailhead_scores);
}

fn find_trailheads<T: TrailheadScore>(map: &Grid<Height>) -> Vec<T> {
    let mut cache: HashMap<Position, T> = HashMap::new();

    map.iter()
        .filter(|(_, height)| **height == Height(0))
        .map(|((x, y), _)| Position { x, y })
        .map(|start| find_trailhead(map, &mut cache, start))
        .collect()
}

fn find_trailhead<T: TrailheadScore>(
    map: &Grid<Height>,
    cache: &mut HashMap<Position, T>,
    start: Position,
) -> T {
    if let Some(positions) = cache.get(&start) {
        return positions.clone();
    }

    if let Some(Height(9)) = map.get(start.x, start.y) {
        return T::from(start);
    }

    let results: T = [Step::Up, Step::Down, Step::Left, Step::Right]
        .into_iter()
        .map(|step| {
            if let Some(next_position) = try_take_step(map, start, step) {
                find_trailhead(map, cache, next_position)
            } else {
                T::default()
            }
        })
        .fold(T::default(), |mut acc, x| acc.combine(x));

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
