use std::env;
use std::fs;
use std::num::TryFromIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Mul(u32, u32),
}
impl Instruction {
    fn eval(&self) -> u32 {
        match self {
            Instruction::Mul(a, b) => a * b,
        }
    }
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
}
impl Program {
    fn run(&self) -> u32 {
        self.instructions
            .iter()
            .map(|instruction| instruction.eval())
            .sum()
    }
}

#[derive(Debug)]
enum ParseProgramError {
    DigitError(TryFromIntError),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    Mul,
    Val(u32),
    LBracket,
    RBracket,
    Comma,
    Invalid,
}

fn tokenise(s: &str) -> Result<Vec<Token>, ParseProgramError> {
    let mut tokens = Vec::new();

    let chars: Vec<char> = s.chars().collect();

    let mut i = 0;
    while i < s.len() {
        if i + 3 < s.len() && chars[i..i + 3] == ['m', 'u', 'l'] {
            tokens.push(Token::Mul);
            i += 3;
            continue;
        }

        if chars[i] == '(' {
            tokens.push(Token::LBracket);
            i += 1;
            continue;
        }

        if chars[i] == ')' {
            tokens.push(Token::RBracket);
            i += 1;
            continue;
        }

        if chars[i] == ',' {
            tokens.push(Token::Comma);
            i += 1;
            continue;
        }

        if let Some(digit) = chars[i].to_digit(10) {
            let val: u32 = digit;
            tokens.push(Token::Val(val));
            i += 1;
            continue;
        }

        tokens.push(Token::Invalid);
        i += 1;
    }

    Ok(tokens)
}

fn syntax_analsyis(tokens: &[Token]) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    let mut i = 0;
    while i < tokens.len() {
        let token = tokens[i];
        if let Token::Mul = token {
            if let Some((instruction, used_tokens)) = parse_mul(tokens, i) {
                instructions.push(instruction);
                i += used_tokens;
            }
        }

        i += 1;
    }

    instructions
}

fn parse_mul(tokens: &[Token], start_index: usize) -> Option<(Instruction, usize)> {
    let mut i = 0;

    if tokens[start_index + i] != Token::Mul {
        return None;
    }

    log::trace!("got mul");

    i += 1;

    if tokens[start_index + i] != Token::LBracket {
        return None;
    }

    log::trace!("got l bracket");

    i += 1;

    let (left_val, token_size) = parse_val(tokens, start_index + i)?;

    log::trace!("got left val {}", left_val);
    i += token_size;

    if tokens[start_index + i] != Token::Comma {
        return None;
    }

    log::trace!("got comma");

    i += 1;

    let (right_val, token_size) = parse_val(tokens, start_index + i)?;

    log::trace!("got right val {}", right_val);

    i += token_size;

    if tokens[start_index + i] != Token::RBracket {
        return None;
    }

    log::trace!("got r bracket");

    Some((Instruction::Mul(left_val, right_val), i))
}

fn parse_val(tokens: &[Token], start_index: usize) -> Option<(u32, usize)> {
    let mut i = 0;

    let mut vals = Vec::new();
    while start_index + i < tokens.len() {
        let token = tokens[start_index + i];
        if let Token::Comma | Token::RBracket = token {
            let mut total = 0;

            for (k, v) in vals.iter().enumerate() {
                const TEN: u32 = 10;
                let power = TEN.pow((vals.len() - k - 1) as u32);
                total += v * power as u32;
            }

            return Some((total, i));
        }

        if i > 3 {
            return None;
        }

        if let Token::Val(val) = token {
            vals.push(val);
            i += 1;
            continue;
        }

        return None;
    }

    None
}

impl FromStr for Program {
    type Err = ParseProgramError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = tokenise(s)?;
        log::debug!("tokens: {:?}", tokens);

        let instructions = syntax_analsyis(&tokens);

        Ok(Program { instructions })
    }
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() -> Result<(), ParseProgramError> {
    env_logger::init();

    let input = load_input();
    let program: Program = input.parse()?;

    log::debug!("{:?}", program);

    let result = program.run();
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{parse_mul, parse_val, Instruction, Token};

    #[test]
    fn test_parse_val() {
        let tokens = vec![Token::Val(8), Token::Comma];
        let val = parse_val(&tokens, 0);
        assert_eq!(Some((8, 1)), val);

        let tokens = vec![Token::Val(8), Token::Val(4), Token::Comma];
        let val = parse_val(&tokens, 0);
        assert_eq!(Some((84, 2)), val);

        let tokens = vec![
            Token::Mul,
            Token::LBracket,
            Token::Val(2),
            Token::Comma,
            Token::Val(1),
            Token::RBracket,
        ];
        let val = parse_val(&tokens, 2);
        assert_eq!(Some((2, 1)), val);
    }

    #[test]
    fn test_parse_mul() {
        env_logger::init();
        let tokens = vec![
            Token::Mul,
            Token::LBracket,
            Token::Val(2),
            Token::Comma,
            Token::Val(1),
            Token::RBracket,
        ];
        let val = parse_mul(&tokens, 0);
        assert_eq!(Some((Instruction::Mul(2, 1), 5)), val);
    }
}
