pub mod adjecent;
pub mod parser;
pub type ParticleCoord = i32;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Particle {
    x: ParticleCoord,
    y: ParticleCoord,
    z: ParticleCoord,
}
impl Particle {
    pub fn new(x: ParticleCoord, y: ParticleCoord, z: ParticleCoord) -> Self {
        Self { x, y, z }
    }
    pub fn x(&self) -> ParticleCoord {
        self.x
    }
    pub fn y(&self) -> ParticleCoord {
        self.y
    }
    pub fn z(&self) -> ParticleCoord {
        self.z
    }
    pub fn get_min_x(particles: &Vec<Particle>) -> ParticleCoord {
        particles
            .iter()
            .min_by(|a, b| a.x.cmp(&b.x))
            .unwrap_or(&Self::new(i32::MIN, i32::MIN, i32::MIN))
            .x
    }
    pub fn get_min_y(particles: &Vec<Particle>) -> ParticleCoord {
        particles
            .iter()
            .min_by(|a, b| a.y.cmp(&b.y))
            .unwrap_or(&Self::new(i32::MIN, i32::MIN, i32::MIN))
            .y
    }
    pub fn get_min_z(particles: &Vec<Particle>) -> ParticleCoord {
        particles
            .iter()
            .min_by(|a, b| a.z.cmp(&b.z))
            .unwrap_or(&Self::new(i32::MIN, i32::MIN, i32::MIN))
            .z
    }
    pub fn get_max_x(particles: &Vec<Particle>) -> ParticleCoord {
        particles
            .iter()
            .max_by(|a, b| a.x.cmp(&b.x))
            .unwrap_or(&Self::new(i32::MIN, i32::MIN, i32::MIN))
            .x
    }
    pub fn get_max_y(particles: &Vec<Particle>) -> ParticleCoord {
        particles
            .iter()
            .max_by(|a, b| a.y.cmp(&b.y))
            .unwrap_or(&Self::new(i32::MIN, i32::MIN, i32::MIN))
            .y
    }
    pub fn get_max_z(particles: &Vec<Particle>) -> ParticleCoord {
        particles
            .iter()
            .max_by(|a, b| a.z.cmp(&b.z))
            .unwrap_or(&Self::new(i32::MIN, i32::MIN, i32::MIN))
            .z
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_x() {
        let particles = vec![
            Particle::new(2, 2, 4),
            Particle::new(2, 2, 6),
            Particle::new(1, 2, 5),
            Particle::new(3, 2, 5),
            Particle::new(2, 1, 5),
            Particle::new(2, 3, 5),
        ];
        assert_eq!(1, Particle::get_min_x(&particles));
    }
    #[test]
    fn test_min_y() {
        let particles = vec![
            Particle::new(2, 2, 4),
            Particle::new(2, 2, 6),
            Particle::new(1, 2, 5),
            Particle::new(3, 2, 5),
            Particle::new(2, 1, 5),
            Particle::new(2, 3, 5),
        ];
        assert_eq!(1, Particle::get_min_y(&particles));
    }
    #[test]
    fn test_min_z() {
        let particles = vec![
            Particle::new(2, 2, 4),
            Particle::new(2, 2, 6),
            Particle::new(1, 2, 5),
            Particle::new(3, 2, 5),
            Particle::new(2, 1, 5),
            Particle::new(2, 3, 5),
        ];
        assert_eq!(4, Particle::get_min_z(&particles));
    }
    #[test]
    fn test_max_x() {
        let particles = vec![
            Particle::new(2, 2, 4),
            Particle::new(2, 2, 6),
            Particle::new(1, 2, 5),
            Particle::new(3, 2, 5),
            Particle::new(2, 1, 5),
            Particle::new(2, 3, 5),
        ];
        assert_eq!(3, Particle::get_max_x(&particles));
    }
    #[test]
    fn test_max_y() {
        let particles = vec![
            Particle::new(2, 2, 4),
            Particle::new(2, 2, 6),
            Particle::new(1, 2, 5),
            Particle::new(3, 2, 5),
            Particle::new(2, 1, 5),
            Particle::new(2, 3, 5),
        ];
        assert_eq!(3, Particle::get_max_y(&particles));
    }
    #[test]
    fn test_max_z() {
        let particles = vec![
            Particle::new(2, 2, 4),
            Particle::new(2, 2, 6),
            Particle::new(1, 2, 5),
            Particle::new(3, 2, 5),
            Particle::new(2, 1, 5),
            Particle::new(2, 3, 5),
        ];
        assert_eq!(6, Particle::get_max_z(&particles));
    }
}
