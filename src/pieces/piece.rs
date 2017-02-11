use std::fmt;
use pieces::paper::Paper;
use pieces::rock::Rock;
use pieces::scissors::Scissors;

#[derive(Debug)]
pub enum GamePiece {
    Rock,
    Paper,
    Scissors,
    Invalid,
}

pub const DEFAULT_PIECE: GamePiece = GamePiece::Invalid;

impl GamePiece {
    pub fn from_i32(n: i32) -> GamePiece {
        match n {
            0 => GamePiece::Rock,
            1 => GamePiece::Paper,
            2 => GamePiece::Scissors,
            _ => GamePiece::Invalid,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            &GamePiece::Rock => 0,
            &GamePiece::Paper => 1,
            &GamePiece::Scissors => 2,
            &GamePiece::Invalid => 100,
        }
    }

    pub fn to_string(&self) -> String {
        String::from(match self {
            &GamePiece::Rock => "rock",
            &GamePiece::Paper => "paper",
            &GamePiece::Scissors => "scissors",
            &GamePiece::Invalid => "an invalid piece",
        })
    }

    pub fn to_piece(&self) -> Option<Box<Piece>> {
        match self {
            &GamePiece::Rock => Some(Box::new(Rock{})),
            &GamePiece::Paper => Some(Box::new(Paper{})),
            &GamePiece::Scissors => Some(Box::new(Scissors{})),
            &GamePiece::Invalid => None,
        }
    }
}

impl fmt::Display for GamePiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub enum Outcome {
    Wins,
    Loses,
    Ties,
    Invalid,
}

pub trait Piece {
    fn fights(&self, piece: &GamePiece) -> Outcome;
    fn to_game_piece(&self) -> GamePiece;
}