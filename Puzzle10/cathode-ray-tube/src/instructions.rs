#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Noop,
    Addx(u8, i32),
}

impl Instruction {
    pub fn load_instructions(instructions_content: String, add_length: u8) -> Vec<Self> {
        let mut instructions = vec![];
        for instruction in instructions_content.lines() {
            let split_instruction = instruction.trim().split_whitespace().collect::<Vec<_>>();
            assert!(
                split_instruction.len() <= 2,
                "Invalid number of parameter for an instruction"
            );
            match split_instruction[0] {
                "noop" => instructions.push(Self::Noop),
                "addx" => {
                    // TODO: check if parsing failed
                    let value = split_instruction[1].parse::<i32>().unwrap();
                    instructions.push(Self::Addx(add_length, value))
                }
                _ => {}
            }
        }
        instructions
    }
}
