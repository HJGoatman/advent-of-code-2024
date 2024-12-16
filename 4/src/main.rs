mod grid;

use grid::Grid;
use std::collections::HashSet;
use std::env;
use std::fmt::{Display, Write};
use std::fs;

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    let grid: Grid<Letter> = input.parse().unwrap();

    log::debug!("{}", grid);

    let target_word = [Letter::X, Letter::M, Letter::A, Letter::S];

    let word_locations = search(&grid, &target_word);
    assert!(word_locations.iter().all(|word| word.len() == 4));
    let num_occurances = word_locations.len();

    let mut active_characters = HashSet::new();
    word_locations.iter().for_each(|word| {
        word.iter().for_each(|position| {
            active_characters.insert(position);
        })
    });
    let active_grid =
        grid.map_elements(
            |(position, v)| match &active_characters.contains(&position) {
                true => format!("{}", v),
                false => ".".to_string(),
            },
        );

    log::debug!("{}", active_grid);
    println!("{}", num_occurances);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Letter {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum ParseXMASError {
    InvalidCharacter(char),
}

impl Display for ParseXMASError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseXMASError::InvalidCharacter(c) => {
                f.write_fmt(format_args!("got invalid character: {}", c))?;
            }
        }
        Ok(())
    }
}

impl TryFrom<char> for Letter {
    type Error = ParseXMASError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Letter::X),
            'M' => Ok(Letter::M),
            'A' => Ok(Letter::A),
            'S' => Ok(Letter::S),
            _ => Err(ParseXMASError::InvalidCharacter(value)),
        }
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Letter::X => 'X',
            Letter::M => 'M',
            Letter::A => 'A',
            Letter::S => 'S',
        };
        f.write_char(char)
    }
}

type Position = (usize, usize);
type WordLocation = Vec<Position>;

fn search(grid: &Grid<Letter>, target_word: &[Letter]) -> Vec<WordLocation> {
    let word_rotations = get_word_rotations(target_word);

    let mut found_word_locations: Vec<WordLocation> = Vec::new();

    for word_rotation in word_rotations.iter() {
        for start_y in 0..grid.get_height() as i32 {
            for start_x in 0..grid.get_width() as i32 {
                let potential_match: Vec<(i32, i32)> = word_rotation
                    .iter()
                    .map(|(x_diff, y_diff)| (start_x + x_diff, start_y + y_diff))
                    .collect();

                let word_found =
                    target_word
                        .iter()
                        .zip(potential_match.iter())
                        .all(|(expected, (x, y))| match grid.get(*x, *y) {
                            Some(actual) => actual == expected,
                            None => false,
                        });

                if word_found {
                    let last_potential = potential_match.iter().last().unwrap();
                    log::trace!(
                        "Word found ({}, {}) => ({}, {})",
                        start_x,
                        start_y,
                        last_potential.0,
                        last_potential.1,
                    );

                    let word_location = potential_match
                        .into_iter()
                        .map(|(x, y)| (x as usize, y as usize))
                        .collect();

                    found_word_locations.push(word_location);
                }
            }
        }
    }

    found_word_locations
}

fn get_word_rotations(target_word: &[Letter]) -> Vec<Vec<(i32, i32)>> {
    const ROTATION_FACTORS: [(i32, i32); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    ROTATION_FACTORS
        .iter()
        .map(|(x, y)| {
            (0..target_word.len() as i32)
                .map(|i| (x * i, y * i))
                .collect()
        })
        .collect()
}
