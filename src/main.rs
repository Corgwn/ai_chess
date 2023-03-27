use std::env;
use std::fs::File;
use std::io::Write;

mod board;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Must provide an input and output file (in that order)");
        return;
    }
    let mut game = board::GameState::from_fen(&args[1]);

    //Find all valid moves
    let valid_moves = game.valid_moves();

    let mut f = File::create(args[2].clone()).unwrap();
    writeln!(f, "{:?}", valid_moves).unwrap();
}
