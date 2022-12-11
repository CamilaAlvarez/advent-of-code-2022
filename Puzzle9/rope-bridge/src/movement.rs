#[derive(Debug, Clone, Copy)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

impl MovementDirection {
    pub fn parse_to_movement_list(instruction_set: String) -> Vec<MovementDirection> {
        let mut movements = vec![];
        for line in instruction_set.lines() {
            let direction_instruction = line.trim().split_whitespace().collect::<Vec<_>>();
            // TODO: Error check! What if instruction is incomplete
            // TODO: handle invalid number of steps
            let steps = direction_instruction[1].parse::<i32>().unwrap();
            let direction = match direction_instruction[0] {
                "U" => Self::Up,
                "D" => Self::Down,
                "L" => Self::Left,
                "R" => Self::Right,
                _ => Self::Up,
            };
            for _ in 0..steps {
                movements.push(direction);
            }
        }
        movements
    }
}
