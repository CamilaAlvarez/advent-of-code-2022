use beacon_exclusion_zone::parser::parse_map;
use clap::Parser;
use std::fs;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, short)]
    input: String,
    #[arg(long, short)]
    y: i32,
}

fn main() {
    let args = Args::parse();
    let file_content = fs::read_to_string(args.input).unwrap();
    let sensor_map = parse_map(file_content);
    let unavailable_tiles_at_y = sensor_map.number_no_possible_beacon_location(args.y);

    if let Some(unavailable_tiles) = unavailable_tiles_at_y {
        println!(
            "In the row where y={}, there are {} positions where a beacon cannot be present.",
            args.y, unavailable_tiles
        )
    }
}
