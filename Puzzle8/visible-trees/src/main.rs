use clap::Parser;
use std::fs;
use visible_trees::trees::TreeHeightMap;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    input: String,
}
fn main() {
    let args = Args::parse();
    // TODO: error check
    let input_file = fs::read_to_string(args.input).unwrap();
    let height_map = TreeHeightMap::new(input_file);
    let visible_trees_map = height_map.visible_tree_map();
    let number_visible_trees = visible_trees_map.iter().fold(0, |acc, row| {
        acc + row
            .iter()
            .fold(0, |acc, has_tree| if *has_tree { acc + 1 } else { acc })
    });
    println!("Visible trees: {}", number_visible_trees);
    let tree_score_map = height_map.get_scores_map();
    let highest_score = tree_score_map
        .iter()
        .max_by(|x, y| x.iter().max().cmp(&y.iter().max()))
        .unwrap()
        .iter()
        .max();
    println!("Highest score: {}", highest_score.unwrap());
}
