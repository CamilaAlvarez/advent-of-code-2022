use clap::Parser;
use proboscidea_volcanium::parser::parse_file_to_valves;
use proboscidea_volcanium::path::get_best_path;
use std::fs;
use std::rc::Rc;
use std::time::Instant;

#[derive(Parser)]
#[command(about, version, author)]
struct Args {
    #[arg(long, short)]
    input: String,
    #[arg(long, short)]
    first_valve_name: String,
    #[arg(long, short)]
    minutes: i32,
}
fn main() {
    let args = Args::parse();
    let filecontent = fs::read_to_string(args.input).unwrap();
    let valves = parse_file_to_valves(filecontent);
    let elapsed_time = Instant::now();
    if let Some(first_valve) = valves.get(&args.first_valve_name) {
        if let Some(most_pressure) = get_best_path(
            first_valve,
            &valves.values().map(|v| Rc::clone(&v)).collect::<Vec<_>>(),
            args.minutes,
        ) {
            println!("Pressure released: {}", most_pressure);
            println!("Took {}s", elapsed_time.elapsed().as_millis());
        }
    } else {
        println!("No valve named {}", args.first_valve_name);
    }
}
