use std::{env, cmp};
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::time::Instant;
use rand::{seq::SliceRandom, thread_rng};
use cpython::{py_module_initializer, PyResult, Python, py_fn};

mod board;
use board::{ GameState, term, heuristic, Move };

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
    let best_move = id_minimax(game.clone(), !game.curr_move);
    let elapsed = now.elapsed();
    println!("{}s", elapsed.as_secs());

    let mut f = File::create(args[2].clone()).unwrap();
    writeln!(f, "{} moves:", valid_moves.len()).unwrap();
    for mov in valid_moves {
        write!(f, "{} ", mov).unwrap();
    }
    writeln!(f, "\nMy move: {}", best_move).unwrap();
}

py_module_initializer! (rust_ai, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "find_random_move", py_fn!(py, find_random_move_py(fen: String)))?;
    m.add(py, "moves_as_strings", py_fn!(py, moves_as_strings_py(fen: String)))?;
    Ok(())
});

fn find_random_move (fen: &str) -> Option<String> {
  let game = GameState::from_fen(fen);
  let mov = game.valid_moves.choose(&mut thread_rng())?;
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

fn moves_as_strings_py (_: Python, fen: String) -> PyResult<Vec<String>> {
  let valid_moves = GameState::from_fen(&fen).valid_moves;
  let mut mov_strings = valid_moves.iter().map(|mov| mov.to_string()).collect::<Vec<String>>();
  mov_strings.sort();
  Ok(mov_strings)
}

fn dl_minimax_value (game: GameState, depth: usize, max_player: bool) -> f32 {
    if let Some(winner) = term(&game) {
        return winner;
    }
    if depth == 0 {
        return heuristic(&game);
    }
    if max_player {
        let mut value = f32::NEG_INFINITY;
        for action in game.valid_moves.iter() {
            let action_val = dl_minimax_value(game.clone().make_move(*action), depth - 1, !max_player);
            value = f32::max(value, action_val);
        }
        value
    }
    else /* Min Player */ {
        let mut value = f32::INFINITY;
        for action in game.valid_moves.iter() {
            let action_val = dl_minimax_value(game.clone().make_move(*action), depth - 1, !max_player);
            value = f32::min(value, action_val);
        }
        value
    }
}

fn id_minimax (game: GameState, max_player: bool) -> Move {
    let mut depth: usize = 1;
    let mut best_value: f32 = 0.0;
    let mut best_move = game.valid_moves[0].clone();
    while best_value != f32::MIN || best_value != f32::MAX {
        let now = Instant::now();
        println!("Checking Depth {depth}");
        for action in game.valid_moves.iter() {
            let minimax_value = dl_minimax_value(game.clone().make_move(*action), depth, max_player);
            match max_player {
                true => {
                    if best_value < minimax_value {
                        best_value = minimax_value;
                        best_move = *action;
                    }
                },
                false => {
                    if best_value > minimax_value {
                        best_value = minimax_value;
                        best_move = *action;
                    }
                }
            }
        }
        depth += 1;
        println!("Took {:?} milliseconds, best move so far: {}", now.elapsed().as_millis(), best_move);
        if depth >= 6 {
            break;
        }
    }
    best_move
}