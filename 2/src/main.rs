use std::cmp::Ordering;
use std::env;
use std::fs;
use std::num::ParseIntError;

use itertools::Itertools;

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() -> Result<(), ParseIntError> {
    env_logger::init();

    let input = load_input();

    let reports = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split(" ").map(|level| level.parse()).collect())
        .collect::<Result<Vec<Vec<u64>>, ParseIntError>>()?;

    log::debug!("reports: {:?}", reports);

    let num_reports_safe: u64 = reports
        .iter()
        .fold(0, |acc, report| if is_safe(report) { acc + 1 } else { acc });

    println!("{}", num_reports_safe);

    let num_reports_safe_with_dampener = reports.iter().fold(0, |acc, report| {
        if report
            .iter()
            .combinations(report.len() - 1)
            .any(|report| is_safe(&report.into_iter().copied().collect::<Vec<u64>>()))
        {
            acc + 1
        } else {
            acc
        }
    });

    println!("{}", num_reports_safe_with_dampener);

    Ok(())
}

fn is_safe(report: &[u64]) -> bool {
    let orderings: Vec<Ordering> = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(a, b)| a.cmp(b))
        .collect();

    log::debug!("{:?}", orderings);

    let all_decreasing = orderings
        .iter()
        .copied()
        .all(|ordering| ordering == Ordering::Greater);
    let all_increasing = orderings
        .iter()
        .copied()
        .all(|ordering| ordering == Ordering::Less);

    if !all_increasing && !all_decreasing {
        return false;
    }

    let differences: Vec<u64> = if all_increasing {
        report
            .iter()
            .zip(report.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect()
    } else {
        report
            .iter()
            .zip(report.iter().skip(1))
            .map(|(a, b)| a - b)
            .collect()
    };

    log::debug!("{:?}", differences);

    differences.into_iter().all(|diff| (1..=3).contains(&diff))
}
