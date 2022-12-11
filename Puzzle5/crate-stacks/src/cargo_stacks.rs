pub struct CargoStacks {
    stacks: Vec<Vec<String>>,
}

impl CargoStacks {
    pub fn new(num_stacks: usize) -> Self {
        let mut stacks = Vec::with_capacity(num_stacks);
        for _ in 0..num_stacks {
            stacks.push(vec![]);
        }
        Self { stacks }
    }
    pub fn add_item_to_stack(&mut self, index: usize, item: String) {
        assert!(index > 0, "Invalid index");
        self.stacks[index - 1].push(item);
    }
    // Todo: error checking
    pub fn move_item(&mut self, from: usize, to: usize) {
        assert!(from > 0 && to > 0, "Invalid position, from");
        if let Some(moved_item) = self.stacks[from - 1].pop() {
            self.stacks[to - 1].push(moved_item);
        }
    }
    pub fn peek(&self) -> Vec<String> {
        let mut top_stacks = Vec::with_capacity(self.stacks.len());
        for stack in self.stacks.iter() {
            if stack.is_empty() {
                top_stacks.push("_".to_string());
            } else {
                top_stacks.push(stack.last().unwrap().clone());
            }
        }
        top_stacks
    }
    pub fn pop_from_stack(&mut self, index: usize) -> String {
        assert!(index > 0, "Invalid index");
        if let Some(moved_item) = self.stacks[index - 1].pop() {
            return moved_item;
        }
        "_".to_string()
    }
}
