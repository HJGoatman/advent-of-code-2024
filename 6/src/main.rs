use std::collections::{BTreeSet, HashMap, HashSet};
use std::env;
use std::fmt::{Display, Write};
use std::fs;

mod grid;
use grid::Grid;

#[derive(Debug, Clone, Copy)]
enum MapKey {
    Guard(Direction),
    Empty,
    Obstruction,
}

impl TryFrom<char> for MapKey {
    type Error = ParseMapKeyError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(MapKey::Guard(Direction::Up)),
            '>' => Ok(MapKey::Guard(Direction::Right)),
            'v' => Ok(MapKey::Guard(Direction::Down)),
            '<' => Ok(MapKey::Guard(Direction::Left)),
            '.' => Ok(MapKey::Empty),
            '#' => Ok(MapKey::Obstruction),
            a => Err(ParseMapKeyError::InvalidChar(a)),
        }
    }
}

impl Display for MapKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            MapKey::Guard(Direction::Up) => '^',
            MapKey::Guard(Direction::Right) => '>',
            MapKey::Guard(Direction::Down) => 'v',
            MapKey::Guard(Direction::Left) => '<',
            MapKey::Empty => '.',
            MapKey::Obstruction => '#',
        };
        f.write_char(c)
    }
}

#[derive(Debug)]
enum ParseMapKeyError {
    InvalidChar(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    let area: Grid<MapKey> = input.parse().unwrap();
    log::debug!("{}", area);

    let (starting_position, guard) = area
        .iter()
        .find(|(_, key)| {
            if let MapKey::Guard(_) = key {
                return true;
            } else {
                return false;
            }
        })
        .unwrap();

    let starting_direction = match guard {
        MapKey::Guard(direction) => Ok(direction),
        _ => Err("Not a guard"),
    }
    .unwrap();

    log::debug!("{:?}: {:?}", starting_position, starting_direction);

    let visited_positions = get_visited_positions(&area, starting_position, *starting_direction);
    visited_positions
        .iter()
        .for_each(|a| log::debug!("{:?}", a));
    let num_unique_positions = visited_positions
        .iter()
        .map(|e| e.0)
        .collect::<HashSet<(i32, i32)>>()
        .len();
    println!("{}", num_unique_positions);

    let num_possible_obstacles = find_possible_obstacle_locations(&area, &visited_positions);
    println!("{}", num_possible_obstacles);
}

fn find_possible_obstacle_locations(grid: &Grid<MapKey>, visited_positions: &[GuardState]) -> u32 {
    let mut area: Grid<MapKey> = grid.clone();
    let mut attempted_obstacle_locations = HashSet::new();
    let mut num_possible_locations = 0;

    for i in 0..visited_positions.len() - 1 {
        let starting_position = visited_positions[i];
        let potential_obstacle_location = visited_positions[i + 1].0;

        if attempted_obstacle_locations.contains(&potential_obstacle_location) {
            continue;
        }

        let (potential_x, potential_y) = potential_obstacle_location;
        let existing_tile = grid.get(potential_x, potential_y).unwrap();

        area.set(potential_x, potential_y, MapKey::Obstruction);

        log::debug!("\n");
        if guard_will_get_stuck_in_a_loop(&area, starting_position) {
            log::debug!("Found!");
            num_possible_locations += 1;
        }

        area.set(potential_x, potential_y, *existing_tile);

        attempted_obstacle_locations.insert(potential_obstacle_location);
    }

    num_possible_locations
}

fn guard_will_get_stuck_in_a_loop(
    area: &Grid<MapKey>,
    starting_position: ((i32, i32), Direction),
) -> bool {
    let mut guard_walk = GuardWalk {
        current_position: starting_position,
        area: &area,
    };

    let mut visited_positions = HashSet::new();
    visited_positions.insert(starting_position);

    while let Some(position) = guard_walk.next() {
        if visited_positions.contains(&position) {
            return true;
        }

        visited_positions.insert(position);
    }

    false
}

fn get_visited_positions(
    area: &Grid<MapKey>,
    starting_position: (i32, i32),
    starting_direction: Direction,
) -> Vec<GuardState> {
    let mut visited_positions = Vec::new();
    visited_positions.push((starting_position, starting_direction));

    let mut guard_walk = GuardWalk {
        current_position: (starting_position, starting_direction),
        area,
    };

    while let Some((position, direction)) = guard_walk.next() {
        visited_positions.push((position, direction));
    }

    visited_positions
}

type GuardState = ((i32, i32), Direction);

struct GuardWalk<'a> {
    current_position: GuardState,
    area: &'a Grid<MapKey>,
}

impl<'a> Iterator for GuardWalk<'a> {
    type Item = GuardState;

    fn next(&mut self) -> Option<Self::Item> {
        let ((x, y), direction) = self.current_position;
        let (new_x, new_y) = match direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };

        let next_position = match self.area.get(new_x, new_y)? {
            MapKey::Obstruction => {
                let new_direction = turn_right(direction);
                ((x, y), new_direction)
            }
            _ => ((new_x, new_y), direction),
        };

        self.current_position = next_position;

        Some(next_position)
    }
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}
