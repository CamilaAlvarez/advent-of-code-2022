use boiling_boulders::particle::adjecent::AdjecencyMap;
use boiling_boulders::particle::parser;
use clap::Parser;
use std::fs;

#[derive(Debug, Parser)]
#[command(about, version, author)]
struct Args {
    #[arg(short, long)]
    input: String,
}
// We'll try the following
// 1. Go over the list once to create a list with all possible adjacent cubes for all particles (might be a series of maps to make it faster to access),
// with the number of cubes adjecent to that point
// 2. Assume all sides are visible (compute 6 * number of particles). Store this value in a variable visible_sides
// 3. Go through the list again, each time I find the particle in the adjacent list sustract the number of particles adjecent to this point (stored in the list from point 1) from visible sides
// 4. Return visible_sides
fn main() {
    let args = Args::parse();
    let particles_string = fs::read_to_string(args.input).unwrap();
    let particles = parser::parse_particles_from_string(particles_string);
    let mut adjacency_map = AdjecencyMap::new();
    for particle in particles.iter() {
        adjacency_map.add_adjecent_points_to(*particle);
    }
    // 2. Assume all sides are visible (compute 6 * number of particles). Store this value in a variable visible_sides
    let mut visible_sides = 6 * particles.len() as i32;
    // 3. Go through the list again, each time I find the particle in the adjacent list sustract the number of particles adjecent to this point
    for particle in particles.iter() {
        // For each interesection there will be two points we'll check, so we don't have to multiply by two
        visible_sides -= adjacency_map.number_of_particles_adjacent_to(*particle);
    }
    // 4. Return visible_sides
    println!("Visible sides: {}", visible_sides);
}
