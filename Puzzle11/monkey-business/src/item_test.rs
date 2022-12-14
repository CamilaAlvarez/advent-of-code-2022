use super::operation::LazyOperation;

#[derive(Debug, Clone)]
pub struct MonkeyTest {
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
}
impl MonkeyTest {
    pub fn new(divisible_by: u64, if_true: usize, if_false: usize) -> Self {
        Self {
            divisible_by,
            if_true,
            if_false,
        }
    }

    pub fn execute(&self, value: &Box<LazyOperation>) -> usize {
        if value.is_divisible_by(self.divisible_by) {
            return self.if_true;
        }
        self.if_false
    }
    pub fn divisible_by(&self) -> u64 {
        self.divisible_by
    }
}
