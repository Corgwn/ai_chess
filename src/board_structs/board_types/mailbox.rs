use std::isize;
use std::sync::Arc;

use crate::utils::castling::CastleRights;
use crate::utils::checks::Checks;
use crate::utils::gamemove1d::{CastleTypes, GameMove1d, PassantTypes};
use crate::utils::pieces::{PieceColors, PieceTypes, Pieces};
use crate::utils::position::Position;

const UP_LEFT: i8 = 9;
const UP: i8 = 10;
const UP_RIGHT: i8 = 11;
const LEFT: i8 = -1;
const RIGHT: i8 = 1;
const DOWN_LEFT: i8 = -11;
const DOWN: i8 = -10;
const DOWN_RIGHT: i8 = -9;
const DOWN_TWICE: i8 = -20;
const UP_TWICE: i8 = 20;

const KNIGHT_OFFSETS: [i8; 8] = [-21, -19, -12, -8, 8, 12, 19, 21];
const BISHOP_OFFSETS: [i8; 4] = [UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT];
const ROOK_OFFSETS: [i8; 4] = [UP, LEFT, RIGHT, DOWN];
const QUEEN_OFFSETS: [i8; 8] = [
    UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT, UP, LEFT, RIGHT, DOWN,
];

#[derive(Clone)]
pub struct Mailbox {
    board: [Pieces; 120],
    curr_player: PieceColors,
    //In order: white kingside, white queenside, black kingside, black queenside
    castling_rights: CastleRights,
    //None if no en passant is possible, Some if possible by taking the position given with a pawn
    en_passant: Option<Position>,
    half_moves: u8,
    full_moves: u8,
    check: Option<Checks>,
    white_king: Position,
    black_king: Position,
    previous_state: Option<Arc<Mailbox>>,
    black_attack_map: [u8; 120],
    white_attack_map: [u8; 120],
}

impl Mailbox {
    fn setup_board(fen: Option<&str>) -> Self {
        todo!()
    }
    fn get_valid_moves(&self) -> Vec<GameMove1d> {
        let mut moves = vec![];
        for (i, piece) in self.board.iter().enumerate() {
            let pos = Position { value: i };
            match piece {
                Pieces {
                    piece_type: PieceTypes::Empty,
                    ..
                } => continue,
                Pieces {
                    piece_type: PieceTypes::Offboard,
                    ..
                } => continue,
                Pieces { color: x, .. } if x != &self.curr_player => continue,
                Pieces {
                    piece_type: PieceTypes::Knight,
                    ..
                } => moves.extend(self.generate_moves(pos, &KNIGHT_OFFSETS, false)),
                Pieces {
                    piece_type: PieceTypes::Rook,
                    ..
                } => moves.extend(self.generate_moves(pos, &ROOK_OFFSETS, true)),
                Pieces {
                    piece_type: PieceTypes::Bishop,
                    ..
                } => moves.extend(self.generate_moves(pos, &BISHOP_OFFSETS, true)),
                Pieces {
                    piece_type: PieceTypes::Queen,
                    ..
                } => moves.extend(self.generate_moves(pos, &QUEEN_OFFSETS, true)),
                Pieces {
                    piece_type: PieceTypes::King,
                    ..
                } => moves.extend(self.generate_king_moves(pos)),
                Pieces {
                    piece_type: PieceTypes::Pawn,
                    ..
                } => moves.extend(self.generate_pawn_moves(pos)),
            };
        }
        return moves;
    }
    fn make_move(&self, mov: &GameMove1d) -> Self {
        todo!()
    }
    fn get_check(&self) -> Option<Checks> {
        self.check
    }
    fn get_curr_player(&self) -> PieceColors {
        self.curr_player
    }
    fn get_castle_rights(&self) -> CastleRights {
        self.castling_rights
    }
    fn is_legal_square(&self, pos: Position) -> bool {
        self.board[pos.value].piece_type == PieceTypes::Offboard
    }
    fn generate_moves(&self, pos: Position, offsets: &[i8], ray: bool) -> Vec<GameMove1d> {
        let mut moves = vec![];

        for offset in offsets {
            let mut test_pos = pos;
            loop {
                test_pos.value = test_pos
                    .value
                    .checked_add_signed(*offset as isize)
                    .expect("Invalid position detected {test_pos}");

                if !self.is_legal_square(test_pos) {
                    break;
                }

                let capture: bool;
                match self.board[test_pos.value] {
                    Pieces {
                        piece_type: PieceTypes::Offboard,
                        ..
                    } => break,
                    Pieces {
                        piece_type: PieceTypes::Empty,
                        ..
                    } => {
                        capture = false;
                    }
                    Pieces { color: x, .. } if x == self.curr_player => {
                        break;
                    }
                    _ => {
                        capture = true;
                    }
                }

                moves.push(GameMove1d {
                    start: pos,
                    end: test_pos,
                    castle: None,
                    promote: None,
                    passant: None,
                    capture: capture,
                });

                if !ray {
                    break;
                }
            }
        }

        return moves;
    }
    fn generate_king_moves(&self, pos: Position) -> Vec<GameMove1d> {
        let mut moves = vec![];

        // Add standard moves
        for offset in QUEEN_OFFSETS {
            let mut test_pos = pos;
            test_pos.value = test_pos
                .value
                .checked_add_signed(offset as isize)
                .expect("Invalid position detected {test_pos}");

            if !self.is_legal_square(test_pos) {
                continue;
            }

            let capture: bool;
            match self.board[test_pos.value] {
                Pieces {
                    piece_type: PieceTypes::Offboard,
                    ..
                } => continue,
                Pieces {
                    piece_type: PieceTypes::Empty,
                    ..
                } => {
                    capture = false;
                }
                Pieces { color: x, .. } if x == self.curr_player => continue,
                _ => {
                    capture = true;
                }
            }

            // Check if move puts king in check

            moves.push(GameMove1d {
                start: pos,
                end: test_pos,
                castle: None,
                promote: None,
                passant: None,
                capture: capture,
            });
        }

        // Add castle moves
        match self.curr_player {
            PieceColors::White
                if (self.castling_rights.white_queen
                    && self.black_attack_map[24] == 0
                    && self.black_attack_map[23] == 0
                    && self.check != Some(Checks::White)) =>
            {
                moves.push(GameMove1d {
                    start: pos,
                    end: Position { value: 23 },
                    castle: Some(CastleTypes::WhiteQueen),
                    promote: None,
                    passant: None,
                    capture: false,
                })
            }
            PieceColors::White
                if (self.castling_rights.white_king
                    && self.black_attack_map[26] == 0
                    && self.black_attack_map[27] == 0
                    && self.check != Some(Checks::White)) =>
            {
                moves.push(GameMove1d {
                    start: pos,
                    end: Position { value: 27 },
                    castle: Some(CastleTypes::WhiteKing),
                    promote: None,
                    passant: None,
                    capture: false,
                })
            }
            PieceColors::Black
                if (self.castling_rights.black_queen
                    && self.white_attack_map[94] == 0
                    && self.white_attack_map[93] == 0
                    && self.check != Some(Checks::Black)) =>
            {
                moves.push(GameMove1d {
                    start: pos,
                    end: Position { value: 93 },
                    castle: Some(CastleTypes::BlackQueen),
                    promote: None,
                    passant: None,
                    capture: false,
                })
            }
            PieceColors::Black
                if (self.castling_rights.black_king
                    && self.white_attack_map[96] == 0
                    && self.white_attack_map[97] == 0
                    && self.check != Some(Checks::Black)) =>
            {
                moves.push(GameMove1d {
                    start: pos,
                    end: Position { value: 97 },
                    castle: Some(CastleTypes::BlackKing),
                    promote: None,
                    passant: None,
                    capture: false,
                })
            }
            _ => {}
        }

        return moves;
    }
    fn generate_pawn_moves(&self, pos: Position) -> Vec<GameMove1d> {
        todo!()
    }
}
