use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::time::Instant;
use rand::{seq::SliceRandom, thread_rng};
use cpython::{py_module_initializer, PyResult, Python, py_fn};

mod board;
use board::GameState;

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

py_module_initializer! (rust_ai, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "find_random_move", py_fn!(py, find_random_move_py(fen: String)))?;
    m.add(py, "moves_as_strings", py_fn!(py, moves_as_strings_py(fen: String)))?;
    Ok(())
});

pub fn find_random_move (fen: &str) -> Option<String> {
  let game = GameState::from_fen(fen);
  let valid_moves = game.valid_moves();
  let mov = valid_moves.choose(&mut thread_rng())?;
  Some(mov.to_string())
}

fn find_random_move_py (_: Python, fen: String) -> PyResult<String> {
  if let Some(mov) = find_random_move(&fen) {
    Ok(mov)
  }
  else {
    Ok("".to_owned())
  }
}

pub fn moves_as_strings_py (_: Python, fen: String) -> PyResult<Vec<String>> {
  let valid_moves = GameState::from_fen(&fen).valid_moves();
  let mut mov_strings = valid_moves.iter().map(|mov| mov.to_string()).collect::<Vec<String>>();
  mov_strings.sort();
  Ok(mov_strings)
}
