use pieces::piece::*;

pub struct Scissors {}

impl Piece for Scissors {

    fn fights(&self, opponent: &GamePiece) -> Outcome {
        match opponent {
            &GamePiece::Paper => Outcome::Wins,
            &GamePiece::Rock => Outcome::Loses,
            &GamePiece::Scissors => Outcome::Ties,
            _ => Outcome::Invalid,
        }
    }

    fn to_game_piece(&self) -> GamePiece {
        return GamePiece::Scissors;
    }
}