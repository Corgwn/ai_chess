use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::time::Instant;
use crate::ai_funcs::ai_types::abminimax::id_minimax;
use crate::board_structs::board_types::array2d::GameState;

pub mod board_structs;
pub mod ai_funcs;
pub mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Must provide an input and output file (in that order)");
        return;
    }
    let file = BufReader::new(File::open(&args[1]).unwrap());
    let fen_string = file.lines().next().unwrap().unwrap();
    let game = GameState::from_fen(&fen_string);

    //Find all valid moves
    let now = Instant::now();
    let valid_moves = game.valid_moves();
    println!("Finding Best Move");
    let best_move = id_minimax(game.clone(), !game.curr_move, 900000000000);
    let elapsed = now.elapsed();
    println!("{}s", elapsed.as_secs());

    let mut f = File::create(args[2].clone()).unwrap();
    writeln!(f, "{} moves:", valid_moves.len()).unwrap();
    for mov in valid_moves {
        write!(f, "{} ", mov).unwrap();
    }
    writeln!(f, "\nMy move: {}", best_move).unwrap();
}
