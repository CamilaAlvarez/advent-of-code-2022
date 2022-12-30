use super::{Particle, ParticleCoord};
use std::collections::HashMap;

pub struct AdjecencyMap {
    // we encode x, y, z with the help of sub-hashmaps
    possible_adjecent_points:
        HashMap<ParticleCoord, HashMap<ParticleCoord, HashMap<ParticleCoord, i32>>>,
}
impl AdjecencyMap {
    pub fn new() -> Self {
        Self {
            possible_adjecent_points: HashMap::new(),
        }
    }
    pub fn add_adjecent_points_to(&mut self, particle: Particle) {
        // each particle has 6 possible adjecent particles
        // Modify x, but keep y and z the same
        self.add_to_map_with(particle.x() + 1, particle.y(), particle.z());
        self.add_to_map_with(particle.x() - 1, particle.y(), particle.z());

        // Modify y, but keep x and z the same
        self.add_to_map_with(particle.x(), particle.y() + 1, particle.z());
        self.add_to_map_with(particle.x(), particle.y() - 1, particle.z());

        // Modify z, but keep y and x the same
        self.add_to_map_with(particle.x(), particle.y(), particle.z() + 1);
        self.add_to_map_with(particle.x(), particle.y(), particle.z() - 1);
    }
    fn add_to_map_with(&mut self, x: ParticleCoord, y: ParticleCoord, z: ParticleCoord) {
        if let Some(x_plus_y_map) = self.possible_adjecent_points.get_mut(&x) {
            if let Some(x_plus_z_map) = x_plus_y_map.get_mut(&y) {
                if let Some(counter) = x_plus_z_map.get_mut(&z) {
                    *counter += 1;
                } else {
                    x_plus_z_map.insert(z, 1);
                }
            } else {
                let mut z_map = HashMap::new();
                z_map.insert(z, 1);
                x_plus_y_map.insert(y, z_map);
            }
        } else {
            let mut z_map = HashMap::new();
            z_map.insert(z, 1);
            let mut y_map = HashMap::new();
            y_map.insert(y, z_map);
            self.possible_adjecent_points.insert(x, y_map);
        }
    }
    pub fn number_of_particles_adjacent_to(&self, particle: Particle) -> i32 {
        if let Some(y_map) = self.possible_adjecent_points.get(&particle.x()) {
            if let Some(z_map) = y_map.get(&particle.y()) {
                if let Some(count) = z_map.get(&particle.z()) {
                    return count.clone();
                }
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_number_adjecent() {
        let particle = Particle::new(1, 1, 1);
        let adjecent_particle = Particle::new(2, 1, 1);
        let mut adjecency_map = AdjecencyMap::new();
        adjecency_map.add_adjecent_points_to(particle);
        assert_eq!(
            1,
            adjecency_map.number_of_particles_adjacent_to(adjecent_particle)
        );
    }
}
