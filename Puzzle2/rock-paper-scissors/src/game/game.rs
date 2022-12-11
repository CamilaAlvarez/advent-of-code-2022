use super::error::RockPaperScissorsError;
use super::hand_option::HandOption;
use super::opponent::parse_opponent;
use super::player::{parse_player_move, PlayerOption};
use std::fs;

pub struct Round {
    opponent: HandOption,
    player: PlayerOption,
    round_points: u32,
}

impl Round {
    pub fn new(opponent_str: &str, player_str: &str) -> Result<Self, RockPaperScissorsError> {
        let opponent = parse_opponent(opponent_str)?;
        let player = parse_player_move(opponent, player_str)?;
        Ok(Self {
            opponent,
            player,
            round_points: resolve_round_points(opponent, player),
        })
    }
    pub fn opponent(&self) -> HandOption {
        self.opponent
    }
    pub fn player(&self) -> PlayerOption {
        self.player
    }
    pub fn round_points(&self) -> u32 {
        self.round_points
    }
}

pub fn resolve_round_points(opponent_move: HandOption, player_move: PlayerOption) -> u32 {
    match player_move {
        PlayerOption::Rock(hand, points)
        | PlayerOption::Paper(hand, points)
        | PlayerOption::Scissors(hand, points) => points + hand.points_against(opponent_move),
    }
}

pub fn load_game(filename: &str) -> Result<Vec<Round>, RockPaperScissorsError> {
    let mut rounds = vec![];
    let game_str = fs::read_to_string(filename)?;
    for game_line in game_str.lines() {
        if game_line.trim().is_empty() {
            continue;
        }
        let round_line = game_line.trim().split_whitespace().collect::<Vec<_>>();
        if round_line.len() < 2 {
            return Err(RockPaperScissorsError::InvalidGame(format!(
                "Game is missing players: {}",
                game_line
            )));
        }
        rounds.push(Round::new(round_line[0], round_line[1])?);
    }
    Ok(rounds)
}
