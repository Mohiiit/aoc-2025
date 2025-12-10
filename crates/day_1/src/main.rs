use std::fs;

fn move_right(current_dial: &mut u32, instruction: u32, answer: &mut u32) {
    *current_dial += instruction % 100;
    *current_dial %= 100;
    *answer += (*current_dial == 0) as u32;
}

fn move_left(current_dial: &mut u32, instruction: u32, answer: &mut u32) {
    *current_dial = (*current_dial + 100 - instruction % 100) % 100;
    *answer += (*current_dial == 0) as u32;
}

fn solve_part_one() {
    let file_path = "crates/day_1/input.txt";
    let file_content = fs::read_to_string(file_path).expect("issue while reading the input file");
    let lines: Vec<String> = file_content
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    let mut current_dial: u32 = 50;
    let mut answer: u32 = 0;

    for instruction in &lines {
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

fn move_right_second(current_dial: &mut u32, instruction: u32, answer: &mut u32) {
    let temp_current = *current_dial;
    *current_dial += instruction % 100;
    *current_dial %= 100;
    // *answer += (*current_dial==0) as u32;
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

fn solve_part_two() {
    let file_path = "crates/day_1/input.txt";
    let file_content = fs::read_to_string(file_path).expect("issue while reading the input file");
    let lines: Vec<String> = file_content
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    let mut current_dial: u32 = 50;
    let mut answer: u32 = 0;

    for instruction in &lines {
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
fn main() {
    println!("Hello, day1!");
    solve_part_two();
}
