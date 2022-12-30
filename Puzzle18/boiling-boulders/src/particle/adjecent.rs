use super::{Particle, ParticleCoord};
use std::{cell::RefCell, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
enum PositionType {
    Particle,
    Air,
}
pub struct AdjecencyMap {
    // we encode x, y, z with the help of sub-hashmaps
    possible_adjecent_points:
        HashMap<ParticleCoord, HashMap<ParticleCoord, HashMap<ParticleCoord, i32>>>,
    position_with_particles: RefCell<
        HashMap<ParticleCoord, HashMap<ParticleCoord, HashMap<ParticleCoord, PositionType>>>,
    >,
}
impl AdjecencyMap {
    pub fn new() -> Self {
        Self {
            possible_adjecent_points: HashMap::new(),
            position_with_particles: RefCell::new(HashMap::new()),
        }
    }
    pub fn add_adjecent_points_to(&mut self, particle: Particle) {
        self.add_position_type_with(
            particle.x(),
            particle.y(),
            particle.z(),
            PositionType::Particle,
        );
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
        self.add_position_type_with(x, y, z, PositionType::Air);
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
    fn add_position_type_with(
        &self,
        x: ParticleCoord,
        y: ParticleCoord,
        z: ParticleCoord,
        position_type: PositionType,
    ) {
        let mut position_with_particles = self.position_with_particles.borrow_mut();
        if let Some(x_plus_y_map) = position_with_particles.get_mut(&x) {
            if let Some(x_plus_z_map) = x_plus_y_map.get_mut(&y) {
                if let Some(current_position_type) = x_plus_z_map.get_mut(&z) {
                    // if the position was labeled as air we can change it
                    if current_position_type == &PositionType::Air {
                        *current_position_type = position_type;
                    }
                } else {
                    x_plus_z_map.insert(z, position_type);
                }
            } else {
                let mut z_map = HashMap::new();
                z_map.insert(z, position_type);
                x_plus_y_map.insert(y, z_map);
            }
        } else {
            let mut z_map = HashMap::new();
            z_map.insert(z, position_type);
            let mut y_map = HashMap::new();
            y_map.insert(y, z_map);
            position_with_particles.insert(x, y_map);
        }
    }

    pub fn number_of_particles_adjacent_to(&self, particle: Particle) -> i32 {
        self.add_position_type_with(
            particle.x(),
            particle.y(),
            particle.z(),
            PositionType::Particle,
        );
        if let Some(y_map) = self.possible_adjecent_points.get(&particle.x()) {
            if let Some(z_map) = y_map.get(&particle.y()) {
                if let Some(count) = z_map.get(&particle.z()) {
                    return count.clone();
                }
            }
        }
        0
    }
    pub fn number_of_particles_adjacent_to_inner_air_pockets(
        &self,
        min_x: ParticleCoord,
        max_x: ParticleCoord,
        min_y: ParticleCoord,
        max_y: ParticleCoord,
        min_z: ParticleCoord,
        max_z: ParticleCoord,
    ) -> i32 {
        // we need to check that the air pockets are actually pockets
        let position_with_particles = self.position_with_particles.borrow();
        let mut air_adjacency_count = 0;
        for i in min_x + 1..max_x {
            if let Some(y_map) = position_with_particles.get(&i) {
                for j in min_y + 1..max_y {
                    if let Some(z_map) = y_map.get(&j) {
                        for k in min_z + 1..max_z {
                            if let Some(position_type) = z_map.get(&k) {
                                if position_type == &PositionType::Air {
                                    if let Some(y_map) = self.possible_adjecent_points.get(&i) {
                                        if let Some(z_map) = y_map.get(&j) {
                                            if let Some(count) = z_map.get(&k) {
                                                air_adjacency_count += count.clone();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        air_adjacency_count
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
    #[test]
    fn test_check_number_adjecent_to_inner_air_pockets() {
        let particles = vec![
            Particle::new(2, 2, 4),
            Particle::new(2, 2, 6),
            Particle::new(1, 2, 5),
            Particle::new(3, 2, 5),
            Particle::new(2, 1, 5),
            Particle::new(2, 3, 5),
        ];
        let mut adjecency_map = AdjecencyMap::new();
        for particle in particles {
            adjecency_map.add_adjecent_points_to(particle);
        }

        assert_eq!(
            6,
            adjecency_map.number_of_particles_adjacent_to_inner_air_pockets(1, 3, 1, 3, 4, 6)
        );
    }
}
