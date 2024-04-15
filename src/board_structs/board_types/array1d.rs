use crate::board_structs::board::Board;
use crate::utils::game_move::GameMove;
use crate::utils::pieces::{Pieces, Pieces::*, BLACK, WHITE};

#[derive(Copy, Clone)]
pub struct Mailbox {
    board: [Pieces; 120],
    curr_player: bool,
    //In order: white kingside, white queenside, black kingside, black queenside
    castling_rights: [bool; 4],
    //None if no en passant is possible, Some if possible by taking the position given with a pawn
    en_passant: Option<usize>,
    half_moves: usize,
    full_moves: usize,
    check: Option<bool>,
    white_king: usize,
    black_king: usize,
}

impl Board for Mailbox {
    fn setup_board(fen: Option<&str>) -> Self {
        todo!()
    }

    fn get_valid_moves(&self) -> Vec<GameMove> {
        let mut result = vec![];
        for pos in 0..=120 {
            result.extend(generate_piece_moves(self, pos));
        }
        result
    }

    fn make_move(&self, mov: GameMove) -> Self {
        todo!()
    }

    fn get_check(&self) -> Option<bool> {
        todo!()
    }

    fn get_curr_player(&self) -> bool {
        self.curr_player
    }

    fn get_board_as_2d(&self) -> [[Pieces; 8]; 8] {
        todo!()
    }
}

fn generate_piece_moves(game: &Mailbox, pos: usize) -> Vec<GameMove> {
    let result = vec![];
    let piece_type = game.board[pos];

    match piece_type {
        Rook(x) | Queen(x) | Bishop(x) => if x == game.curr_player {},
        _ => {}
    }

    result
}
