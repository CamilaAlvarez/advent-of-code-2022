use std::convert::From;
use std::fmt;

#[derive(Debug)]
pub enum RockPaperScissorsError {
    InvalidMove(String),
    InvalidGame(String),
}

impl From<String> for RockPaperScissorsError {
    fn from(error: String) -> Self {
        RockPaperScissorsError::InvalidMove(error)
    }
}
impl fmt::Display for RockPaperScissorsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMove(e) => write!(f, "Invalid move: {}", e),
            Self::InvalidGame(e) => write!(f, "Invalid game: {}", e),
        }
    }
}
impl From<std::io::Error> for RockPaperScissorsError {
    fn from(e: std::io::Error) -> Self {
        RockPaperScissorsError::InvalidGame(e.to_string())
    }
}
