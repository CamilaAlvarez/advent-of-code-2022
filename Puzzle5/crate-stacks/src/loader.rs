use super::cargo_stacks::CargoStacks;
use super::instruction::Instruction;
use regex::Regex;

pub fn load_stacks(mut stacks_lines: Vec<&str>) -> Option<CargoStacks> {
    let stack_indexes = stacks_lines.pop();
    stacks_lines.reverse();
    if let Some(indexes) = stack_indexes {
        let number_stacks = indexes.trim().split_whitespace().collect::<Vec<_>>().len();
        let mut cargo_stacks = CargoStacks::new(number_stacks);
        for line in stacks_lines.iter() {
            for i in 0..number_stacks {
                let item = line[i * 4..(i * 4 + 3)].trim();
                if !item.is_empty() {
                    cargo_stacks.add_item_to_stack(i + 1, item.replace("[", "").replace("]", ""));
                }
            }
        }
        return Some(cargo_stacks);
    }
    None
}

pub fn load_instructions(instructions: Vec<&str>) -> Vec<Instruction> {
    let mut instructions_enum = vec![];
    let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    for instruction in instructions.iter() {
        let caps = re.captures(instruction).unwrap();
        let number = caps.get(1).map_or("", |m| m.as_str());
        let from_stack = caps.get(2).map_or("", |m| m.as_str());
        let to_stack = caps.get(3).map_or("", |m| m.as_str());
        instructions_enum.push(Instruction::Move(
            number.parse().unwrap(),
            from_stack.parse().unwrap(),
            to_stack.parse().unwrap(),
        ));
    }
    instructions_enum
}
