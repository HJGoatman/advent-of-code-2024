use std::{
    collections::{HashMap, HashSet},
    env, fs,
    num::ParseIntError,
};

#[derive(Debug)]
enum ParseInputError {
    TooManyParts,
    ParseRuleError(ParseRuleError),
    ParseUpdatesError(ParseUpdateError),
}

#[derive(Debug)]
enum ParseRuleError {
    InvalidPageNumber(ParseIntError),
    InvalidFormat,
}

#[derive(Debug)]
enum ParseUpdateError {
    ParsePageNumberError(ParseIntError),
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

type PageNumber = u32;
type PageOrderingRules = HashMap<u32, HashSet<u32>>;

fn main() {
    env_logger::init();

    let input = load_input();

    let (page_ordering_rules, updates) = parse(&input).unwrap();
    let correctly_ordered_updates: Vec<&Vec<u32>> = updates
        .iter()
        .filter(|update| is_correct_order(&page_ordering_rules, update))
        .collect();

    log::debug!("{:?}", correctly_ordered_updates);

    let middle_page_numbers: Vec<u32> = correctly_ordered_updates
        .iter()
        .map(|update| update.get(((update.len() + 1) / 2) - 1).unwrap())
        .copied()
        .collect();

    let sum: u32 = middle_page_numbers.iter().sum();
    println!("{}", sum);
}

fn parse(input: &str) -> Result<(PageOrderingRules, Vec<Vec<PageNumber>>), ParseInputError> {
    let parts: Vec<&str> = input.split("\n\n").filter(|l| !l.is_empty()).collect();
    if parts.len() != 2 {
        return Err(ParseInputError::TooManyParts);
    }

    let page_ordering_rules = parse_page_ordering_rules(parts[0])?;
    let updates = parse_updates(parts[1]).map_err(ParseInputError::ParseUpdatesError)?;

    return Ok((page_ordering_rules, updates));
}

fn parse_page_ordering_rules(parts: &str) -> Result<PageOrderingRules, ParseInputError> {
    let rules: Vec<(PageNumber, PageNumber)> = parts
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(parse_rule)
        .collect::<Result<Vec<(PageNumber, PageNumber)>, ParseRuleError>>()
        .map_err(ParseInputError::ParseRuleError)?;

    let mut page_ordering_rules = HashMap::new();

    rules.into_iter().for_each(|(page_a, page_b)| {
        page_ordering_rules
            .entry(page_a)
            .and_modify(|pages: &mut HashSet<u32>| {
                pages.insert(page_b);
            })
            .or_insert_with(|| {
                let mut set = HashSet::new();
                set.insert(page_b);
                set
            });
    });

    return Ok(page_ordering_rules);
}

fn parse_rule(input: &str) -> Result<(PageNumber, PageNumber), ParseRuleError> {
    let values = input
        .split('|')
        .map(|v| v.parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()
        .map_err(ParseRuleError::InvalidPageNumber)?;

    if values.len() != 2 {
        return Err(ParseRuleError::InvalidFormat);
    }

    return Ok((values[0], values[1]));
}

fn parse_updates(parts: &str) -> Result<Vec<Vec<u32>>, ParseUpdateError> {
    parts
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(parse_update)
        .collect::<Result<Vec<Vec<u32>>, ParseIntError>>()
        .map_err(ParseUpdateError::ParsePageNumberError)
}

fn parse_update(line: &str) -> Result<Vec<u32>, ParseIntError> {
    line.split(',').map(|s| s.parse()).collect()
}

fn is_correct_order(page_ordering_rules: &PageOrderingRules, update: &[PageNumber]) -> bool {
    for i in 0..(update.len() - 1) {
        for j in i..update.len() {
            if !is_before(page_ordering_rules, update[i], update[j]) {
                return false;
            }
        }
    }

    true
}

fn is_before(
    page_ordering_rules: &PageOrderingRules,
    page_number_a: u32,
    page_number_b: u32,
) -> bool {
    match page_ordering_rules.get(&page_number_b) {
        Some(pages_after_b) => !pages_after_b.contains(&page_number_a),
        _ => true,
    }
}
