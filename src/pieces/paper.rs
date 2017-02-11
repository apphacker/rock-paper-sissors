use pieces::piece::*;

pub struct Paper {}

impl Piece for Paper {

    fn fights(&self, opponent: &GamePiece) -> Outcome {
        match opponent {
            &GamePiece::Paper => Outcome::Ties,
            &GamePiece::Rock => Outcome::Wins,
            &GamePiece::Scissors => Outcome::Loses,
            _ => Outcome::Invalid,
        }
    }

    fn to_game_piece(&self) -> GamePiece {
        return GamePiece::Paper;
    }
}