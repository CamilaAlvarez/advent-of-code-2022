use clap::Parser;
use rucksack_items::rucksack::loader;

#[derive(Debug, Parser)]
#[command(author, version)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();
    let badgegroups = loader::load_badgegroup(&args.filename);
    let sum_priorities = badgegroups.iter().fold(0, |acc, item| acc + item.points());
    println!("Sum priorities: {}", sum_priorities);
}
