use super::error::RockPaperScissorsError;
use super::hand_option::HandOption;
use std::convert::From;

#[derive(Debug, Clone, Copy)]
pub enum PlayerOption {
    Rock(HandOption, u32),
    Paper(HandOption, u32),
    Scissors(HandOption, u32),
}
impl PlayerOption {
    pub fn new(player_move: HandOption) -> Self {
        match player_move {
            HandOption::Rock => Self::Rock(HandOption::Rock, 1),
            HandOption::Paper => Self::Paper(HandOption::Paper, 2),
            HandOption::Scissors => Self::Scissors(HandOption::Scissors, 3),
        }
    }
}

pub fn parse_player_move(
    opponent_move: HandOption,
    player_move: &str,
) -> Result<PlayerOption, RockPaperScissorsError> {
    let chosen_move = match player_move {
        "X" => parse_lose(opponent_move),
        "Y" => parse_draw(opponent_move),
        "Z" => parse_win(opponent_move),
        _ => {
            return Err(RockPaperScissorsError::from(format!(
                "No such move: {}",
                player_move
            )))
        }
    };
    Ok(PlayerOption::new(chosen_move))
}
fn parse_win(opponent_move: HandOption) -> HandOption {
    match opponent_move {
        HandOption::Rock => HandOption::Paper,
        HandOption::Paper => HandOption::Scissors,
        HandOption::Scissors => HandOption::Rock,
    }
}
fn parse_lose(opponent_move: HandOption) -> HandOption {
    match opponent_move {
        HandOption::Rock => HandOption::Scissors,
        HandOption::Paper => HandOption::Rock,
        HandOption::Scissors => HandOption::Paper,
    }
}
fn parse_draw(opponent_move: HandOption) -> HandOption {
    opponent_move
}
