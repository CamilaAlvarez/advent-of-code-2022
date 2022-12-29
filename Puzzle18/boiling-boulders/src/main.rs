// We'll try the following
// 1. Go over the list once to create a list with all possible adjacent cubes for all particles (might be a series of maps to make it faster to access),
// with the number of cubes adjecent to that point
// 2. Assume all sides are visible (compute 6 * number of particles). Store this value in a variable visible_sides
// 3. Go through the list again, each time I find the particle in the adjacent list sustract the number of particles adjecent to this point (stored in the list from point 1) + 1 from visible sides
// 4. Return visible_sides
fn main() {
    println!("Hello, world!");
}
