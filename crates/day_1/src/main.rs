use std::env;
use std::fs;
use thiserror::Error;

enum Part {
    First,
    Second,
}
#[derive(Debug, Error)]
enum PartError {
    #[error("Invalid Part: {0}")]
    Invalid(String),
    #[error("Missing Part")]
    Missing,
}

#[derive(Debug, Error)]
enum InputPathError {
    #[error("Missing Input Path")]
    Missing,
    #[error("Unable to read input from: {0}")]
    UnableToReadFrom(String),
    #[error("Parse Error: {0}")]
    ParseError(InstructionParseError),
}

#[derive(Debug, Error)]
enum InstructionParseError {
    #[error("Wrong Direction given: {0}")]
    WrongDirection(char),
    #[error("No Direction given")]
    NoDirection,
    #[error("Steps parsing failed")]
    StepsParsingFailed,
}

enum Direction {
    Left,
    Right,
}

struct Instruction {
    direction: Direction,
    steps: u32,
}
fn parse_part(arg: Option<String>) -> Result<Part, PartError> {
    match arg.as_deref() {
        Some("1") => Ok(Part::First),
        Some("2") => Ok(Part::Second),
        Some(random) => Err(PartError::Invalid(random.to_string())),
        None => Err(PartError::Missing),
    }
}

fn parse_instruction(line: &str) -> Result<Instruction, InstructionParseError> {
    let first_char = line.chars().next();
    let direction = match first_char {
        Some('L') => Direction::Left,
        Some('R') => Direction::Right,
        Some(random) => return Err(InstructionParseError::WrongDirection(random)),
        None => return Err(InstructionParseError::NoDirection),
    };
    let steps = line[1..]
        .parse()
        .map_err(|_| InstructionParseError::StepsParsingFailed)?;
    Ok(Instruction { direction, steps })
}

fn parse_input_path(arg: Option<String>) -> Result<Vec<Instruction>, InputPathError> {
    match arg.as_deref() {
        Some(path) => {
            let content = fs::read_to_string(path.to_string())
                .map_err(|_| InputPathError::UnableToReadFrom(path.to_string()))?;
            content
                .lines()
                .map(|line| parse_instruction(line.trim()))
                .collect::<Result<Vec<_>, _>>()
                .map_err(InputPathError::ParseError)
        }
        None => Err(InputPathError::Missing),
    }
}

fn move_right(current_dial: &mut u32, instruction: u32, answer: &mut u32) {
    *current_dial += instruction % 100;
    *current_dial %= 100;
    *answer += (*current_dial == 0) as u32;
}

fn move_left(current_dial: &mut u32, instruction: u32, answer: &mut u32) {
    *current_dial = (*current_dial + 100 - instruction % 100) % 100;
    *answer += (*current_dial == 0) as u32;
}

fn move_right_second(current_dial: &mut u32, instruction: u32, answer: &mut u32) {
    let temp_current = *current_dial;
    *current_dial += instruction % 100;
    *current_dial %= 100;
    *answer += instruction / 100;
    if temp_current > *current_dial {
        *answer += 1;
    }
}

fn move_left_second(current_dial: &mut u32, instruction: u32, answer: &mut u32) {
    let steps = instruction % 100;
    *answer += instruction / 100;
    if steps >= *current_dial && *current_dial != 0 {
        *answer += 1;
    }
    *current_dial = (*current_dial + 100 - steps) % 100;
}
const DIAL_START: u32 = 50;

fn solve_part_one(instructions: &Vec<Instruction>) -> u32 {
    let mut current_dial = DIAL_START;
    let mut answer = 0;

    instructions
        .iter()
        .for_each(|instruction| match instruction.direction {
            Direction::Left => move_left(&mut current_dial, instruction.steps, &mut answer),
            Direction::Right => move_right(&mut current_dial, instruction.steps, &mut answer),
        });
    answer
}

fn solve_part_two(instructions: &Vec<Instruction>) -> u32 {
    let mut current_dial = DIAL_START;
    let mut answer = 0;

    instructions
        .iter()
        .for_each(|instruction| match instruction.direction {
            Direction::Left => move_left_second(&mut current_dial, instruction.steps, &mut answer),
            Direction::Right => {
                move_right_second(&mut current_dial, instruction.steps, &mut answer)
            }
        });
    answer
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inputs: Vec<String> = env::args().collect();
    let part = parse_part(inputs.get(1).cloned())?;
    let instructions = parse_input_path(inputs.get(2).cloned())?;
    let answer = match part {
        Part::First => solve_part_one(&instructions),
        Part::Second => solve_part_two(&instructions),
    };
    println!("answer is: {:?}", answer);
    Ok(())
}
