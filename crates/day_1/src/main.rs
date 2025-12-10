use std::fs;
use std::env;

enum Part {
    First,
    Second
}
#[derive(Debug)]
enum PartError {
    Invalid(String),
    Missing
}

#[derive(Debug)]
enum InputPathError {
    Missing,
    UnableToReadFrom(String),
    ParseError(InstructionParseError)
}

#[derive(Debug)]
enum InstructionParseError {
    WrongDirection(char),
    NoDirection,
    StepsParsingFailed
}

enum Direction {
    Left, Right
}

struct Instruction {
    direction: Direction,
    steps: u32
}
fn parse_part(arg: Option<String>) -> Result<Part, PartError> {
    match arg.as_deref() {
        Some("1") => Ok(Part::First),
        Some("2") => Ok(Part::Second),
        Some(random) => Err(PartError::Invalid(random.to_string())),
        None => Err(PartError::Missing)
    }
}

fn parse_instruction(line: &str) -> Result<Instruction, InstructionParseError> {
    let first_char = line.chars().next();
    let direction = match first_char {
        Some('L') => Direction::Left,
        Some('R') => Direction::Right,
        Some(random) => return Err(InstructionParseError::WrongDirection(random)),
        None => return Err(InstructionParseError::NoDirection)
    };
    let steps: u32 = line[1..].parse().map_err(|_| Err(InstructionParseError::StepsParsingFailed))?;
    Ok(Instruction {
        direction,
        steps
    })
}

fn parse_input_path(arg: Option<String>) -> Result<Vec<Instruction>, InputPathError> {
    match arg.as_deref() {
        Some(path) => {
            let content = fs::read_to_string(path.to_string()).map_err(|_| Err(InputPathError::UnableToReadFrom(path.to_string())))?;
            content
                .lines()
                .map(|line| parse_instruction(line.trim()))
                .collect::<Result<Vec<_>, _>>()
                .map_err(InputPathError::ParseError)
        },
        None => Err(InputPathError::Missing)
    }
}



struct first_part;
struct second_part;

trait solution {
    fn solve(input: Vec<String>);
}

impl solution for first_part {
    fn solve(input: Vec<String>) {
        let mut current_dial= DIAL_START;
        let mut answer: u32 = 0;

        for instruction in &input {
            if let Some(first_char) = instruction.chars().next() {
                let number_part = &instruction[1..];
                let number: u32 = number_part.parse().unwrap();
                match first_char {
                    'L' => move_left(&mut current_dial, number, &mut answer),
                    'R' => move_right(&mut current_dial, number, &mut answer),
                    _ => unreachable!("input should have only L and R"),
                }
            }
        }
        println!("1st part answer is: {:?}", answer);
    }
}

impl solution for second_part {
    fn solve(input: Vec<String>) {
        let mut current_dial= DIAL_START;
        let mut answer: u32 = 0;

        for instruction in &input {
            if let Some(first_char) = instruction.chars().next() {
                let number_part = &instruction[1..];
                let number: u32 = number_part.parse().unwrap();
                match first_char {
                    'L' => move_left_second(&mut current_dial, number, &mut answer),
                    'R' => move_right_second(&mut current_dial, number, &mut answer),
                    _ => unreachable!("input should have only L and R"),
                }
            }
        }
        println!("2nd part answer is: {:?}", answer);
    }
}
fn take_input() -> Vec<String> {
    let file_path = env::args().nth(2);
    let file_path = match file_path {
        Some(file_path) => {
            println!("reading file: {:?}", file_path);
            file_path
        },
        None => {
            panic!("no input path file given")
        }
    };
    let file_content = fs::read_to_string(file_path).expect("issue while reading the input file");
    let lines: Vec<String> = file_content
        .lines()
        .map(|line| line.trim().to_string())
        .collect();
    lines
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
    *answer += (instruction / 100);
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


fn main() {
    let part = env::args().nth(1);
    let input = take_input();
    match part.as_deref() {
        Some("1") => first_part::solve(input),
        Some("2") => second_part::solve(input),
        _ => eprintln!("usage: cargo run -- <1|2>"),
    }
}
