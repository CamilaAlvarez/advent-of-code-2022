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
}
