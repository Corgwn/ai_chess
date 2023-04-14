use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::time::Instant;

mod board;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Must provide an input and output file (in that order)");
        return;
    }
    let file = BufReader::new(File::open(&args[1]).unwrap());
    let fen_string = file.lines().next().unwrap().unwrap();
    let game = board::GameState::from_fen(&fen_string);

    //Find all valid moves
    let now = Instant::now();
    let valid_moves = game.valid_moves();
    let elapsed = now.elapsed();
    println!("{}", elapsed.as_nanos());

    let mut f = File::create(args[2].clone()).unwrap();
    writeln!(f, "{:?}", game).unwrap();
    writeln!(f, "{} moves:", valid_moves.len()).unwrap();
    for mov in valid_moves {
        writeln!(f, "{}", mov).unwrap();
    }
}
