use super::elf_work::AssignedSections;
use std::fs;

pub fn load_assignment_pairs(filename: &str) -> Vec<(AssignedSections, AssignedSections)> {
    let mut assignments = vec![];
    let content = fs::read_to_string(filename).unwrap();
    for line in content.lines() {
        if let Some(assignment_pair) = AssignedSections::parse_pair(line) {
            assignments.push(assignment_pair);
        }
    }
    assignments
}
