#![allow(dead_code)]
use std::sync::mpsc::Receiver;
use std::time::Instant;

use crate::board_structs::board_types::mailbox::Mailbox;
use crate::utils::checks::Checks;
use crate::utils::gamemove1d::GameMove1d;
use crate::utils::pieces::{PieceColors, PieceTypes, Pieces};
use crate::utils::position::Position;

pub struct MailboxNegamax;

impl MailboxNegamax {
    pub(crate) fn uci_infinite_find_move(
        game: Mailbox,
        rx: Receiver<&str>,
        available_moves: Option<Vec<GameMove1d>>,
    ) -> GameMove1d {
        let start_time = Instant::now();
        let mut best_move: GameMove1d;
        let mut depth: usize = 1;

        best_move = root_nega_max(&game, depth, available_moves.clone());
        depth += 1;

        loop {
            match rx.try_recv() {
                Ok("stop") => break,
                Ok(_) => {}
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
                Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
            }

            best_move = root_nega_max(&game, depth, available_moves.clone());
            depth += 1;

            println!(
                "info depth {} pv {} time {}",
                depth,
                best_move,
                start_time.elapsed().as_millis(),
            )
        }

        best_move
    }
    pub(crate) fn uci_find_move(
        game: Mailbox,
        search_time: u128,
        available_moves: Option<Vec<GameMove1d>>,
        max_plies: Option<usize>,
        _max_nodes: Option<usize>,
        rx: Receiver<&str>,
    ) -> GameMove1d {
        let start_time = Instant::now();
        let mut elapsed_time;
        let mut best_move: GameMove1d;
        let mut depth: usize = 1;

        best_move = root_nega_max(&game, depth, available_moves.clone());
        depth += 1;
        elapsed_time = start_time.elapsed();

        while elapsed_time.as_millis() < search_time || depth < max_plies.unwrap_or(usize::MAX) {
            match rx.try_recv() {
                Ok("stop") => break,
                Ok(_) => {}
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
                Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
            }

            best_move = root_nega_max(&game, depth, available_moves.clone());
            depth += 1;
            elapsed_time = start_time.elapsed();

            println!(
                "info depth {} pv {} time {}",
                depth,
                best_move,
                start_time.elapsed().as_millis(),
            )
        }

        best_move
    }
    pub(crate) fn uci_search_mate(
        game: Mailbox,
        search_time: u128,
        available_moves: Option<Vec<GameMove1d>>,
        mate_moves: usize,
    ) -> GameMove1d {
        // TODO: set up search thread, and listen for both stop command and search information
        // If stop command is issued, immediately stop search and return most recent best move
        let best_move: GameMove1d = Default::default();

        // TODO: if search ends normally, return best move
        best_move
    }
}

fn root_nega_max(
    game: &Mailbox,
    depth: usize,
    available_moves: Option<Vec<GameMove1d>>,
) -> GameMove1d {
    let valid_moves = available_moves.unwrap_or_else(|| game.get_valid_moves());
    let mut max_score = i32::MIN;
    let mut best_move = valid_moves[0];
    for mv in valid_moves {
        let score = nega_max(game.make_move(&mv), depth - 1);
        if max_score < score {
            max_score = score;
            best_move = mv;
        }
    }
    best_move
}

fn nega_max(game: Mailbox, depth: usize) -> i32 {
    let valid_moves = game.get_valid_moves();
    if depth == 0 {
        return evaluate(game, valid_moves);
    }
    let mut max = i32::MIN;
    for game_move in valid_moves {
        let new_game = game.make_move(&game_move);
        let score = -nega_max(new_game, depth - 1);
        if score > max {
            max = score;
        }
    }
    max
}

fn evaluate(game: Mailbox, valid_moves: Vec<GameMove1d>) -> i32 {
    // Check if game is terminal
    if valid_moves.is_empty() {
        let check = game.get_check();
        match check {
            Some(Checks::White) => {
                if game.get_curr_player() == PieceColors::Black {
                    return i32::MAX;
                } else if game.get_curr_player() == PieceColors::White {
                    return i32::MIN;
                }
            }
            Some(Checks::Black) => {
                if game.get_curr_player() == PieceColors::Black {
                    return i32::MIN;
                } else if game.get_curr_player() == PieceColors::White {
                    return i32::MAX;
                }
            }
            None => return 0,
        }
    }
    // Game is not terminal, get heuristic of the game
    let mut curr_player_value: i32 = 0;
    game.board.iter().enumerate().for_each(|(index, piece)| {
        if piece.color == game.get_curr_player() {
            curr_player_value += get_piece_value(piece, Position { value: index })
        } else {
            curr_player_value -= get_piece_value(piece, Position { value: index })
        }
    });

    curr_player_value
}

fn get_piece_value(piece: &Pieces, _pos: Position) -> i32 {
    const PAWN_VAL: i32 = 100;
    const BISHOP_VAL: i32 = 350;
    const KNIGHT_VAL: i32 = 300;
    const ROOK_VAL: i32 = 500;
    const QUEEN_VAL: i32 = 900;
    const KING_VAL: i32 = 400;
    match piece {
        Pieces {
            piece_type: PieceTypes::Empty,
            ..
        } => 0,
        Pieces {
            piece_type: PieceTypes::Knight,
            ..
        } => KNIGHT_VAL,
        Pieces {
            piece_type: PieceTypes::Bishop,
            ..
        } => BISHOP_VAL,
        Pieces {
            piece_type: PieceTypes::Pawn,
            ..
        } => PAWN_VAL,
        Pieces {
            piece_type: PieceTypes::Rook,
            ..
        } => ROOK_VAL,
        Pieces {
            piece_type: PieceTypes::Queen,
            ..
        } => QUEEN_VAL,
        Pieces {
            piece_type: PieceTypes::King,
            ..
        } => KING_VAL,
        _ => 0,
    }
}
