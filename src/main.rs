use crate::ai_funcs::ai_types::abminimax::ABMinimax;
use crate::ai_funcs::ai_types::random::Random;
use crate::board_structs::board::Board;
use crate::board_structs::board_types::array2d::Array2D;
use crate::utils::pieces::{BLACK, WHITE};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

pub mod ai_funcs;
pub mod board_structs;
pub mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Must provide an input and output file (in that order)");
        return;
    }
    let file = BufReader::new(File::open(&args[1]).unwrap());
    let fen_string = file.lines().next().unwrap().unwrap();

    //Change board type here
    let mut game = Array2D::read_from_fen(fen_string);
    let player1 = ABMinimax {};
    let player2 = ABMinimax {};

    let mut turn_num: usize = 0;
    let mut time_left: u128 = 900000000000;
    while time_left > 0 && !game.get_valid_moves().is_empty() {
        let start = Instant::now();
        let turn: bool = (turn_num % 2) != 0;
        let next_move = match turn {
            WHITE => player1.find_move(game, WHITE, time_left),
            BLACK => player2.find_move(game, BLACK, time_left),
        };

        let turn_color = match turn {
            WHITE => "WHITE",
            BLACK => "BLACK",
        };

        time_left = match time_left.checked_sub(start.elapsed().as_nanos()) {
            None => break,
            Some(x) => x,
        };
        turn_num += 1;
        game = game.make_move(next_move);
        println!(
            "\nTurn number: {} | Player: {} | Move: {} | Time Left: {}\n",
            turn_num, turn_color, next_move, time_left
        );
    }
}
