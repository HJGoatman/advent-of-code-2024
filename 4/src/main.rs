mod grid;

use grid::Grid;
use itertools::Itertools;
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

    let word_locations = search_xmas(&grid);
    assert!(word_locations.iter().all(|word| word.len() == 4));
    let num_occurances = word_locations.len();

    let active_grid = create_active_grid(&grid, word_locations);
    log::debug!("{}", active_grid);
    println!("{}", num_occurances);

    let word_locations = search_x_mas(&grid);
    let num_occurances = word_locations.len();
    let active_grid = create_active_grid(&grid, word_locations);
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

fn search_xmas(grid: &Grid<Letter>) -> Vec<WordLocation> {
    const TARGET_WORD: [Letter; 4] = [Letter::X, Letter::M, Letter::A, Letter::S];

    let mut found_word_locations = Vec::new();
    let word_rotations = get_word_rotations(&TARGET_WORD);

    for word_rotation in word_rotations.iter() {
        let mut locations = search(
            grid,
            &TARGET_WORD
                .iter()
                .copied()
                .zip(word_rotation.iter().copied())
                .collect::<Vec<(Letter, (i32, i32))>>(),
        );
        found_word_locations.append(&mut locations);
    }

    found_word_locations
}

fn search_x_mas(grid: &Grid<Letter>) -> Vec<WordLocation> {
    const ENDS: [Letter; 2] = [Letter::M, Letter::S];
    let ends: Vec<Vec<&Letter>> = ENDS
        .iter()
        .permutations(2)
        .cartesian_product(ENDS.iter().permutations(2))
        .map(|(mut a, mut b)| {
            a.append(&mut b);
            a
        })
        .collect();

    let mut patterns = Vec::new();
    for end in ends.into_iter() {
        let mut pattern = vec![(Letter::A, (0, 0))];

        let end_positions = [(-1, -1), (1, 1), (1, -1), (-1, 1)];
        end.into_iter()
            .zip(end_positions.iter())
            .for_each(|(l, (x, y))| {
                pattern.push((*l, (*x, *y)));
            });

        patterns.push(pattern);
    }

    patterns
        .iter()
        .flat_map(|pattern| search(grid, pattern))
        .collect()
}

fn search(grid: &Grid<Letter>, pattern: &[(Letter, (i32, i32))]) -> Vec<WordLocation> {
    let mut found_word_locations: Vec<WordLocation> = Vec::new();

    for start_y in 0..grid.get_height() as i32 {
        for start_x in 0..grid.get_width() as i32 {
            let potential_match: Vec<(Letter, (i32, i32))> = pattern
                .iter()
                .map(|(v, (x_diff, y_diff))| (*v, (start_x + x_diff, start_y + y_diff)))
                .collect();

            let word_found =
                potential_match
                    .iter()
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
                    last_potential.1 .0,
                    last_potential.1 .1,
                );

                let word_location = potential_match
                    .into_iter()
                    .map(|(_, (x, y))| (x as usize, y as usize))
                    .collect();

                found_word_locations.push(word_location);
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

fn create_active_grid(grid: &Grid<Letter>, word_locations: Vec<WordLocation>) -> Grid<String> {
    let mut active_characters = HashSet::new();
    word_locations.iter().for_each(|word| {
        word.iter().for_each(|position| {
            active_characters.insert(position);
        })
    });
    

    grid.map_elements(
            |(position, v)| match &active_characters.contains(&position) {
                true => format!("{}", v),
                false => ".".to_string(),
            },
        )
}
