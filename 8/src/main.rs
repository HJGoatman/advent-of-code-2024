use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::i32;
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Frequency(char);

#[derive(Debug)]
struct Map {
    antennas: HashMap<Frequency, Vec<Position>>,
    width: u16,
    height: u16,
}

impl Map {
    fn find_antinode_locations(&self, min_harmonic: i32, max_harmonic: i32) -> HashSet<Position> {
        self.antennas
            .iter()
            .map(|(frequency, locations)| {
                (
                    frequency,
                    find_antinode_locations(
                        locations,
                        self.width,
                        self.height,
                        min_harmonic,
                        max_harmonic,
                    ),
                )
            })
            .inspect(|(frequency, locations)| log::trace!("{:?}: {:?}", frequency, locations))
            .map(|(_, locations)| locations)
            .flatten()
            .collect()
    }
}

fn find_antinode_locations(
    locations: &[Position],
    width: u16,
    height: u16,
    min_harmonic: i32,
    max_harmonic: i32,
) -> HashSet<Position> {
    let mut antinode_locations = HashSet::new();

    for i in 0..locations.len() {
        for j in 0..locations.len() {
            if i == j {
                continue;
            }

            let antenna_a_position = locations[i];
            let antenna_b_position = locations[j];

            let antenna_a_x = antenna_a_position.x as i32;
            let antenna_b_x = antenna_b_position.x as i32;
            let antenna_a_y = antenna_a_position.y as i32;
            let antenna_b_y = antenna_b_position.y as i32;

            let delta_x = antenna_b_x - antenna_a_x;
            let delta_y = antenna_b_y - antenna_a_y;

            for harmonic in min_harmonic..max_harmonic {
                let antinode_x = antenna_a_x + harmonic * delta_x;
                let antinode_y = antenna_a_y + harmonic * delta_y;

                if antinode_x < 0
                    || antinode_y < 0
                    || antinode_x >= width as i32
                    || antinode_y >= height as i32
                {
                    break;
                }
                let antinode_position = Position {
                    x: antinode_x as u16,
                    y: antinode_y as u16,
                };

                log::trace!(
                    "{:?} -> {:?} => {:?}",
                    antenna_a_position,
                    antenna_b_position,
                    antinode_position
                );

                antinode_locations.insert(antinode_position);
            }
        }
    }

    antinode_locations
}

#[derive(Debug)]
enum ParseMapError {
    UnequalLineLenghts,
}

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut antennas = HashMap::new();

        let mut width = None;
        let mut x = 0;
        let mut y = 0;

        for c in s.chars() {
            match c {
                '\n' => {
                    if let Some(existing_width) = width {
                        if x != existing_width {
                            return Err(ParseMapError::UnequalLineLenghts);
                        }
                    } else {
                        width = Some(x);
                    }

                    y += 1;
                    x = 0;
                    continue;
                }
                '.' => {}
                frequency => {
                    antennas
                        .entry(Frequency(frequency))
                        .and_modify(|positions: &mut Vec<Position>| {
                            positions.push(Position { x, y });
                        })
                        .or_insert_with(|| vec![Position { x, y }]);
                }
            };

            x += 1;
        }

        let height = y + 1;

        Ok(Map {
            antennas,
            width: width.unwrap(),
            height,
        })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position {
    x: u16,
    y: u16,
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    log::debug!("\n{}", input);

    let map: Map = input.parse().unwrap();
    log::debug!("{:?}", map);

    let antinode_locations = map.find_antinode_locations(2, 3);
    log::debug!("{:?}", antinode_locations);
    let num_antinode_locations = antinode_locations.len();
    println!("{}", num_antinode_locations);

    let antinode_locations = map.find_antinode_locations(1, i32::MAX);
    log::debug!("{:?}", antinode_locations);
    let num_antinode_locations = antinode_locations.len();
    println!("{}", num_antinode_locations);
}
