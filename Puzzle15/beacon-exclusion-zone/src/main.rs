use beacon_exclusion_zone::parser::parse_map;
use beacon_exclusion_zone::sensor::{MAX_COORDINATE_VALUE, MIN_COORDINATE_VALUE};
use clap::Parser;
use std::fs;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, short)]
    input: String,
    #[arg(long, short)]
    y: i32,
    #[arg(action, long, short)]
    locate_beacon: bool,
}

fn main() {
    let args = Args::parse();
    let file_content = fs::read_to_string(args.input).unwrap();
    if !args.locate_beacon {
        let sensor_map = parse_map(file_content.clone(), args.y, args.locate_beacon);
        let unavailable_tiles_at_y = sensor_map.number_no_possible_beacon_location();

        if let Some(unavailable_tiles) = unavailable_tiles_at_y {
            println!(
                "In the row where y={}, there are {} positions where a beacon cannot be present.",
                args.y, unavailable_tiles
            )
        }
    } else {
        for y in MIN_COORDINATE_VALUE..=MAX_COORDINATE_VALUE {
            println!("Checking y={}", y);
            let sensor_map = parse_map(file_content.clone(), y, args.locate_beacon);
            if let Some(unavailable_tiles) = sensor_map.number_no_possible_beacon_location() {
                if unavailable_tiles < (MAX_COORDINATE_VALUE - MIN_COORDINATE_VALUE) as usize {
                    println!("Available spot at y={}!", y);
                }
            }
        }
    }
}
