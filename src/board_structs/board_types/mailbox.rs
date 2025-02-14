use std::isize;
use std::sync::Arc;

use crate::board_structs::board;
use crate::utils::castling::CastleRights;
use crate::utils::checks::Checks;
use crate::utils::chess_errors::ChessError;
use crate::utils::gamemove1d::{to_num, CastleTypes, GameMove1d, PassantTypes};
use crate::utils::pieces::{self, PieceColors, PieceTypes, Pieces};
use crate::utils::position::Position;

const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

const UL: i8 = 9;
const U: i8 = 10;
const UR: i8 = 11;
const L: i8 = -1;
const R: i8 = 1;
const DL: i8 = -11;
const D: i8 = -10;
const DR: i8 = -9;
const DD: i8 = -20;
const UU: i8 = 20;

const KNIGHT_OFFSETS: [i8; 8] = [-21, -19, -12, -8, 8, 12, 19, 21];
const BISHOP_OFFSETS: [i8; 4] = [UL, UR, DL, DR];
const ROOK_OFFSETS: [i8; 4] = [U, L, R, D];
const QUEEN_OFFSETS: [i8; 8] = [UL, UR, DL, DR, U, L, R, D];

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
    fn setup_board(fen: Option<&str>) -> Result<Self, ChessError> {
        let mut fields = fen.unwrap_or(START_POSITION).split_ascii_whitespace();
        // Read board positions
        let board = fields.next().unwrap();
        let mut board_state: [Pieces; 120] = [Pieces {
            piece_type: PieceTypes::Offboard,
            color: PieceColors::Empty,
        }; 120];
        // Set Board state
        let mut index: usize = 21;
        for char in board.chars() {
            match char {
                '/' => index += 2,
                x if x.is_ascii_digit() => {
                    let num_empty = x.to_digit(10).unwrap() as usize;
                    for i in 0..num_empty {
                        board_state[index] = Pieces {
                            piece_type: PieceTypes::Empty,
                            color: PieceColors::Empty,
                        };
                        index += 1;
                    }
                }
                y => {
                    board_state[index] = Pieces::from(&y);
                    index += 1;
                }
            }
        }

        // Read Current Move
        let curr_player = match fields.next().unwrap() {
            "b" => PieceColors::Black,
            "w" => PieceColors::White,
            _ => return Err(ChessError::FENParseError),
        };

        // Read Castling Rights
        let temp = fields.next().unwrap();
        let castling_rights = CastleRights {
            white_king: temp.contains("K"),
            white_queen: temp.contains("Q"),
            black_king: temp.contains("k"),
            black_queen: temp.contains("q"),
        };

        // Read En Passant Targets
        let temp = fields.next().unwrap();
        let en_passant: Option<Position> = if !temp.eq("-") {
            Some(Position {
                value: to_num(temp),
            })
        } else {
            None
        };

        // Read Move Numbers
        let half_moves = fields.next().unwrap().parse::<u8>().unwrap();
        let full_moves = fields.next().unwrap().parse::<u8>().unwrap();

        // Find King Positions
        let mut black_king = Position { value: 95 };
        let mut white_king = Position { value: 25 };
        for (index, piece) in board_state.iter().enumerate() {
            match piece {
                Pieces {
                    piece_type: PieceTypes::King,
                    color: PieceColors::Black,
                } => black_king = Position { value: index },
                Pieces {
                    piece_type: PieceTypes::King,
                    color: PieceColors::White,
                } => white_king = Position { value: index },
                _ => continue,
            }
        }

        // Generate Attack Maps
        let (white_attack_map, black_attack_map) = Mailbox::generate_attack_maps(board_state);

        // Get Checks
        Ok(Mailbox {
            board: board_state,
            curr_player,
            castling_rights,
            en_passant,
            half_moves,
            full_moves,
            check: verify_checks(board_state, white_king, black_king),
            white_king,
            black_king,
            previous_state: None,
            black_attack_map,
            white_attack_map,
        })
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

    fn is_legal_square(board: [Pieces; 120], pos: Position) -> bool {
        board[pos.value].piece_type == PieceTypes::Offboard
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

                if !Mailbox::is_legal_square(self.board, test_pos) {
                    break;
                }

                // Check destination of move and handle accordingly
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

                // Check if move puts king in check
                if self.verify_curr_player_check(&pos, &test_pos) {
                    break;
                }

                // Move is good to add to move list
                moves.push(GameMove1d {
                    start: pos,
                    end: test_pos,
                    castle: None,
                    promote: None,
                    passant: None,
                    capture: capture,
                });

                // Only loop if piece can slide
                if !ray {
                    break;
                }
            }
        }

        return moves;
    }
    fn generate_king_moves(&self, start: Position) -> Vec<GameMove1d> {
        let mut moves = vec![];

        // Generate standard moves
        for offset in QUEEN_OFFSETS {
            let mut test_end = start;
            test_end.value = test_end
                .value
                .checked_add_signed(offset as isize)
                .expect("Invalid position detected {test_pos}");

            if !Mailbox::is_legal_square(self.board, test_end) {
                continue;
            }

            // Check destination of move and handle accordingly
            let capture: bool;
            match self.board[test_end.value] {
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
            };

            // Check if move puts king in check
            if self.verify_curr_player_check(&start, &test_end) {
                continue;
            }

            // Move is good to add to move list
            moves.push(GameMove1d {
                start,
                end: test_end,
                castle: None,
                promote: None,
                passant: None,
                capture: capture,
            });
        }

        // If someone is in check, either game is over or you can't castle
        // so king moves limited to standard moves
        if self.check.is_some() {
            return moves;
        }

        // Add castle moves
        match self.curr_player {
            PieceColors::White => {
                if self.castling_rights.white_queen
                    && self.black_attack_map[23..=24].iter().all(|&x| x == 0)
                    && self.board[23..=24]
                        .iter()
                        .all(|&i| i.piece_type == PieceTypes::Empty)
                {
                    moves.push(GameMove1d {
                        start,
                        end: Position { value: 23 },
                        castle: Some(CastleTypes::WhiteQueen),
                        promote: None,
                        passant: None,
                        capture: false,
                    })
                } else if self.castling_rights.white_king
                    && self.black_attack_map[26..=27].iter().all(|&x| x == 0)
                    && self.board[26..=27]
                        .iter()
                        .all(|&i| i.piece_type == PieceTypes::Empty)
                {
                    moves.push(GameMove1d {
                        start,
                        end: Position { value: 27 },
                        castle: Some(CastleTypes::WhiteKing),
                        promote: None,
                        passant: None,
                        capture: false,
                    })
                }
            }
            PieceColors::Black => {
                if self.castling_rights.black_queen
                    && self.white_attack_map[93..=94].iter().all(|&x| x == 0)
                    && self.board[93..=94]
                        .iter()
                        .all(|&i| i.piece_type == PieceTypes::Empty)
                {
                    moves.push(GameMove1d {
                        start,
                        end: Position { value: 93 },
                        castle: Some(CastleTypes::BlackQueen),
                        promote: None,
                        passant: None,
                        capture: false,
                    })
                } else if self.castling_rights.black_king
                    && self.white_attack_map[96..=97].iter().all(|&x| x == 0)
                    && self.board[96..=97]
                        .iter()
                        .all(|&i| i.piece_type == PieceTypes::Empty)
                {
                    moves.push(GameMove1d {
                        start,
                        end: Position { value: 97 },
                        castle: Some(CastleTypes::BlackKing),
                        promote: None,
                        passant: None,
                        capture: false,
                    })
                }
            }
            _ => {}
        }

        return moves;
    }

    fn generate_pawn_moves(&self, start: Position) -> Vec<GameMove1d> {
        let mut moves = vec![];

        if self.board[start.value].color != self.curr_player {
            return vec![];
        }

        // Define moves for current player's pawn
        let double_forward: i8;
        let forward: i8;
        if self.curr_player == PieceColors::White {
            double_forward = UU;
            forward = U;
        } else {
            double_forward = DD;
            forward = D;
        };
        let capture_left = forward - 1;
        let capture_right = forward + 1;

        // Standard move
        let test_end = Position {
            value: start
                .value
                .checked_add_signed(forward as isize)
                .expect("Invalid position found"),
        };
        match self.board[test_end.value] {
            Pieces {
                piece_type: PieceTypes::Empty,
                color,
            } => {
                if self.verify_curr_player_check(&start, &test_end) {
                    match test_end.value {
                        21..=28 if color == PieceColors::Black => {
                            moves.push(GameMove1d {
                                start,
                                end: test_end,
                                castle: None,
                                promote: Some(Pieces {
                                    piece_type: PieceTypes::Queen,
                                    color,
                                }),
                                passant: None,
                                capture: false,
                            });
                            moves.push(GameMove1d {
                                start: start,
                                end: test_end,
                                castle: None,
                                promote: Some(Pieces {
                                    piece_type: PieceTypes::Bishop,
                                    color,
                                }),
                                passant: None,
                                capture: false,
                            });
                            moves.push(GameMove1d {
                                start,
                                end: test_end,
                                castle: None,
                                promote: Some(Pieces {
                                    piece_type: PieceTypes::Knight,
                                    color,
                                }),
                                passant: None,
                                capture: false,
                            });
                            moves.push(GameMove1d {
                                start,
                                end: test_end,
                                castle: None,
                                promote: Some(Pieces {
                                    piece_type: PieceTypes::Rook,
                                    color,
                                }),
                                passant: None,
                                capture: false,
                            });
                        }
                        91..=98 if color == PieceColors::White => {
                            moves.push(GameMove1d {
                                start,
                                end: test_end,
                                castle: None,
                                promote: Some(Pieces {
                                    piece_type: PieceTypes::Queen,
                                    color,
                                }),
                                passant: None,
                                capture: false,
                            });
                            moves.push(GameMove1d {
                                start: start,
                                end: test_end,
                                castle: None,
                                promote: Some(Pieces {
                                    piece_type: PieceTypes::Bishop,
                                    color,
                                }),
                                passant: None,
                                capture: false,
                            });
                            moves.push(GameMove1d {
                                start,
                                end: test_end,
                                castle: None,
                                promote: Some(Pieces {
                                    piece_type: PieceTypes::Knight,
                                    color,
                                }),
                                passant: None,
                                capture: false,
                            });
                            moves.push(GameMove1d {
                                start,
                                end: test_end,
                                castle: None,
                                promote: Some(Pieces {
                                    piece_type: PieceTypes::Rook,
                                    color,
                                }),
                                passant: None,
                                capture: false,
                            });
                        }
                        _ => {
                            moves.push(GameMove1d {
                                start,
                                end: test_end,
                                castle: None,
                                promote: None,
                                passant: None,
                                capture: false,
                            });
                        }
                    }
                }
            }
            _ => {}
        }

        // Double Move if possible
        let test_end = Position {
            value: start
                .value
                .checked_add_signed(double_forward as isize)
                .expect("Invalid position found"),
        };
        let test_half = Position {
            value: start
                .value
                .checked_add_signed(forward as isize)
                .expect("Invalid position found"),
        };
        if self.board[test_half.value].piece_type == PieceTypes::Empty {
            if self.board[test_half.value].piece_type == PieceTypes::Empty {
                if self.verify_curr_player_check(&start, &test_end) {
                    moves.push(GameMove1d {
                        start: start,
                        end: test_end,
                        castle: None,
                        promote: None,
                        passant: Some(PassantTypes::PassantAvailable(test_half)),
                        capture: false,
                    });
                }
            }
        }

        // Captures
        for capture in [capture_left, capture_right] {
            let test_end = Position {
                value: start
                    .value
                    .checked_add_signed(capture as isize)
                    .expect("Invalid position found"),
            };
            match self.board[test_end.value] {
                Pieces {
                    piece_type: PieceTypes::Empty,
                    ..
                } => {}
                Pieces {
                    piece_type: PieceTypes::Offboard,
                    ..
                } => {}
                Pieces { color, .. } if color == self.curr_player => {}
                Pieces { color, .. } if color != self.curr_player => {
                    if self.verify_curr_player_check(&start, &test_end) {
                        match test_end.value {
                            21..=28 if color == PieceColors::Black => {
                                moves.push(GameMove1d {
                                    start,
                                    end: test_end,
                                    castle: None,
                                    promote: Some(Pieces {
                                        piece_type: PieceTypes::Queen,
                                        color,
                                    }),
                                    passant: None,
                                    capture: true,
                                });
                                moves.push(GameMove1d {
                                    start: start,
                                    end: test_end,
                                    castle: None,
                                    promote: Some(Pieces {
                                        piece_type: PieceTypes::Bishop,
                                        color,
                                    }),
                                    passant: None,
                                    capture: true,
                                });
                                moves.push(GameMove1d {
                                    start,
                                    end: test_end,
                                    castle: None,
                                    promote: Some(Pieces {
                                        piece_type: PieceTypes::Knight,
                                        color,
                                    }),
                                    passant: None,
                                    capture: true,
                                });
                                moves.push(GameMove1d {
                                    start,
                                    end: test_end,
                                    castle: None,
                                    promote: Some(Pieces {
                                        piece_type: PieceTypes::Rook,
                                        color,
                                    }),
                                    passant: None,
                                    capture: true,
                                });
                            }
                            91..=98 if color == PieceColors::White => {
                                moves.push(GameMove1d {
                                    start,
                                    end: test_end,
                                    castle: None,
                                    promote: Some(Pieces {
                                        piece_type: PieceTypes::Queen,
                                        color,
                                    }),
                                    passant: None,
                                    capture: true,
                                });
                                moves.push(GameMove1d {
                                    start: start,
                                    end: test_end,
                                    castle: None,
                                    promote: Some(Pieces {
                                        piece_type: PieceTypes::Bishop,
                                        color,
                                    }),
                                    passant: None,
                                    capture: true,
                                });
                                moves.push(GameMove1d {
                                    start,
                                    end: test_end,
                                    castle: None,
                                    promote: Some(Pieces {
                                        piece_type: PieceTypes::Knight,
                                        color,
                                    }),
                                    passant: None,
                                    capture: true,
                                });
                                moves.push(GameMove1d {
                                    start,
                                    end: test_end,
                                    castle: None,
                                    promote: Some(Pieces {
                                        piece_type: PieceTypes::Rook,
                                        color,
                                    }),
                                    passant: None,
                                    capture: true,
                                });
                            }
                            _ => {
                                moves.push(GameMove1d {
                                    start,
                                    end: test_end,
                                    castle: None,
                                    promote: None,
                                    passant: None,
                                    capture: true,
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        return moves;
    }

    //
    fn verify_curr_player_check(&self, start: &Position, end: &Position) -> bool {
        let white_king: Position;
        let black_king: Position;
        match self.board[start.value] {
            Pieces {
                piece_type: PieceTypes::King,
                color: PieceColors::White,
            } => {
                white_king = *end;
                black_king = self.black_king;
            }
            Pieces {
                piece_type: PieceTypes::King,
                color: PieceColors::Black,
            } => {
                white_king = self.white_king;
                black_king = *end;
            }
            _ => {
                white_king = self.white_king;
                black_king = self.black_king;
            }
        }
        let mut test_board = self.board;
        test_board[end.value] = test_board[start.value];
        test_board[start.value] = Pieces {
            piece_type: PieceTypes::Empty,
            color: PieceColors::Empty,
        };

        match self.curr_player {
            PieceColors::White => is_white_checked(test_board, white_king),
            PieceColors::Black => is_black_checked(test_board, black_king),
            PieceColors::Empty => false,
        }
    }

    fn generate_attack_maps(board: [Pieces; 120]) -> ([u8; 120], [u8; 120]) {
        let mut white_attack_map = [0u8; 120];
        let mut black_attack_map = [0u8; 120];

        for (index, piece) in board.iter().enumerate() {
            match piece {
                Pieces {
                    piece_type: PieceTypes::Offboard,
                    ..
                } => continue,
                Pieces {
                    piece_type: PieceTypes::Empty,
                    ..
                } => continue,
                Pieces {
                    piece_type: x,
                    color,
                } => match x {
                    PieceTypes::Knight => {
                        for offset in KNIGHT_OFFSETS {
                            let test_pos = Position {
                                value: index + offset as usize,
                            };
                            if !Mailbox::is_legal_square(board, test_pos) {
                                continue;
                            }
                            match color {
                                PieceColors::Black => black_attack_map[test_pos.value] += 1,
                                PieceColors::White => white_attack_map[test_pos.value] += 1,
                                PieceColors::Empty => {}
                            }
                        }
                    }
                    PieceTypes::Bishop => {
                        for offset in BISHOP_OFFSETS {
                            let mut test_pos = Position { value: index };
                            loop {
                                test_pos = Position {
                                    value: test_pos.value + offset as usize,
                                };
                                match board[test_pos.value].piece_type {
                                    PieceTypes::Offboard => break,
                                    PieceTypes::Empty => match color {
                                        PieceColors::Black => black_attack_map[test_pos.value] += 1,
                                        PieceColors::White => white_attack_map[test_pos.value] += 1,
                                        PieceColors::Empty => {}
                                    },
                                    _ => match color {
                                        PieceColors::Black => black_attack_map[test_pos.value] += 1,
                                        PieceColors::White => white_attack_map[test_pos.value] += 1,
                                        PieceColors::Empty => {}
                                    },
                                }
                                if !Mailbox::is_legal_square(board, test_pos) {
                                    break;
                                }
                                match color {
                                    PieceColors::Black => black_attack_map[test_pos.value] += 1,
                                    PieceColors::White => white_attack_map[test_pos.value] += 1,
                                    PieceColors::Empty => {}
                                }
                            }
                        }
                    }
                    PieceTypes::Rook => {
                        for offset in ROOK_OFFSETS {
                            let mut test_pos = Position { value: index };
                            loop {
                                test_pos = Position {
                                    value: test_pos.value + offset as usize,
                                };
                                match board[test_pos.value].piece_type {
                                    PieceTypes::Offboard => break,
                                    PieceTypes::Empty => match color {
                                        PieceColors::Black => black_attack_map[test_pos.value] += 1,
                                        PieceColors::White => white_attack_map[test_pos.value] += 1,
                                        PieceColors::Empty => {}
                                    },
                                    _ => match color {
                                        PieceColors::Black => black_attack_map[test_pos.value] += 1,
                                        PieceColors::White => white_attack_map[test_pos.value] += 1,
                                        PieceColors::Empty => {}
                                    },
                                }
                                if !Mailbox::is_legal_square(board, test_pos) {
                                    break;
                                }
                                match color {
                                    PieceColors::Black => black_attack_map[test_pos.value] += 1,
                                    PieceColors::White => white_attack_map[test_pos.value] += 1,
                                    PieceColors::Empty => {}
                                }
                            }
                        }
                    }
                    PieceTypes::Queen => {
                        for offset in QUEEN_OFFSETS {
                            let mut test_pos = Position { value: index };
                            loop {
                                test_pos = Position {
                                    value: test_pos.value + offset as usize,
                                };
                                match board[test_pos.value].piece_type {
                                    PieceTypes::Offboard => break,
                                    PieceTypes::Empty => match color {
                                        PieceColors::Black => black_attack_map[test_pos.value] += 1,
                                        PieceColors::White => white_attack_map[test_pos.value] += 1,
                                        PieceColors::Empty => {}
                                    },
                                    _ => match color {
                                        PieceColors::Black => black_attack_map[test_pos.value] += 1,
                                        PieceColors::White => white_attack_map[test_pos.value] += 1,
                                        PieceColors::Empty => {}
                                    },
                                }
                                if !Mailbox::is_legal_square(board, test_pos) {
                                    break;
                                }
                                match color {
                                    PieceColors::Black => black_attack_map[test_pos.value] += 1,
                                    PieceColors::White => white_attack_map[test_pos.value] += 1,
                                    PieceColors::Empty => {}
                                }
                            }
                        }
                    }
                    PieceTypes::Pawn => match color {
                        PieceColors::White => {
                            white_attack_map[index.checked_add_signed(UR as isize).unwrap()] += 1;
                            white_attack_map[index.checked_add_signed(UL as isize).unwrap()] += 1;
                        }
                        PieceColors::Black => {
                            black_attack_map[index.checked_add_signed(DR as isize).unwrap()] += 1;
                            black_attack_map[index.checked_add_signed(DL as isize).unwrap()] += 1;
                        }
                        _ => {}
                    },
                    PieceTypes::King => {
                        for offset in QUEEN_OFFSETS {
                            let test_pos = Position {
                                value: index + offset as usize,
                            };
                            if !Mailbox::is_legal_square(board, test_pos) {
                                continue;
                            }
                            match color {
                                PieceColors::Black => black_attack_map[test_pos.value] += 1,
                                PieceColors::White => white_attack_map[test_pos.value] += 1,
                                PieceColors::Empty => {}
                            }
                        }
                    }
                    _ => {}
                },
            }
        }

        return (white_attack_map, black_attack_map);
    }
}

fn verify_checks(
    board: [Pieces; 120],
    white_king: Position,
    black_king: Position,
) -> Option<Checks> {
    if is_black_checked(board, black_king) {
        return Some(Checks::Black);
    }
    if is_white_checked(board, white_king) {
        return Some(Checks::White);
    }
    return None;
}

fn is_white_checked(board: [Pieces; 120], white_king: Position) -> bool {
    // Check knights
    for offset in KNIGHT_OFFSETS {
        let test_pos = Position {
            value: white_king
                .value
                .checked_add_signed(offset as isize)
                .expect("Invalid position found"),
        };
        match board[test_pos.value] {
            Pieces {
                piece_type: PieceTypes::Knight,
                color: PieceColors::Black,
            } => {
                return true;
            }
            _ => {}
        }
    }

    // Check diagonal sliders
    for offset in BISHOP_OFFSETS {
        let mut test_pos = white_king;
        loop {
            test_pos = Position {
                value: test_pos
                    .value
                    .checked_add_signed(offset as isize)
                    .expect("Invalid position found"),
            };
            match board[test_pos.value] {
                Pieces {
                    piece_type: PieceTypes::Bishop,
                    color: PieceColors::Black,
                }
                | Pieces {
                    piece_type: PieceTypes::Queen,
                    color: PieceColors::Black,
                } => {
                    return true;
                }
                Pieces {
                    piece_type: PieceTypes::Empty,
                    ..
                } => {
                    continue;
                }
                _ => {
                    break;
                }
            }
        }
    }

    // Check orthogonal sliders
    for offset in ROOK_OFFSETS {
        let mut test_pos = white_king;
        loop {
            test_pos = Position {
                value: test_pos
                    .value
                    .checked_add_signed(offset as isize)
                    .expect("Invalid position found"),
            };
            match board[test_pos.value] {
                Pieces {
                    piece_type: PieceTypes::Rook,
                    color: PieceColors::Black,
                }
                | Pieces {
                    piece_type: PieceTypes::Queen,
                    color: PieceColors::Black,
                } => {
                    return true;
                }
                Pieces {
                    piece_type: PieceTypes::Empty,
                    ..
                } => {
                    continue;
                }
                _ => {
                    break;
                }
            }
        }
    }

    // Check pawns
    for offset in [UR, UL] {
        let test_pos = Position {
            value: white_king
                .value
                .checked_add_signed(offset as isize)
                .expect("Invalid position found"),
        };
        match board[test_pos.value] {
            Pieces {
                piece_type: PieceTypes::Pawn,
                color: PieceColors::Black,
            } => {
                return true;
            }
            _ => {}
        }
    }

    return false;
}

fn is_black_checked(board: [Pieces; 120], black_king: Position) -> bool {
    // Check knights
    for offset in KNIGHT_OFFSETS {
        let test_pos = Position {
            value: black_king
                .value
                .checked_add_signed(offset as isize)
                .expect("Invalid position found"),
        };
        match board[test_pos.value] {
            Pieces {
                piece_type: PieceTypes::Knight,
                color: PieceColors::White,
            } => {
                return true;
            }
            _ => {}
        }
    }

    // Check diagonal sliders
    for offset in BISHOP_OFFSETS {
        let mut test_pos = black_king;
        loop {
            test_pos = Position {
                value: test_pos
                    .value
                    .checked_add_signed(offset as isize)
                    .expect("Invalid position found"),
            };
            match board[test_pos.value] {
                Pieces {
                    piece_type: PieceTypes::Bishop,
                    color: PieceColors::White,
                }
                | Pieces {
                    piece_type: PieceTypes::Queen,
                    color: PieceColors::White,
                } => {
                    return true;
                }
                Pieces {
                    piece_type: PieceTypes::Empty,
                    ..
                } => {
                    continue;
                }
                _ => {
                    break;
                }
            }
        }
    }

    // Check orthogonal sliders
    for offset in ROOK_OFFSETS {
        let mut test_pos = black_king;
        loop {
            test_pos = Position {
                value: test_pos
                    .value
                    .checked_add_signed(offset as isize)
                    .expect("Invalid position found"),
            };
            match board[test_pos.value] {
                Pieces {
                    piece_type: PieceTypes::Rook,
                    color: PieceColors::White,
                }
                | Pieces {
                    piece_type: PieceTypes::Queen,
                    color: PieceColors::White,
                } => {
                    return true;
                }
                Pieces {
                    piece_type: PieceTypes::Empty,
                    ..
                } => {
                    continue;
                }
                _ => {
                    break;
                }
            }
        }
    }

    // Check pawns
    for offset in [DR, DL] {
        let test_pos = Position {
            value: black_king
                .value
                .checked_add_signed(offset as isize)
                .expect("Invalid position found"),
        };
        match board[test_pos.value] {
            Pieces {
                piece_type: PieceTypes::Pawn,
                color: PieceColors::White,
            } => {
                return true;
            }
            _ => {}
        }
    }

    return false;
}
