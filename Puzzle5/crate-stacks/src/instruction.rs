#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Move(usize, usize, usize),
}
