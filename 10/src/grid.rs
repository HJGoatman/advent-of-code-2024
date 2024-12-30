use std::fmt::{Display, Write};
use std::str::FromStr;

#[derive(Clone)]
pub struct Grid<T> {
    values: Vec<T>,
    width: usize,
    length: usize,
}

#[derive(Debug)]
pub enum ParseGridError<ValError> {
    DifferingRowSizes { expected: usize, got: usize },
    ParseValError(ValError),
}

impl<T> Display for ParseGridError<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseGridError::DifferingRowSizes { expected, got } => {
                f.write_fmt(format_args!(
                    "got differing row sizes, expected {}, got {}",
                    expected, got,
                ))?;
            }
            ParseGridError::ParseValError(v) => {
                f.write_fmt(format_args!("ParseValError: {}", v))?;
            }
        }
        Ok(())
    }
}

impl<T> FromStr for Grid<T>
where
    T: TryFrom<char>,
{
    type Err = ParseGridError<T::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values: Vec<T> = Vec::new();
        let mut width = None;
        let mut length = 0;
        for line in s.split('\n').filter(|line| !line.is_empty()) {
            match width {
                Some(existing_width) => {
                    if line.len() != existing_width {
                        return Err(ParseGridError::DifferingRowSizes {
                            expected: existing_width,
                            got: line.len(),
                        });
                    }
                }
                None => {
                    width = Some(line.len());
                }
            }

            let mut row = line
                .chars()
                .map(|c| T::try_from(c))
                .collect::<Result<Vec<T>, T::Error>>()
                .map_err(ParseGridError::ParseValError)?;

            values.append(&mut row);
            length += 1;
        }

        Ok(Grid {
            values,
            width: width.unwrap(),
            length,
        })
    }
}

impl<T> Grid<T> {
    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        let index = self.calculate_index(x, y)?;
        self.values.get(index)
    }

    fn calculate_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 {
            return None;
        }
        if x >= self.width as i32 || y >= self.length as i32 {
            return None;
        }

        let index = y as usize * self.width + x as usize;
        Some(index)
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.length
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator {
            grid: self,
            cursor: (0, 0),
        }
    }

    pub fn map_elements<U>(&self, map_function: impl Fn(((usize, usize), &T)) -> U) -> Grid<U> {
        let values = (0..self.length)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .zip(self.values.iter())
            .map(map_function)
            .collect();

        Grid {
            values,
            width: self.width,
            length: self.length,
        }
    }

    pub(crate) fn set(&mut self, potential_x: i32, potential_y: i32, value: T) -> Option<()> {
        let index = self.calculate_index(potential_x, potential_y)?;

        self.values[index] = value;
        Some(())
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.get_width() as i32;
        let height = self.get_height() as i32;

        f.write_char('\n')?;

        for y in 0..height {
            for x in 0..width {
                let value = self.get(x, y).unwrap();
                f.write_fmt(format_args!("{}", value))?;

                if x == width - 1 {
                    f.write_char('\n')?;
                }
            }
        }

        Ok(())
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    cursor: (i32, i32),
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = ((i32, i32), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let grid_width = self.grid.get_width() as i32;
        let grid_height = self.grid.get_height() as i32;
        let (x, y) = self.cursor;

        if y >= grid_height {
            return None;
        }
        // Should panic here as iterator should only return None at end of iteration.
        let item = self.grid.get(x, y).unwrap();

        if x >= grid_width - 1 {
            self.cursor = (0, y + 1);
        } else {
            self.cursor = (x + 1, y);
        }

        Some(((x, y), item))
    }
}
