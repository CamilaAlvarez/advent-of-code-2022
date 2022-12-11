use clap::Parser;
use std::collections::VecDeque;
use std::fs;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, short)]
    input_file: String,
    #[arg(long, short)]
    num_chars: usize,
}

fn is_marker(marker: &VecDeque<char>) -> bool {
    for index in 0..marker.len() {
        let char = marker[index];
        for j in index + 1..marker.len() {
            if char == marker[j] {
                return false;
            }
        }
    }
    true
}

fn main() {
    let args = Args::parse();
    let num_chars = args.num_chars;
    let input = fs::read_to_string(args.input_file).unwrap();
    let first_line = input.lines().collect::<Vec<_>>()[0];
    if first_line.len() < 4 {
        println!("No marker, string too short");
    } else {
        let datastream_chars = first_line.chars().collect::<Vec<_>>();
        let mut marker_index = None;
        let mut possible_marker = VecDeque::with_capacity(num_chars);
        for i in 0..num_chars {
            possible_marker.push_back(datastream_chars[i]);
        }
        if is_marker(&possible_marker) {
            println!("Marker index: {}", num_chars);
            return;
        }

        for index in args.num_chars..datastream_chars.len() {
            let next_char = datastream_chars[index];
            possible_marker.pop_front();
            possible_marker.push_back(next_char);
            if is_marker(&possible_marker) {
                marker_index = Some(index + 1);
                break;
            }
        }
        if let Some(marker_index) = marker_index {
            println!("Marker index: {}", marker_index);
            return;
        }
        println!("No marker");
    }
}
