pub struct Elf {
    carried_calories: u32,
}
impl Elf {
    pub fn new() -> Self {
        Self {
            carried_calories: 0,
        }
    }
    pub fn add_calories(&mut self, calories: u32) {
        self.carried_calories += calories;
    }
    // u32 implements copy
    pub fn calories(&self) -> u32 {
        self.carried_calories
    }
}
