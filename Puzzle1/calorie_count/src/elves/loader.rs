use super::elf::Elf;
use super::errors;
use std::fs;

pub fn load_elves_from_file(filepath: &str) -> Result<Vec<Elf>, errors::ElfError> {
    let mut elves = vec![];
    let elves_file = fs::read_to_string(filepath)?;
    let mut elf = Elf::new();
    for line in elves_file.lines() {
        if line.trim().is_empty() {
            elves.push(elf);
            elf = Elf::new();
            continue;
        }
        let calories: u32 = line.trim().parse()?;
        elf.add_calories(calories);
    }
    Ok(elves)
}
