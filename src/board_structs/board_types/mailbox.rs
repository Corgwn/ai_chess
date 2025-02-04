use std::sync::Arc;

use regex::bytes::CaptureLocations;

use crate::utils::castling::CastleRights;
use crate::utils::gamemove1d::GameMove1d;
use crate::utils::pieces::{PieceColors, PieceTypes, Pieces};

const UP_LEFT: i8 = 9;
const UP: i8 = 10;
const UP_RIGHT: i8 = 11;
const LEFT: i8 = -1;
const RIGHT: i8 = 1;
const DOWN_LEFT: i8 = -11;
const DOWN: i8 = -10;
const DOWN_RIGHT: i8 = -9;

const KNIGHT_OFFSETS: [i8; 8] = [-21, -19, -12, -8, 8, 12, 19, 21];
const BISHOP_OFFSETS: [i8; 4] = [UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT];
const ROOK_OFFSETS: [i8; 4] = [UP, LEFT, RIGHT, DOWN];
const QUEEN_OFFSETS: [i8; 8] = [UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT, UP, LEFT, RIGHT, DOWN];

#[derive(Clone)]
pub struct Mailbox {
    board: [Pieces; 120],
    curr_player: PieceColors,
    //In order: white kingside, white queenside, black kingside, black queenside
    castling_rights: CastleRights,
    //None if no en passant is possible, Some if possible by taking the position given with a pawn
    en_passant: Option<usize>,
    half_moves: usize,
    full_moves: usize,
    check: Option<bool>,
    white_king: usize,
    black_king: usize,
    previous_state: Option<Arc<Mailbox>>,
    black_attack_map: [Pieces; 120],
    white_attack_map: [Pieces; 120],
}

impl Mailbox {
    fn setup_board(fen: Option<&str>) -> Self {
        todo!()
    }
    fn get_valid_moves(&self) -> Vec<GameMove1d> {
        let mut moves = vec![];
        for (pos, piece) in self.board.iter().enumerate() {
            match piece {
                Pieces { piece_type: PieceTypes::Empty, .. } => continue,
                Pieces { piece_type: PieceTypes::Null, .. } => continue,
                Pieces { color: x, .. } if x != &self.curr_player => continue,
                Pieces { piece_type: PieceTypes::Knight, .. } => moves.extend(self.generate_moves(pos, &KNIGHT_OFFSETS, false)),
                Pieces { piece_type: PieceTypes::Rook, .. } => moves.extend(self.generate_moves(pos, &ROOK_OFFSETS, true)),
                Pieces { piece_type: PieceTypes::Bishop, .. } => moves.extend(self.generate_moves(pos, &BISHOP_OFFSETS, true)),
                Pieces { piece_type: PieceTypes::Queen, .. } => moves.extend(self.generate_moves(pos, &QUEEN_OFFSETS, true)),
                Pieces { piece_type: PieceTypes::King, .. } => moves.extend(self.generate_king_moves(pos)),
                Pieces { piece_type: PieceTypes::Pawn, .. } => moves.extend(self.generate_pawn_moves(pos)),
            };
        }
        return moves;
    }
    fn make_move(&self, mov: &GameMove1d) -> Self {
        todo!()
    }
    fn get_check(&self) -> Option<bool> {
        todo!()
    }
    fn get_curr_player(&self) -> PieceColors {
        self.curr_player
    }
    fn get_castle_rights(&self) -> CastleRights {
        self.castling_rights
    }
    fn check_legal_square(&self, pos: usize) -> bool {
        self.board[pos].piece_type == PieceTypes::Null
    }
    fn generate_moves(&self, pos: usize, offsets: &[i8], ray: bool) -> Vec<GameMove1d> {
        todo!()
    }
    fn generate_king_moves(&self, pos: usize) -> Vec<GameMove1d> {
        todo!()
    }
    fn generate_pawn_moves(&self, pos: usize) -> Vec<GameMove1d> {
        todo!()
    }
}

