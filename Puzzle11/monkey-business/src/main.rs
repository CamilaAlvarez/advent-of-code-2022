use clap::Parser;
use monkey_business::monkey_parser::parse_monkeys_from_string;
use std::fs;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    input: String,
}
// TODO: Too slow! might be able to optimize this
fn main() {
    const TOTAL_ROUNDS: u32 = 10000;
    let args = Args::parse();
    let monkey_content = fs::read_to_string(args.input).unwrap();
    let mut monkeys = parse_monkeys_from_string(monkey_content, TOTAL_ROUNDS);
    for _ in 0..TOTAL_ROUNDS {
        monkeys.execute_round();
    }

    let monkey_business = monkeys.monkey_business();
    println!("Monkey business: {}", monkey_business)
}
