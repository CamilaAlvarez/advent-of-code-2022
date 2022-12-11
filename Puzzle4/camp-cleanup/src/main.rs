use camp_cleanup::loader;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}
fn main() {
    let args = Args::parse();
    let pairs = loader::load_assignment_pairs(&args.filename);
    let fully_contained_pairs = pairs
        .iter()
        .filter(|pair| pair.0.overlaps(&pair.1))
        .collect::<Vec<_>>();
    println!("Number overlap ranges: {}", fully_contained_pairs.len());
}
