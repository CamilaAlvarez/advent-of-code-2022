use clap::Parser;
use regolith_reservoir::cave::Cave;
use regolith_reservoir::path::{Line, Point};
use std::fs;

#[derive(Debug, Parser)]
#[command(version, about, author)]
struct Args {
    #[arg(short, long)]
    input: String,
}
fn main() {
    const SOURCE_POINT: Point = (500, 0);
    let args = Args::parse();
    let paths_content = fs::read_to_string(args.input).unwrap();
    let mut lines = vec![];
    let mut points = vec![];
    for line in paths_content.lines() {
        let parsed_lines = Line::parse_lines_from_string(line.to_string());
        for line in parsed_lines.iter() {
            points.append(&mut line.points_in_line());
        }
        lines.push(parsed_lines);
    }
    // we need to get the top-left and bottom-right points
    // we now that y0 = 0, since source is at 500, 0
    let x_min = points.iter().fold(
        usize::MAX,
        |acc, point| if acc > point.0 { point.0 } else { acc },
    );
    let x_max = points.iter().fold(
        usize::MIN,
        |acc, point| if acc < point.0 { point.0 } else { acc },
    );
    let y_max = points.iter().fold(
        usize::MIN,
        |acc, point| if acc < point.1 { point.1 } else { acc },
    );
    let top_left = (x_min, SOURCE_POINT.1);
    let bottom_right = (x_max, y_max);
    let mut cave = Cave::new(top_left, SOURCE_POINT, bottom_right, &lines);
    cave.flood_sand();

    println!(
        "Sand is flooding the cave! {} units ended up resting",
        cave.sand_units_at_rest()
    );
}
