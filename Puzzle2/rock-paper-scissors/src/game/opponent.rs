use super::error::RockPaperScissorsError;
use super::hand_option::HandOption;

pub fn parse_opponent(opponent_move: &str) -> Result<HandOption, RockPaperScissorsError> {
    match opponent_move {
        "A" => Ok(HandOption::Rock),
        "B" => Ok(HandOption::Paper),
        "C" => Ok(HandOption::Scissors),
        _ => Err(RockPaperScissorsError::from(format!(
            "No such move: {}",
            opponent_move
        ))),
    }
}
