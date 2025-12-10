pub enum Direction {
    Left,
    Right,
}

pub struct Instruction {
    pub direction: Direction,
    pub steps: u32,
}
