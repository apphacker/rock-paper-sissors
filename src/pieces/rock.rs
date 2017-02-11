use pieces::piece::*;

pub struct Rock {}

impl Piece for Rock {

    fn fights(&self, opponent: &GamePiece) -> Outcome {
        match opponent {
            &GamePiece::Paper => Outcome::Loses,
            &GamePiece::Rock => Outcome::Ties,
            &GamePiece::Scissors => Outcome::Wins,
            _ => Outcome::Invalid,
        }
    }

    fn to_game_piece(&self) -> GamePiece {
        return GamePiece::Rock;
    }
}