use super::item_test::MonkeyTest;
use super::monkey::{Monkey, Monkeys};
use super::operation::{BinaryOperation, Item};
use regex::Regex;

pub fn parse_monkeys_from_string(monkeys_content: String, max_rounds: u32) -> Monkeys {
    let mut monkeys = vec![];
    let mut monkey_data = vec![];
    let mut tests_divisible_by = vec![];
    let copied_lines = monkeys_content.lines().clone();
    for line in copied_lines {
        if line.trim().starts_with("Test: divisible by ") {
            let divisible_by = get_divisble_by(&line.to_string());
            tests_divisible_by.push(divisible_by);
        }
    }
    for line in monkeys_content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        monkey_data.push(line.trim());
        create_monkey(&mut monkey_data, &mut monkeys, &tests_divisible_by);
    }
    create_monkey(&mut monkey_data, &mut monkeys, &tests_divisible_by);
    Monkeys::new(max_rounds, monkeys)
}

fn create_monkey(
    monkey_data: &mut Vec<&str>,
    monkeys: &mut Vec<Monkey>,
    tests_divisible_by: &Vec<u64>,
) {
    const LINES_PER_MONKEY: usize = 6;
    if monkey_data.len() == LINES_PER_MONKEY {
        let mut test_lines = vec![];
        test_lines.push(monkey_data[3].to_string());
        test_lines.push(monkey_data[4].to_string());
        test_lines.push(monkey_data[5].to_string());
        monkeys.push(Monkey::new(
            parse_monkey_id(monkey_data[0].to_string()),
            parse_items(monkey_data[1].to_string()),
            parse_operation(monkey_data[2].to_string()),
            parse_test(test_lines),
            tests_divisible_by,
        ));
        monkey_data.clear();
    }
}
fn parse_monkey_id(monkey_id_line: String) -> usize {
    // TODO: error check
    let monkey_re = Regex::new(r"Monkey ([0-9]+):").unwrap();
    let capture_monkey_id = monkey_re.captures(&monkey_id_line).unwrap();
    capture_monkey_id
        .get(1)
        .map(|x| x.as_str().to_string().trim().parse::<usize>().unwrap())
        .unwrap()
}
fn parse_operation(operation_str: String) -> BinaryOperation {
    let trimmed_line = operation_str.trim().replace("Operation:", "");
    let items_split = trimmed_line.split_whitespace().collect::<Vec<_>>();
    assert!(items_split.len() == 5, "Invalid operation");
    // the last three items are the operation
    BinaryOperation::new(
        items_split[2].to_string(),
        items_split[3].to_string(),
        items_split[4].to_string(),
    )
}
// TODO: error check
fn parse_test(test_lines: Vec<String>) -> MonkeyTest {
    assert!(test_lines.len() == 3, "Invalid test");
    let capture = get_divisble_by(&test_lines[0]);
    let if_true_re = Regex::new(r"If true: throw to monkey ([0-9]+)").unwrap();
    let capture_if_true = if_true_re.captures(&test_lines[1]).unwrap();
    let if_true = capture_if_true
        .get(1)
        .map(|x| x.as_str().to_string().parse::<usize>().unwrap())
        .unwrap();
    let if_false_re = Regex::new(r"If false: throw to monkey ([0-9]+)").unwrap();
    let capture_if_false = if_false_re.captures(&test_lines[2]).unwrap();
    let if_false = capture_if_false
        .get(1)
        .map(|x| x.as_str().to_string().parse::<usize>().unwrap())
        .unwrap();

    MonkeyTest::new(capture, if_true, if_false)
}

fn get_divisble_by(test_line: &String) -> u64 {
    let monkey_test_re = Regex::new(r"Test: divisible by ([0-9]+)").unwrap();
    let capture_divisible = monkey_test_re.captures(test_line).unwrap();
    let capture = capture_divisible
        .get(1)
        .map(|x| x.as_str().to_string().parse::<Item>().unwrap())
        .unwrap();
    capture
}
fn parse_items(items_line: String) -> Vec<Item> {
    let mut items = vec![];
    let trimmed_line = items_line.trim().replace("Starting items:", "");
    let items_split = trimmed_line.split(",");
    for item in items_split {
        // TODO: check if value is valid to be parsed
        items.push(item.trim().parse::<Item>().unwrap());
    }
    items
}
