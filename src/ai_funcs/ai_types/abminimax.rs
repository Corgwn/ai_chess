#![allow(dead_code)]
use crate::ai_funcs::ai_utils::heuristics::heuristic;
use crate::ai_funcs::ai_utils::heuristics::term;
use rand::{seq::SliceRandom, thread_rng};
use std::collections::BinaryHeap;
use std::time::Instant;

use crate::board_structs::board_types::array2d::GameState;
use crate::utils::game_move::GameMove;

fn find_random_move(fen: &str) -> Option<String> {
    let game = GameState::from_fen(fen);
    let mov = game.available_moves.choose(&mut thread_rng())?;
    Some(mov.to_string())
}

fn max_choice(game: GameState, depth: usize) -> GameMove {
    let mut best_move = game.available_moves[0];
    let mut best_value = i32::MIN;
    let moves = BinaryHeap::from_iter(game.available_moves.iter());
    for action in moves {
        let minimax_value = min_value(game.clone().make_move(*action), depth, &i32::MIN, &i32::MAX);
        if best_value < minimax_value {
            best_value = minimax_value;
            best_move = *action;
        }
    }
    best_move
}

fn min_choice(game: GameState, depth: usize) -> GameMove {
    let mut best_move = game.available_moves[0];
    let mut best_value = i32::MAX;
    let moves = BinaryHeap::from_iter(game.available_moves.iter());
    for action in moves {
        let minimax_value = max_value(game.clone().make_move(*action), depth, &i32::MIN, &i32::MAX);
        if best_value > minimax_value {
            best_value = minimax_value;
            best_move = *action;
        }
    }
    best_move
}

fn max_value(game: GameState, depth: usize, alpha: &i32, beta: &i32) -> i32 {
    if let Some(winner) = term(&game) {
        return winner;
    }
    if depth == 0 {
        return heuristic(&game);
    }
    let mut value = i32::MIN;
    let mut alpha = *alpha;
    let moves = BinaryHeap::from_iter(game.available_moves.iter());
    for action in moves {
        let action_val = min_value(game.clone().make_move(*action), depth - 1, &alpha, beta);
        value = i32::max(value, action_val);
        if value >= *beta {
            return value;
        }
        alpha = i32::max(alpha, value);
    }
    value
}

fn min_value(game: GameState, depth: usize, alpha: &i32, beta: &i32) -> i32 {
    if let Some(winner) = term(&game) {
        return winner;
    }
    if depth == 0 {
        return heuristic(&game);
    }
    let mut value = i32::MAX;
    let mut beta = *beta;
    let moves = BinaryHeap::from_iter(game.available_moves.iter());
    for action in moves {
        let action_val = max_value(game.clone().make_move(*action), depth - 1, alpha, &beta);
        value = i32::min(value, action_val);
        if value <= *alpha {
            return value;
        }
        beta = i32::min(beta, value);
    }
    value
}

pub fn id_minimax(game: GameState, max_player: bool, time_left: u64) -> GameMove {
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
        let time_ratio = (this_iter_time - last_iter_time).as_nanos()
            / (last_iter_time - second_iter_time).as_nanos();
        let time_predict = (this_iter_time - last_iter_time).as_nanos() * time_ratio;

        second_iter_time = last_iter_time;
        last_iter_time = this_iter_time;
        if this_iter_time.elapsed().as_nanos() + time_predict > available_time as u128 {
            done = true;
        }
        println!(
            "Took {:?} milliseconds, best move so far: {}",
            (this_iter_time - second_iter_time).as_millis(),
            best_move
        );
        println!("Took {time_ratio} times longer than last depth, predicting {time_predict}ns for next depth, done: {done}");
    }
    best_move
}
