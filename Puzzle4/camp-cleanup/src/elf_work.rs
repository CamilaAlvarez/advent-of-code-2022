pub struct AssignedSections {
    start_index: u32,
    end_index: u32,
}

impl AssignedSections {
    pub fn new(work_line: &str) -> Option<Self> {
        let split_pair = work_line.trim().split("-").collect::<Vec<_>>();
        if split_pair.len() < 2 {
            return None;
        }
        // todo: Check error
        Some(Self {
            start_index: split_pair[0].parse::<u32>().unwrap(),
            end_index: split_pair[1].parse::<u32>().unwrap(),
        })
    }
    pub fn fully_contains(&self, other: &AssignedSections) -> bool {
        self.start_index <= other.start_index && self.end_index >= other.end_index
    }
    pub fn overlaps(&self, other: &AssignedSections) -> bool {
        (self.start_index <= other.start_index
            && self.end_index <= other.end_index
            && self.end_index >= other.start_index)
            || (self.start_index >= other.start_index
                && self.end_index >= other.end_index
                && self.start_index <= other.end_index)
            || self.fully_contains(other)
            || other.fully_contains(self)
    }
    pub fn parse_pair(line: &str) -> Option<(AssignedSections, AssignedSections)> {
        let pair = line.trim().split(",").collect::<Vec<_>>();
        if pair.len() < 2 {
            return None;
        }
        if let Some(first_elf) = AssignedSections::new(pair[0]) {
            if let Some(second_elf) = AssignedSections::new(pair[1]) {
                return Some((first_elf, second_elf));
            }
        }
        None
    }
}
