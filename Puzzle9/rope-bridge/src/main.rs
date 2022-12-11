use clap::Parser;
use itertools::Itertools;
use rope_bridge::movement::MovementDirection;
use rope_bridge::rope::RopePosition;
use std::fs;

#[derive(Debug, Parser)]
#[command(version, about, author)]
struct Args {
    #[arg(long, short)]
    input: String,
    #[arg(long, short)]
    knots: usize,
}
fn main() {
    let args = Args::parse();
    let instructions = fs::read_to_string(args.input).unwrap();
    let directions = MovementDirection::parse_to_movement_list(instructions);
    let mut rope = RopePosition::new_with_knots(args.knots);
    for direction in directions.into_iter() {
        rope.move_rope(direction);
    }
    let movement_map = rope.movement_map();
    let unique_tail_positions = movement_map.iter().unique().collect::<Vec<_>>();
    println!("Unique positions for tail: {}", unique_tail_positions.len());
}
