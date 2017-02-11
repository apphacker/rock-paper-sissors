use std::io;
use std::io::BufRead;
use pieces::piece::GamePiece;
use pieces::piece::DEFAULT_PIECE;
use log::*;

pub fn run (options: &super::Options) {
    vv(format!("Picking something"), options);
    println!("pick something");
    let stdin = io::stdin();
    let mut game_type = DEFAULT_PIECE;
    for line in stdin.lock().lines() {
        game_type = GamePiece::from_i32(line
            .ok()
            .and_then(|n| n.parse::<i32>().ok())
            .unwrap_or(DEFAULT_PIECE.to_i32()));
        break;
    }
    vv(format!("Something was picked something {}", game_type), options);
    println!("You picked {}!", game_type);
}