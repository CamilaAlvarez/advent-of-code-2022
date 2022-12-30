use super::Particle;
pub fn parse_particles_from_string(particles_string: String) -> Vec<Particle> {
    let mut particles = vec![];
    for line in particles_string.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let split_line = line.trim().split(",").collect::<Vec<_>>();
        assert!(split_line.len() == 3, "Invalid line");
        let x = split_line[0].trim().parse::<i32>().unwrap();
        let y = split_line[1].trim().parse::<i32>().unwrap();
        let z = split_line[2].trim().parse::<i32>().unwrap();
        particles.push(Particle::new(x, y, z));
    }
    particles
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_correct_particles() {
        let expected_particles = vec![Particle::new(1, 1, 1), Particle::new(2, 1, 1)];
        let particles_string = String::from("1,1,1\n2,1,1\n");
        let particles = parse_particles_from_string(particles_string);
        assert_eq!(expected_particles, particles);
    }
}
