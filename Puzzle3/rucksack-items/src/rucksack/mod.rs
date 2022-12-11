mod item;
pub mod loader;

pub struct Rucksack {
    repeated_char: char,
    item_priority: u32,
}

impl Rucksack {
    pub fn new(rucksack_items: &str) -> Option<Self> {
        // todo: we're assuming all chars are ascii
        // todo: handle border cases
        let rucksack_size = rucksack_items.trim().len() / 2;
        let second_compartment = &rucksack_items.trim()[rucksack_size..];
        let chars = rucksack_items.trim().chars();
        let mut count = 0;
        for current_char in chars.into_iter() {
            if count >= rucksack_size {
                break;
            }
            if second_compartment.contains(current_char) {
                return Some(Self {
                    repeated_char: current_char,
                    item_priority: item::get_item_value(current_char),
                });
            }
            count += 1;
        }
        None
    }
    pub fn repeated_char(&self) -> char {
        self.repeated_char
    }
    pub fn points(&self) -> u32 {
        self.item_priority
    }
}

pub struct BadgeGroup {
    repeated_char: char,
    item_priority: u32,
}
impl BadgeGroup {
    pub fn new(elf1: &str, elf2: &str, elf3: &str) -> Option<Self> {
        for item in elf1.chars().into_iter() {
            if elf2.contains(item) && elf3.contains(item) {
                return Some(Self {
                    repeated_char: item,
                    item_priority: item::get_item_value(item),
                });
            }
        }
        None
    }
    pub fn repeated_char(&self) -> char {
        self.repeated_char
    }
    pub fn points(&self) -> u32 {
        self.item_priority
    }
}
