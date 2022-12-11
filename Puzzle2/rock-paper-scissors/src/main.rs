use clap::Parser;
use rock_paper_scissors::game::game;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();
    let rounds = game::load_game(&args.filename).unwrap();
    let sum_points = rounds
        .iter()
        .fold(0, |accum, iter| accum + iter.round_points());
    println!("Sum points: {}", sum_points);
}
