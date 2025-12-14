use std::collections::BTreeSet;
use std::error::Error;
use std::time::Instant;
use std::{env, fs};
use thiserror::Error;
use utils::{Part, parse_part};

#[derive(Debug, Error)]
enum FileParseError {
    #[error("no input file provided, user must provide one")]
    NoInputFileProvided,
    #[error("unable to read the file, maybe the path if wrong: {0}")]
    UnableToReadFile(String),
    #[error("parser error: {0}")]
    LineParsingError(LineParsingIssue),
}

#[derive(Debug, Error)]
enum LineParsingIssue {
    #[error("usize parsing failed on number: {0}")]
    UsizeParsingIssue(String),
}

#[derive(Debug)]
struct Range {
    pub start: u64,
    pub end: u64,
}

fn parse_the_line(line: String) -> Result<Vec<Range>, LineParsingIssue> {
    line.split(',')
        .map(|entry| {
            let parts: Vec<_> = entry.split('-').map(|each_value| each_value).collect();

            let start = parts[0]
                .parse::<u64>()
                .map_err(|_| LineParsingIssue::UsizeParsingIssue(parts[0].to_string()))?;
            let end = parts[1]
                .parse::<u64>()
                .map_err(|_| LineParsingIssue::UsizeParsingIssue(parts[1].to_string()))?;
            Ok(Range { start, end })
        })
        .collect()
}

fn parse_file(input: Option<String>) -> Result<Vec<Range>, FileParseError> {
    match input.as_deref() {
        Some(path) => {
            let content = fs::read_to_string(path.to_string())
                .map_err(|_| FileParseError::UnableToReadFile(path.to_string()))?;
            parse_the_line(content).map_err(FileParseError::LineParsingError)
        }
        None => Err(FileParseError::NoInputFileProvided),
    }
}

fn find_power(mut num: u64) -> u32 {
    let mut ans: u32 = 0;
    while num / 10 > 0 {
        ans += 1;
        num /= 10;
    }
    ans
}


struct PartOne;
struct PartTwo;

trait Solver {
    fn generate_candidates(&self) -> BTreeSet<u64>;
    fn solve(&self, range: &[Range]) -> u64 {
        let candidates = self.generate_candidates();
        range
            .iter()
            .map(|range| candidates.range(range.start..=range.end).sum::<u64>())
            .sum::<u64>()
    }
}
impl Solver for PartOne {
    fn generate_candidates(&self) -> BTreeSet<u64> {
        let mut candidates = BTreeSet::new();
        for base in 1..=70000 {
            let power = find_power(base.clone());
            let candidate = (base as u64 * (10_u64.pow(power + 1))) + (base as u64);
            candidates.insert(candidate as u64);
        }
        candidates
    }
}

impl Solver for PartTwo {
    fn generate_candidates(&self) -> BTreeSet<u64> {
        let mut candidates = BTreeSet::new();
        let max_num = 10_u64.pow(10);
        for base in 1..=70000 {
            let power_factor: u32 = find_power(base) + 1;
            let mut changing_value = base;
            while changing_value < max_num {
                changing_value = (changing_value * (10_u64.pow(power_factor))) + base;
                candidates.insert(changing_value);
            }
        }
        candidates
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let input: Vec<String> = env::args().collect();
    let part = parse_part(input.get(1).cloned())?;
    let question_input = parse_file(input.get(2).cloned())?;
    let solver: Box<dyn Solver> = match part {
        Part::First => Box::new(PartOne),
        Part::Second => Box::new(PartTwo),
    };
    let ans = solver.solve(&question_input);
    println!("solution for the part: {:?} is: {:?}", part, ans);
    let time_taken = Instant::now() - start_time;
    println!("time taken by the code: {:?}", time_taken);
    Ok(())
}
