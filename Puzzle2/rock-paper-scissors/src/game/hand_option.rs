#[derive(Debug, Clone, Copy)]
pub enum HandOption {
    Rock,
    Paper,
    Scissors,
}

impl HandOption {
    pub fn points_against(&self, opponent: HandOption) -> u32 {
        match self {
            Self::Rock => self.resolve_rock(opponent),
            Self::Paper => self.resolve_paper(opponent),
            Self::Scissors => self.resolve_scissors(opponent),
        }
    }

    fn resolve_rock(&self, opponent: HandOption) -> u32 {
        match opponent {
            Self::Rock => 3,
            Self::Paper => 0,
            Self::Scissors => 6,
        }
    }

    fn resolve_paper(&self, opponent: HandOption) -> u32 {
        match opponent {
            Self::Rock => 6,
            Self::Paper => 3,
            Self::Scissors => 0,
        }
    }

    fn resolve_scissors(&self, opponent: HandOption) -> u32 {
        match opponent {
            Self::Rock => 0,
            Self::Paper => 6,
            Self::Scissors => 3,
        }
    }
}
