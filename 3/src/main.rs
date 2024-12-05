use std::env;
use std::fs;
use std::num::TryFromIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

struct Interpreter1 {}

impl Interpreter1 {
    fn run(&self, program: &Program) -> u32 {
        program
            .instructions
            .iter()
            .map(|instruction| match instruction {
                Instruction::Mul(a, b) => a * b,
                Instruction::Do => 0,
                Instruction::Dont => 0,
            })
            .sum()
    }
}

struct Interpreter2 {
    instructions_enabled: bool,
}

impl Interpreter2 {
    fn run(&mut self, program: &Program) -> u32 {
        let mut total = 0;

        for instruction in program.instructions.iter() {
            match instruction {
                Instruction::Mul(a, b) => {
                    if self.instructions_enabled {
                        total += a * b;
                    }
                }
                Instruction::Do => {
                    self.instructions_enabled = true;
                }
                Instruction::Dont => {
                    self.instructions_enabled = false;
                }
            }
        }

        total
    }
}

#[derive(Debug)]
enum ParseProgramError {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    Mul,
    Val(u32),
    LBracket,
    RBracket,
    Comma,
    Invalid,
    Do,
    Dont,
}

fn tokenise(s: &str) -> Result<Vec<Token>, ParseProgramError> {
    let mut tokens = Vec::new();

    let chars: Vec<char> = s.chars().collect();

    let mut i = 0;
    while i < s.len() {
        let mul_chars = ['m', 'u', 'l'];
        if i + mul_chars.len() < s.len() && chars[i..i + mul_chars.len()] == mul_chars {
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

        let do_chars = ['d', 'o', '(', ')'];
        if i + do_chars.len() < s.len() && chars[i..i + do_chars.len()] == do_chars {
            tokens.push(Token::Do);
            i += do_chars.len();
            continue;
        }

        let dont_chars = ['d', 'o', 'n', '\'', 't', '(', ')'];
        if i + dont_chars.len() < s.len() && chars[i..i + dont_chars.len()] == dont_chars {
            tokens.push(Token::Dont);
            i += dont_chars.len();
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

        if let Token::Do = token {
            instructions.push(Instruction::Do);
        }

        if let Token::Dont = token {
            instructions.push(Instruction::Dont);
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

    let interpreter1 = Interpreter1 {};

    let result = interpreter1.run(&program);
    println!("{}", result);

    let mut interpreter2 = Interpreter2 {
        instructions_enabled: true,
    };
    let result2 = interpreter2.run(&program);
    println!("{}", result2);
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
