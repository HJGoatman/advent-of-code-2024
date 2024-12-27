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
    ParseError(ParseEquationError),
}

enum Operator {
    Add,
    Multiply,
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() -> Result<(), BridgeRepairError> {
    env_logger::init();

    let input = load_input();
    let equations = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.parse())
        .collect::<Result<Vec<Equation>, ParseEquationError>>()
        .map_err(BridgeRepairError::ParseError)?;

    equations.iter().for_each(|e| log::debug!("{:?}", e));

    let possible_equations = equations
        .iter()
        .filter(|equation| is_possible(equation))
        .collect::<Vec<&Equation>>();

    let total_calibration_result: u64 = possible_equations
        .iter()
        .map(|equation| equation.test_value)
        .sum();
    println!("{}", total_calibration_result);

    Ok(())
}

fn is_possible(equation: &Equation) -> bool {
    return is_possible_recursive(equation.test_value, None, &equation.numbers);
}

fn is_possible_recursive(test_value: u64, current_value: Option<u64>, numbers: &[u64]) -> bool {
    if numbers.is_empty() {
        if current_value == None {
            return false;
        }

        return test_value == current_value.unwrap();
    }

    if current_value == None {
        return is_possible_recursive(test_value, Some(numbers[0]), &numbers[1..]);
    }

    let current = current_value.unwrap();

    const OPERATORS: [Operator; 2] = [Operator::Add, Operator::Multiply];
    OPERATORS.into_iter().any(|op| {
        let new_value = apply(op, current, numbers[0]);
        return is_possible_recursive(test_value, Some(new_value), &numbers[1..]);
    })
}

fn apply(op: Operator, a: u64, b: u64) -> u64 {
    match op {
        Operator::Add => a + b,
        Operator::Multiply => a * b,
    }
}
