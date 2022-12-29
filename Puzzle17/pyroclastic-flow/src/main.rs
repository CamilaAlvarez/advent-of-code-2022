use clap::Parser;
use pyroclastic_flow::cave::shape::RockShapeIterator;
use pyroclastic_flow::cave::Cave;
use pyroclastic_flow::jet::parser;
use std::fs;

#[derive(Debug, Parser)]
#[command(author, about, version)]
struct Args {
    #[arg(long, short)]
    input: String,
    #[arg(long, short)]
    max_iters: usize,
}
fn main() {
    let args = Args::parse();
    let filecontent = fs::read_to_string(args.input).unwrap();
    let jet_pattern = parser::parse_jet_pattern(filecontent);
    let rock_iterator = RockShapeIterator::new();
    let mut cave = Cave::new(jet_pattern, rock_iterator);
    for _ in 0..args.max_iters {
        cave.spawn_and_move_new_rock();
    }
    println!("Final cave height: {}", cave.height());
}
