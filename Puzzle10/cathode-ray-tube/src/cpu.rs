use super::instructions::Instruction;
pub struct CPU {
    register_value: i32,
    cycles: u32,
    operations_to_execute: Vec<Instruction>,
    pending_operation: Option<Instruction>,
    current_char: char,
    screen_row_width: u32,
}

impl CPU {
    pub fn new(mut operations: Vec<Instruction>, screen_row_width: u32) -> Self {
        operations.reverse();
        Self {
            register_value: 1,
            cycles: 0,
            operations_to_execute: operations,
            pending_operation: None,
            current_char: '#',
            screen_row_width,
        }
    }
    pub fn consume_next_operation(&mut self) {
        if let Some(instruction) = &self.pending_operation {
            self.execute(*instruction);
        } else {
            let next_operation = self.operations_to_execute.pop();
            if let Some(operation) = next_operation {
                self.execute(operation);
            }
        }
    }

    fn execute(&mut self, operation: Instruction) {
        match operation {
            Instruction::Noop => {
                self.pending_operation = self.operations_to_execute.pop();
            }
            Instruction::Addx(remaining_cycles, value) => {
                if remaining_cycles == 0 {
                    self.register_value += value;
                    if let Some(operation) = self.operations_to_execute.pop() {
                        self.pending_operation = match operation {
                            Instruction::Noop => None,
                            Instruction::Addx(cycles, value) => {
                                Some(Instruction::Addx(cycles - 1, value))
                            }
                        };
                    } else {
                        self.pending_operation = None;
                    }
                } else {
                    self.pending_operation = Some(Instruction::Addx(remaining_cycles - 1, value));
                }
            }
        };
        // We compute the character before the cycle ends
        self.current_char = self.character_to_draw(self.screen_row_width);
        self.cycles += 1;
    }
    pub fn cycles(&self) -> u32 {
        self.cycles
    }
    pub fn register_value(&self) -> i32 {
        self.register_value
    }
    fn character_to_draw(&self, row_size: u32) -> char {
        let character_position: i32 = (self.cycles % row_size) as i32;
        if character_position - 1 <= self.register_value
            && self.register_value <= character_position + 1
        {
            return '#';
        }
        '.'
    }
    pub fn current_char(&self) -> char {
        self.current_char
    }
}
