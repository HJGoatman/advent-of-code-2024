use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

#[derive(Debug)]
enum ParseEquationError {
    IncorrectFormat,
    ParseTestValueError(ParseIntError),
    ParseNumbersError(ParseIntError),
}

impl FromStr for Equation {
    type Err = ParseEquationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s.splitn(2, ": ").collect::<Vec<&str>>();
        if splits.len() != 2 {
            return Err(ParseEquationError::IncorrectFormat);
        }

        let test_value = splits[0]
            .parse()
            .map_err(ParseEquationError::ParseTestValueError)?;

        let numbers = splits[1]
            .split(' ')
            .map(|v| v.parse())
            .collect::<Result<Vec<u64>, ParseIntError>>()
            .map_err(ParseEquationError::ParseNumbersError)?;

        return Ok(Equation {
            test_value,
            numbers,
        });
    }
}

#[derive(Debug)]
enum BridgeRepairError {
    ParseError {
        line_number: u32,
        error: ParseEquationError,
    },
}

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenation,
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("should have been able to read the file")
}

fn main() -> Result<(), BridgeRepairError> {
    env_logger::init();

    let input = load_input();
    let equations = input
        .split('\n')
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .map(|(line_number, line)| {
            line.parse().map_err(|err| BridgeRepairError::ParseError {
                line_number: line_number as u32,
                error: err,
            })
        })
        .collect::<Result<Vec<Equation>, BridgeRepairError>>()?;

    equations.iter().for_each(|e| log::debug!("{:?}", e));

    const PART_1_OPERATORS: [Operator; 2] = [Operator::Add, Operator::Multiply];
    let total_calibration_result =
        calculate_total_calibration_result(&PART_1_OPERATORS, &equations);
    println!("{}", total_calibration_result);

    const PART_2_OPERATORS: [Operator; 3] =
        [Operator::Add, Operator::Multiply, Operator::Concatenation];
    let total_calibration_result =
        calculate_total_calibration_result(&PART_2_OPERATORS, &equations);
    println!("{}", total_calibration_result);

    Ok(())
}

fn calculate_total_calibration_result(operators: &[Operator], equations: &[Equation]) -> u64 {
    let possible_equations = equations
        .iter()
        .filter(|equation| is_possible(operators, equation))
        .collect::<Vec<&Equation>>();

    log::debug!("Num possible: {}", possible_equations.len());
    equations.iter().for_each(|e| log::debug!("{:?}", e));

    let total_calibration_result: u64 = possible_equations
        .iter()
        .map(|equation| equation.test_value)
        .sum();

    total_calibration_result
}

fn is_possible(operators: &[Operator], equation: &Equation) -> bool {
    return is_possible_recursive(operators, equation.test_value, None, &equation.numbers);
}

fn is_possible_recursive(
    operators: &[Operator],
    test_value: u64,
    current_value: Option<u64>,
    numbers: &[u64],
) -> bool {
    if numbers.is_empty() {
        if current_value == None {
            return false;
        }

        return test_value == current_value.unwrap();
    }

    if current_value == None {
        return is_possible_recursive(operators, test_value, Some(numbers[0]), &numbers[1..]);
    }

    let current = current_value.unwrap();
    if current > test_value {
        return false;
    }

    operators.into_iter().any(|op| {
        let new_value = apply(*op, current, numbers[0]);
        if new_value == None {
            return false;
        }

        return is_possible_recursive(
            operators,
            test_value,
            Some(new_value.unwrap()),
            &numbers[1..],
        );
    })
}

fn apply(op: Operator, a: u64, b: u64) -> Option<u64> {
    match op {
        Operator::Add => Some(a + b),
        Operator::Multiply => Some(a * b),
        Operator::Concatenation => {
            if a == 0 {
                return None;
            }

            let num_digits = b.ilog10() + 1;
            log::trace!("Num Digits of {}: {}", a, num_digits);
            const TEN: u64 = 10;
            a.checked_mul(TEN.checked_pow(num_digits)?)?.checked_add(b)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{apply, is_possible, Equation, Operator};

    #[test]
    fn test_apply() {
        let _ = env_logger::try_init();
        let actual = apply(Operator::Concatenation, 12345, 6789);
        let expected = Some(123456789);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_concat() {
        let actual = is_possible(
            &[Operator::Add, Operator::Multiply, Operator::Concatenation],
            &Equation {
                test_value: 123456789101112,
                numbers: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            },
        );

        assert!(actual)
    }

    #[test]
    fn test_concat_2() {
        let actual = apply(Operator::Concatenation, 1, 2);
        let expected = Some(12);

        assert_eq!(expected, actual);
    }
}
