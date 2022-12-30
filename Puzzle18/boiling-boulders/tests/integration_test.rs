use boiling_boulders::particle::{adjecent::AdjecencyMap, parser};

#[test]
fn test_get_correct_visible_sides() {
    let particles_string = String::from(
        "2,2,2\n1,2,2\n3,2,2\n2,1,2\n2,3,2\n2,2,1\n2,2,3\n2,2,4\n2,2,6\n1,2,5\n3,2,5\n2,1,5\n2,3,5",
    );
    let particles = parser::parse_particles_from_string(particles_string);
    // 1. Go over the list once to create a list with all possible adjacent cubes for all particles
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
    assert_eq!(64, visible_sides);
}
#[test]
fn test_get_correct_visible_sides_removing_air_pockets() {
    let particles_string = String::from(
        "2,2,2\n1,2,2\n3,2,2\n2,1,2\n2,3,2\n2,2,1\n2,2,3\n2,2,4\n2,2,6\n1,2,5\n3,2,5\n2,1,5\n2,3,5",
    );
    let particles = parser::parse_particles_from_string(particles_string);
    // 1. Go over the list once to create a list with all possible adjacent cubes for all particles
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
    // 4. Remove air pockets
    visible_sides -=
        adjacency_map.number_of_particles_adjacent_to_inner_air_pockets(1, 3, 1, 3, 1, 6);
    // 5. Return visible_sides
    assert_eq!(58, visible_sides);
}
