use clap::Parser;
use hill_climbing_algorithm::dijkstra;
use hill_climbing_algorithm::height_map::HeightMap;
use std::fs;

#[derive(Debug, Parser)]
#[command(version, author, about)]
struct Args {
    #[arg(long, short)]
    input: String,
    #[arg(long, action)]
    is_hiking: bool,
}
fn main() {
    let args = Args::parse();
    let input_data = fs::read_to_string(args.input).unwrap();
    let copied_lines = input_data.lines().clone().collect::<Vec<_>>();
    let mut start = vec![];
    let mut end = None;
    for i in 0..copied_lines.len() {
        let chars = copied_lines[i].chars().collect::<Vec<_>>();
        for j in 0..chars.len() {
            if chars[j] == 'S' {
                start.push((i, j));
            } else if chars[j] == 'E' {
                end = Some((i, j))
            } else if args.is_hiking && chars[j] == 'a' {
                start.push((i, j));
            }
        }
    }
    assert!(!start.is_empty(), "No starting point");
    assert!(end.is_some(), "No ending point");
    let height_map = HeightMap::new_from_string(input_data);
    let mut possible_path_lengths = vec![];
    if let Some(dest) = end {
        for source in start.iter() {
            let path_length = dijkstra::get_path_length(&height_map, *source, dest);
            if let Some(length) = path_length {
                possible_path_lengths.push(length);
            } else {
                println!(
                    "No path from ({}, {}) to ({}, {})!",
                    source.0, source.1, dest.0, dest.1
                );
            }
        }
    }

    if possible_path_lengths.len() > 0 {
        possible_path_lengths.sort();
        println!("Sorthest path length: {}", possible_path_lengths[0]);
    }
}
