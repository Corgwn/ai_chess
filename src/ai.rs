#![allow(dead_code)]
use std::env;
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

py_module_initializer! (rust_ai, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "find_random_move", py_fn!(py, find_random_move_py(fen: String)))?;
    m.add(py, "moves_as_strings", py_fn!(py, moves_as_strings_py(fen: String)))?;
    m.add(py, "id_minimax", py_fn!(py, id_minimax_py(fen: String, time: u64)))?;
    Ok(())
});

fn find_random_move (fen: &str) -> Option<String> {
  let game = GameState::from_fen(fen);
  let mov = game.available_moves.choose(&mut thread_rng())?;
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
  let valid_moves = GameState::from_fen(&fen).available_moves;
  let mut mov_strings = valid_moves.iter().map(|mov| mov.to_string()).collect::<Vec<String>>();
  mov_strings.sort();
  Ok(mov_strings)
}

fn max_choice (game: GameState, depth: usize) -> Move {
    let mut best_move = game.available_moves[0];
    let mut best_value = i32::MIN;

    for action in game.available_moves.iter() {
        let minimax_value = min_value(game.clone().make_move(*action), depth, &i32::MIN, &i32::MAX);
        if best_value < minimax_value {
            best_value = minimax_value;
            best_move = *action;
        }
    }
    best_move
}

fn min_choice (game: GameState, depth: usize) -> Move {
    let mut best_move = game.available_moves[0];
    let mut best_value = i32::MAX;

    for action in game.available_moves.iter() {
        let minimax_value = max_value(game.clone().make_move(*action), depth, &i32::MIN, &i32::MAX);
        if best_value > minimax_value {
            best_value = minimax_value;
            best_move = *action;
        }
    }
    best_move
}

fn max_value (game: GameState, depth: usize, alpha: &i32, beta: &i32) -> i32 {
    if let Some(winner) = term(&game) {
        return winner;
    }
    if depth == 0 {
        return heuristic(&game);
    }
    let mut value = i32::MIN;
    let mut alpha = *alpha;
    for action in game.available_moves.iter() {
        let action_val = min_value(game.clone().make_move(*action), depth - 1, &alpha, beta);
        value = i32::max(value, action_val);
        if value >= *beta {
            return value;
        }
        alpha = i32::max(alpha, value);
    }
    value
}

fn min_value (game: GameState, depth: usize, alpha: &i32, beta: &i32) -> i32 {
    if let Some(winner) = term(&game) {
        return winner;
    }
    if depth == 0 {
        return heuristic(&game);
    }
    let mut value = i32::MAX;
    let mut beta = *beta;
    for action in game.available_moves.iter() {
        let action_val = max_value(game.clone().make_move(*action), depth - 1, alpha, &beta);
        value = i32::min(value, action_val);
        if value <= *alpha {
            return value;
        }
        beta = i32::min(beta, value);
    }
    value
}

fn id_minimax (game: GameState, max_player: bool, time_left: u64) -> Move {
    let mut second_iter_time = Instant::now();
    let available_time = time_left / 20;

    let mut depth: usize = 1;
    let mut done = false;
    let mut best_move;
    let move_search = if max_player { max_choice } else { min_choice };
    
    best_move = move_search(game.clone(), depth);
    depth += 1;
    let mut last_iter_time = Instant::now();

    while !done {
        println!("Checking Depth {depth}");
        best_move = move_search(game.clone(), depth);
        depth += 1;
        
        let this_iter_time = Instant::now();
        let time_ratio = (this_iter_time - last_iter_time).as_nanos() / (last_iter_time - second_iter_time).as_nanos();
        let time_predict = (this_iter_time - last_iter_time).as_nanos() * time_ratio;

        second_iter_time = last_iter_time;
        last_iter_time = this_iter_time;
        if this_iter_time.elapsed().as_nanos() + time_predict > available_time as u128 {
            done = true;
        }
        println!("Took {:?} milliseconds, best move so far: {}", (this_iter_time - second_iter_time).as_millis(), best_move);
        println!("Took {time_ratio} times longer than last depth, predicting {time_predict}ns for next depth, done: {done}");
    }
    best_move
}

fn id_minimax_py (_: Python, fen: String, time: u64) -> PyResult<String> {
    let game = GameState::from_fen(&fen);
    Ok(id_minimax(game.clone(), !&game.curr_move, time).to_string())
}