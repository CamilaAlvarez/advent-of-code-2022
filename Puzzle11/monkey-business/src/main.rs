use clap::Parser;
use monkey_business::monkey_parser::parse_monkeys_from_string;
use std::fs;
use std::time::Instant;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    input: String,
}
// TODO: Too slow! might be able to optimize this
fn main() {
    const TOTAL_ROUNDS: u32 = 10000;
    const CAN_REDUCE_WORRY: bool = false;
    let args = Args::parse();
    let monkey_content = fs::read_to_string(args.input).unwrap();
    let mut monkeys = parse_monkeys_from_string(monkey_content, TOTAL_ROUNDS, CAN_REDUCE_WORRY);
    for i in 0..TOTAL_ROUNDS {
        let start = Instant::now();
        monkeys.execute_round();
        let duration = start.elapsed();
        println!("Time elapsed in round {} is: {:?}", i, duration);
    }

    let monkey_business = monkeys.get_ordered_monkeys();
    for monkey in monkey_business.iter() {
        println!(
            "Monkey {} has inspected {} items",
            monkey.monkey_id(),
            monkey.inspected_items()
        );
    }
}
