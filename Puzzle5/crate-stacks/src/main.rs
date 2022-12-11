use crate_stacks::instruction::Instruction;
use crate_stacks::loader;
use std::fs;
fn main() {
    let instructions_string = fs::read_to_string("input.txt").unwrap();
    let mut stack_instructions = vec![];
    let mut instructions = vec![];
    let mut is_instruction = false;
    for line in instructions_string.lines() {
        if line.is_empty() {
            is_instruction = true;
            continue;
        }
        if is_instruction {
            instructions.push(line.trim());
        } else {
            stack_instructions.push(line);
        }
    }
    let stacks = loader::load_stacks(stack_instructions);

    if let Some(mut stacks) = stacks {
        let instructions = loader::load_instructions(instructions);
        for instruction in instructions.iter() {
            match instruction {
                Instruction::Move(number, from_stack, to_stack) => {
                    let mut moved_items = vec![];
                    for _ in 0..*number {
                        moved_items.push(stacks.pop_from_stack(*from_stack));
                    }
                    moved_items.reverse();
                    for item in moved_items.iter() {
                        stacks.add_item_to_stack(*to_stack, item.clone());
                    }
                }
            }
        }
        let top_items = stacks.peek();
        for item in top_items.iter() {
            print!("{}", item);
        }
        println!("");
    }
}
