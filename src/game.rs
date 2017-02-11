use std::io;
use std::io::BufRead;
use pieces::piece::GamePiece;
use pieces::piece::DEFAULT_PIECE;
use pieces::piece::Outcome;
use log::*;
use rand;
use rand::distributions::{IndependentSample, Range};

pub fn run (options: &super::Options) {
    vv(format!("Picking something"), options);
    println!("Pick an option: 1) Rock, 2) Paper, 3) Scissors.");
    let stdin = io::stdin();
    let mut player_choice = DEFAULT_PIECE;
    for line in stdin.lock().lines() {
        player_choice = GamePiece::from_i32(line
            .ok()
            .and_then(|n| n.parse::<i32>().ok())
            .and_then(|n| Some(n - 1))
            .unwrap_or(DEFAULT_PIECE.to_i32()));
        break;
    }
    vv(format!("Something was picked something {}", player_choice), options);
    println!("You picked {}!", player_choice);

    let mut rng = rand::thread_rng();
    let between = Range::new(0, 3);
    let ai_choice = GamePiece::from_i32(between.ind_sample(&mut rng));
    println!("I picked {}.", ai_choice);

    let player_piece = player_choice.to_piece().expect("Try again with a valid piece, options 1 - 3");
    let ai_choice = ai_choice.to_piece().unwrap();
    let result = player_piece.fights(&ai_choice.as_ref().to_game_piece());

    match result {
        Outcome::Wins => println!("You won!"),
        Outcome::Loses => println!("You lost!"),
        Outcome::Ties => println!("It was a tie!"),
        _ => (),
    }
}