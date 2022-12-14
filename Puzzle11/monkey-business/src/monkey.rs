use super::item_test::MonkeyTest;
use super::operation::{BinaryOperation, Item, LazyOperation};
use std::collections::VecDeque;

struct MovedItem {
    monkey_id: usize,
    item: Box<LazyOperation>,
}
pub struct Monkeys {
    monkeys: Vec<Monkey>,
    rounds: u32,
    total_rounds: u32,
}
#[derive(Debug, Clone)]
pub struct Monkey {
    monkey_id: usize,
    items: VecDeque<Box<LazyOperation>>,
    operation: BinaryOperation,
    test: MonkeyTest,
    inspected_items: u32,
    tests_divisible_by: Vec<Item>,
}

impl MovedItem {
    fn new(monkey_id: usize, item: Box<LazyOperation>) -> Self {
        Self { monkey_id, item }
    }
}
impl Monkey {
    pub fn new(
        monkey_id: usize,
        items: Vec<Item>,
        operation: BinaryOperation,
        test: MonkeyTest,
        tests_divisible_by: &Vec<Item>,
    ) -> Self {
        let mut lazy_items = VecDeque::new();
        for item in items.iter() {
            lazy_items.push_back(LazyOperation::create_number(*item, tests_divisible_by))
        }
        Self {
            monkey_id,
            items: lazy_items,
            operation,
            test,
            inspected_items: 0,
            tests_divisible_by: tests_divisible_by.clone(),
        }
    }
    fn execute_operation(&mut self) -> Option<MovedItem> {
        if let Some(item) = self.items.pop_front() {
            let new_value = self.operation.execute(&item, &self.tests_divisible_by);
            let next_monkey = self.test.execute(&new_value);
            self.inspected_items += 1;
            return Some(MovedItem::new(next_monkey, new_value));
        }
        None
    }
    fn add_new_item(&mut self, new_item: Box<LazyOperation>) {
        self.items.push_back(new_item)
    }
    pub fn monkey_id(&self) -> usize {
        self.monkey_id
    }
    pub fn inspected_items(&self) -> u32 {
        self.inspected_items
    }
}
impl Monkeys {
    pub fn new(total_rounds: u32, monkeys: Vec<Monkey>) -> Self {
        Self {
            monkeys,
            rounds: 0,
            total_rounds,
        }
    }
    pub fn execute_round(&mut self) {
        for i in 0..self.monkeys.len() {
            while let Some(moved_item) = self.monkeys[i].execute_operation() {
                assert!(
                    moved_item.monkey_id < self.monkeys.len(),
                    "Invalid monkey id"
                );
                self.monkeys[moved_item.monkey_id].add_new_item(moved_item.item);
            }
        }
        self.rounds += 1;
        if self.rounds % 1000 == 0 {
            println!("Rounds: {}", self.rounds);
        }
    }
    pub fn get_ordered_monkeys(&self) -> Vec<Monkey> {
        let mut ordered_monkeys = self.monkeys.clone();
        ordered_monkeys
            .sort_by(|monkey1, monkey2| monkey2.inspected_items.cmp(&monkey1.inspected_items));
        ordered_monkeys
    }
    pub fn total_rounds(&self) -> u32 {
        self.total_rounds
    }
    pub fn monkey_business(&self) -> u128 {
        let ordered_monkeys = self.get_ordered_monkeys();
        assert!(ordered_monkeys.len() >= 2, "Too few monkeys");
        ordered_monkeys[0].inspected_items as u128 * ordered_monkeys[1].inspected_items as u128
    }
}
