mod error;
mod types;

use std::env;
use std::fs;

use error::{InputPathError, InstructionParseError};
use types::{Direction, Instruction};
use utils::{Part, parse_part};

const DIAL_START: u32 = 50;

pub trait Rotater {
    fn move_left(&self, current_pos: u32, steps: u32) -> (u32, u32);
    fn move_right(&self, current_pos: u32, steps: u32) -> (u32, u32);
}

pub struct Part1Way;
pub struct Part2Way;

impl Rotater for Part1Way {
    fn move_left(&self, current_pos: u32, steps: u32) -> (u32, u32) {
        let new_pos = (current_pos + 100 - steps % 100) % 100;
        (new_pos, (new_pos == 0) as u32)
    }
    fn move_right(&self, current_pos: u32, steps: u32) -> (u32, u32) {
        let new_pos = (current_pos + steps % 100) % 100;
        (new_pos, (new_pos == 0) as u32)
    }
}

impl Rotater for Part2Way {
    fn move_left(&self, current_pos: u32, steps: u32) -> (u32, u32) {
        let rem = steps % 100;
        let mut answer = steps / 100;

        if current_pos != 0 && rem >= current_pos {
            answer += 1;
        }
        ((current_pos + 100 - rem) % 100, answer)
    }
    fn move_right(&self, current_pos: u32, steps: u32) -> (u32, u32) {
        let temp_current = current_pos;
        let new_pos = (current_pos + steps % 100) % 100;
        let mut answer = steps / 100;
        if temp_current > new_pos {
            answer += 1;
        }
        (new_pos, answer)
    }
}

pub trait Solver {
    fn solve(&self, instructions: &[Instruction]) -> u32;
}

impl<S: Rotater> Solver for S {
    fn solve(&self, instructions: &[Instruction]) -> u32 {
        let mut current_dial = DIAL_START;
        let mut answer = 0;

        for instruction in instructions {
            let (new_pos, count) = match instruction.direction {
                Direction::Left => self.move_left(current_dial, instruction.steps),
                Direction::Right => self.move_right(current_dial, instruction.steps),
            };
            current_dial = new_pos;
            answer += count;
        }
        answer
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inputs: Vec<String> = env::args().collect();
    let part = parse_part(inputs.get(1).cloned())?;
    let instructions = parse_input_path(inputs.get(2).cloned())?;

    let solver: Box<dyn Solver> = match part {
        Part::First => Box::new(Part1Way),
        Part::Second => Box::new(Part2Way),
    };
    let answer = solver.solve(&instructions);
    println!("answer is: {:?}", answer);
    Ok(())
}
