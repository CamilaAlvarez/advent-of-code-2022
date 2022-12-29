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

// We found that after going though a number of jet "pushes" we fall into a pattern,
// so we don't have to iterate all billion times we just need to find the pattern
// For example, for the test input, after 98 jet pushes, we move 35 rocks every 200 pushes, incresing the height by 53 every 35 rocks.
// And we know that by the end of the 98th (20th rock) push the height was 36. With that we know:
// - After 2022th rock: 36 + 57*53 + height generated by 7 rocks (2) = 3059
// - After the 1billion rock: 36 + 28571428570*53 + height generated by 30 rocks (which is 42) = 1514285714288
fn main() {
    let args = Args::parse();
    let filecontent = fs::read_to_string(args.input).unwrap();
    let total_iterations = filecontent.len();
    let jet_pattern = parser::parse_jet_pattern(filecontent);
    let rock_iterator = RockShapeIterator::new();
    let mut cave = Cave::new(jet_pattern, rock_iterator);
    let mut iterations_jet = 0;
    let mut last_stored_height = 0;
    let mut last_stored_jet_iterations = 0;
    for i in 0..args.max_iters {
        //if (i + 1) % 5 == 0 {
        println!(
                "Iterations to reach total jet: {}, Cave height: {}, Number jet covered items: {}, Difference height: {}, difference jet: {}",
                i + 1,
                cave.height(),
                iterations_jet,
                cave.height() - last_stored_height,
                iterations_jet - last_stored_jet_iterations
            );
        last_stored_height = cave.height();
        last_stored_jet_iterations = iterations_jet;
        //}

        iterations_jet += cave.spawn_and_move_new_rock();
    }
    println!("Total jet items: {}", total_iterations);
    println!("Number jet covered items: {}", iterations_jet);
    println!("Final cave height: {}", cave.height());
}