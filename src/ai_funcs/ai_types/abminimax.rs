#![allow(dead_code)]
use crate::ai_funcs::ai_utils::heuristics::{heuristic, is_terminal};
use std::collections::BinaryHeap;
use std::time::Instant;
use crate::board_structs::board::Board;
use crate::utils::game_move::GameMove;
use crate::utils::pieces::WHITE;

pub struct ABMinimax {
}

impl ABMinimax {
    pub fn find_move<T: Board>(&self, game: T, max_player: bool, time_left: u128) -> GameMove {
        id_minimax(game, max_player, time_left)
    }
}

fn max_choice<T: crate::board_structs::board::Board>(game: T, depth: usize, available_moves: &Vec<GameMove>) -> GameMove {
    let mut best_move = available_moves[0];
    let mut best_value = i32::MIN;
    let moves = BinaryHeap::from_iter(available_moves.iter());
    for action in moves {
        let minimax_value = min_value(game.clone().make_move(*action), depth, &i32::MIN, &i32::MAX, available_moves);
        if best_value < minimax_value {
            best_value = minimax_value;
            best_move = *action;
        }
    }
    best_move
}

fn min_choice<T: crate::board_structs::board::Board>(game: T, depth: usize, available_moves: &Vec<GameMove>) -> GameMove {
    let mut best_move = available_moves[0];
    let mut best_value = i32::MAX;
    let moves = BinaryHeap::from_iter(available_moves.iter());
    for action in moves {
        let minimax_value = max_value(game.clone().make_move(*action), depth, &i32::MIN, &i32::MAX, available_moves);
        if best_value > minimax_value {
            best_value = minimax_value;
            best_move = *action;
        }
    }
    best_move
}

fn max_value<T: Board>(game: T, depth: usize, alpha: &i32, beta: &i32, available_moves: &Vec<GameMove>) -> i32 {
    if let Some(winner) = is_terminal(&game, available_moves) {
        return winner;
    }
    if depth == 0 {
        return heuristic(game);
    }
    let mut value = i32::MIN;
    let mut alpha = *alpha;
    let moves = BinaryHeap::from_iter(available_moves.iter());
    for action in moves {
        let action_val = min_value(game.clone().make_move(*action), depth - 1, &alpha, beta, available_moves);
        value = i32::max(value, action_val);
        if value >= *beta {
            return value;
        }
        alpha = i32::max(alpha, value);
    }
    value
}

fn min_value<T: Board>(game: T, depth: usize, alpha: &i32, beta: &i32, available_moves: &Vec<GameMove>) -> i32 {
    if let Some(winner) = is_terminal(&game, available_moves) {
        return winner;
    }
    if depth == 0 {
        return heuristic(game);
    }
    let mut value = i32::MAX;
    let mut beta = *beta;
    let moves = BinaryHeap::from_iter(available_moves.iter());
    for action in moves {
        let action_val = max_value(game.clone().make_move(*action), depth - 1, alpha, &beta, available_moves);
        value = i32::min(value, action_val);
        if value <= *alpha {
            return value;
        }
        beta = i32::min(beta, value);
    }
    value
}

fn id_minimax<T: Board>(game: T, max_player: bool, time_left: u128) -> GameMove {
    let mut second_iter_time = Instant::now();
    let available_time = time_left / 20;

    let mut depth: usize = 1;
    let mut done = false;
    let mut best_move;
    let move_search = if max_player == WHITE { max_choice } else { min_choice };

    best_move = move_search(game.clone(), depth, &game.get_valid_moves());
    depth += 1;
    let mut last_iter_time = Instant::now();

    while !done {
        let valid_moves = game.get_valid_moves();
        best_move = move_search(game.clone(), depth, &valid_moves);
        depth += 1;

        let this_iter_time = Instant::now();
        let time_ratio = (this_iter_time - last_iter_time).as_nanos()
            / (last_iter_time - second_iter_time).as_nanos();
        let time_predict = (this_iter_time - last_iter_time).as_nanos() * time_ratio;

        second_iter_time = last_iter_time;
        last_iter_time = this_iter_time;
        if this_iter_time.elapsed().as_nanos() + time_predict > available_time {
            done = true;
        }
        println!(
            "Depth {} | Took {} ns | Best Move: {} | {} times last depth | Predicting {} ns",
            depth,
            (this_iter_time - second_iter_time).as_nanos(),
            best_move,
            time_ratio,
            time_predict,
        );
    }
    best_move
}
