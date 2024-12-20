use std::collections::HashSet;
use std::env;
use std::fmt::{Display, Pointer, Write};
use std::fs;

mod grid;
use grid::Grid;

#[derive(Debug)]
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

#[derive(Debug, Clone, Copy)]
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

    if let Some((starting_position, MapKey::Guard(starting_direction))) =
        area.iter().find(|(_, key)| {
            if let MapKey::Guard(_) = key {
                return true;
            } else {
                return false;
            }
        })
    {
        log::debug!("{:?}: {:?}", starting_position, starting_direction);

        let mut visited_positions = HashSet::new();

        let mut position = starting_position;
        let mut direction = *starting_direction;
        loop {
            visited_positions.insert(position);
            if let Some((new_position, new_direction)) =
                get_guard_next_position(&area, position, direction)
            {
                position = new_position;
                direction = new_direction;
            } else {
                break;
            }
        }

        println!("{}", visited_positions.len())
    }
}

fn get_guard_next_position(
    area: &Grid<MapKey>,
    (x, y): (i32, i32),
    direction: Direction,
) -> Option<((i32, i32), Direction)> {
    let (new_x, new_y) = match direction {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    };

    match area.get(new_x, new_y)? {
        MapKey::Obstruction => {
            let new_direction = turn_right(direction);
            Some(((x, y), new_direction))
        }
        _ => Some(((new_x, new_y), direction)),
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
