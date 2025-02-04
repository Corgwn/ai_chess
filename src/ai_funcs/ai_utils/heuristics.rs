use crate::utils::pieces::{Pieces, PieceColors::*, PieceTypes::*, BLACK, WHITE, PieceTypes};

use crate::utils::gamemove2d::GameMove2d;
use std::collections::HashMap;

pub fn is_terminal<T: crate::board_structs::board::Board>(
    game: &T,
    available_moves: &Vec<GameMove2d>,
) -> Option<i32> {
    if available_moves.is_empty() {
        if game.get_check().is_some() {
            Some([i32::MIN + 1, i32::MAX - 1][game.get_curr_player() as usize])
        } else {
            Some(0)
        }
    } else {
        None
    }
}

const MIDGAME: bool = false;
const ENDGAME: bool = true;

const PAWN_VAL: i32 = 100;
const BISHOP_VAL: i32 = 350;
const KNIGHT_VAL: i32 = 300;
const ROOK_VAL: i32 = 500;
const QUEEN_VAL: i32 = 900;
const KING_VAL: i32 = 400;

const MG_PAWN_TABLE: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [5, 10, 10, -20, -20, 10, 10, 5],
    [5, -5, -10, 0, 0, -10, -5, 5],
    [0, 0, 0, 20, 20, 0, 0, 0],
    [5, 5, 10, 25, 25, 10, 5, 5],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [0, 0, 0, 0, 0, 0, 0, 0],
];
const EG_PAWN_TABLE: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [13, 8, 8, 10, 13, 0, 2, -7],
    [4, 7, -6, 1, 0, -5, -1, -8],
    [13, 9, -3, -7, -7, -8, 3, -1],
    [32, 24, 13, 5, -2, 4, 17, 17],
    [94, 100, 85, 67, 56, 53, 82, 84],
    [178, 173, 158, 134, 147, 132, 165, 187],
    [0, 0, 0, 0, 0, 0, 0, 0],
];
const MG_KNIGHT_TABLE: [[i32; 8]; 8] = [
    [-50, -15, -30, -30, -30, -30, -15, -50],
    [-40, -20, 0, 5, 5, 0, -20, -40],
    [-30, 5, 10, 15, 15, 10, 5, -30],
    [-30, 0, 15, 20, 20, 15, 0, -30],
    [-30, 5, 15, 20, 20, 15, 5, -30],
    [-30, 0, 10, 15, 15, 10, 0, -30],
    [-40, -20, 0, 0, 0, 0, -20, -40],
    [-50, -15, -30, -30, -30, -30, -15, -50],
];
const EG_KNIGHT_TABLE: [[i32; 8]; 8] = [
    [-29, -51, -23, -15, -22, -18, -50, -64],
    [-42, -20, -10, -5, -2, -20, -23, -44],
    [-23, -3, -1, 15, 10, -3, -20, -22],
    [-18, -6, 16, 25, 16, 17, 4, -18],
    [-17, 3, 22, 22, 22, 11, 8, -18],
    [-24, -20, 10, 9, -1, -9, -19, -41],
    [-25, -8, -25, -2, -9, -25, -24, -52],
    [-58, -38, -13, -28, -31, -27, -63, -99],
];
const MG_BISHOP_TABLE: [[i32; 8]; 8] = [
    [-20, -10, -10, -10, -10, -10, -10, -20],
    [-10, 5, 0, 0, 0, 0, 5, -10],
    [-10, 10, 10, 10, 10, 10, 10, -10],
    [-10, 0, 10, 10, 10, 10, 0, -10],
    [-10, 5, 5, 10, 10, 5, 5, -10],
    [-10, 0, 5, 10, 10, 5, 0, -10],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-20, -10, -10, -10, -10, -10, -10, -20],
];
const EG_BISHOP_TABLE: [[i32; 8]; 8] = [
    [-23, -9, -23, -5, -9, -16, -5, -17],
    [-14, -18, -7, -1, 4, -9, -15, -27],
    [-12, -3, 8, 10, 13, 3, -7, -15],
    [-6, 3, 13, 19, 7, 10, -3, -9],
    [-3, 9, 12, 9, 14, 10, 3, 2],
    [2, -8, 0, -1, -2, 6, 0, 4],
    [-8, -4, 7, -12, -3, -13, -4, -14],
    [-14, -21, -11, -8, -7, -9, -17, -24],
];
const MG_ROOK_TABLE: [[i32; 8]; 8] = [
    [0, 0, 0, 5, 5, 0, 0, 0],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [5, 10, 10, 10, 10, 10, 10, 5],
    [0, 0, 0, 0, 0, 0, 0, 0],
];
const EG_ROOK_TABLE: [[i32; 8]; 8] = [
    [0, 0, 5, 5, 5, 5, 0, 0],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [5, 10, 10, 10, 10, 10, 10, 5],
    [0, 0, 0, 0, 0, 0, 0, 0],
];
const MG_QUEEN_TABLE: [[i32; 8]; 8] = [
    [-20, -10, -10, -5, -5, -10, -10, -20],
    [-10, 0, 5, 0, 0, 0, 0, -10],
    [-10, 5, 5, 5, 5, 5, 0, -10],
    [0, 0, 5, 5, 5, 5, 0, -5],
    [-5, 0, 5, 5, 5, 5, 0, -5],
    [-10, 0, 5, 5, 5, 5, 0, -10],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-20, -10, -10, -5, -5, -10, -10, -20],
];
const EG_QUEEN_TABLE: [[i32; 8]; 8] = [
    [-33, -28, -22, -43, -5, -32, -20, -41],
    [-22, -23, -30, -16, -16, -23, -36, -32],
    [-16, -27, 15, 6, 9, 17, 10, 5],
    [-18, 28, 19, 47, 31, 34, 39, 23],
    [3, 22, 24, 45, 57, 40, 57, 36],
    [-20, 6, 9, 49, 47, 35, 19, 9],
    [-17, 20, 32, 41, 58, 25, 30, 0],
    [-9, 22, 22, 27, 27, 19, 10, 20],
];
const MG_KING_TABLE: [[i32; 8]; 8] = [
    [20, 40, 10, 0, 0, 40, 30, 20],
    [20, 20, 0, 0, 0, 0, 20, 20],
    [-10, -20, -20, -20, -20, -20, -20, -10],
    [-20, -30, -30, -40, -40, -30, -30, -20],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
];
const EG_KING_TABLE: [[i32; 8]; 8] = [
    [-53, -34, -21, -11, -28, -14, -24, -43],
    [-27, -11, 4, 13, 14, 4, -5, -17],
    [-19, -3, 11, 21, 23, 16, 7, -9],
    [-18, -4, 21, 24, 27, 23, 9, -11],
    [-8, 22, 24, 27, 26, 33, 26, 3],
    [10, 17, 23, 15, 20, 45, 44, 13],
    [-12, 17, 14, 17, 17, 38, 23, 11],
    [-74, -35, -18, -18, -11, 15, 4, -17],
];

const SQUARE_CONTROL_TABLE: [[f32; 8]; 8] = [
    [0.50, 0.50, 0.50, 0.50, 0.50, 0.50, 0.50, 0.50],
    [0.75, 0.75, 0.75, 0.75, 0.75, 0.75, 0.75, 0.75],
    [1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00],
    [1.25, 1.25, 1.25, 1.35, 1.35, 1.25, 1.25, 1.25],
    [1.50, 1.50, 1.50, 1.60, 1.60, 1.50, 1.50, 1.50],
    [1.75, 1.75, 1.75, 1.75, 1.75, 1.75, 1.75, 1.75],
    [2.00, 2.00, 2.00, 2.00, 2.00, 2.00, 2.00, 2.00],
    [2.00, 2.00, 2.00, 2.00, 2.00, 2.00, 2.00, 2.00],
];

// for all squares in attack map
//   check piece on square
//     if enemy piece, multiply score by piece type
//   add square control * attack value * piece_mult to utility

fn get_position_value(piece: &Pieces, [row, col]: [usize; 2], game_time: bool) -> i32 {
    if game_time == ENDGAME {
        match piece {
            Pieces { piece_type: PieceTypes::Empty, .. } => 0,
            Pieces { piece_type: Pawn, color: White } => EG_PAWN_TABLE[row][col],
            Pieces { piece_type: Pawn, color: Black } => -EG_PAWN_TABLE[7 - row][col],
            Pieces { piece_type: King, color: White } => EG_KING_TABLE[row][col],
            Pieces { piece_type: King, color: Black } => -EG_KING_TABLE[7 - row][col],
            Pieces { piece_type: Queen, color: White } => EG_QUEEN_TABLE[row][col],
            Pieces { piece_type: Queen, color: Black } => -EG_QUEEN_TABLE[7 - row][col],
            Pieces { piece_type: Rook, color: White } => EG_ROOK_TABLE[row][col],
            Pieces { piece_type: Rook, color: Black } => -EG_ROOK_TABLE[7 - row][col],
            Pieces { piece_type: Bishop, color: White } => EG_BISHOP_TABLE[row][col],
            Pieces { piece_type: Bishop, color: Black } => -EG_BISHOP_TABLE[7 - row][col],
            Pieces { piece_type: Knight, color: White } => EG_KNIGHT_TABLE[row][col],
            Pieces { piece_type: Knight, color: Black } => -EG_KNIGHT_TABLE[7 - row][col],
            _ => 0,
        }
    } else {
        match piece {
            Pieces { piece_type: PieceTypes::Empty, .. } => 0,
            Pieces { piece_type: Pawn, color: White } => MG_PAWN_TABLE[row][col],
            Pieces { piece_type: Pawn, color: Black } => -MG_PAWN_TABLE[7 - row][col],
            Pieces { piece_type: King, color: White } => MG_KING_TABLE[row][col],
            Pieces { piece_type: King, color: Black } => -MG_KING_TABLE[7 - row][col],
            Pieces { piece_type: Queen, color: White } => MG_QUEEN_TABLE[row][col],
            Pieces { piece_type: Queen, color: Black } => -MG_QUEEN_TABLE[7 - row][col],
            Pieces { piece_type: Rook, color: White } => MG_ROOK_TABLE[row][col],
            Pieces { piece_type: Rook, color: Black } => -MG_ROOK_TABLE[7 - row][col],
            Pieces { piece_type: Bishop, color: White } => MG_BISHOP_TABLE[row][col],
            Pieces { piece_type: Bishop, color: Black } => -MG_BISHOP_TABLE[7 - row][col],
            Pieces { piece_type: Knight, color: White } => MG_KNIGHT_TABLE[row][col],
            Pieces { piece_type: Knight, color: Black } => -MG_KNIGHT_TABLE[7 - row][col],
            _ => 0,
        }
    }
}

pub fn heuristic<T: crate::board_structs::board::Board>(game: T) -> i32 {
    let mut result = 0;
    // let game_time = if game.get_board_as_2d()
    //     .iter()
    //     .flatten()
    //     .filter(|x| x != &&Empty && x != &&Pawn(WHITE) && x != &&Pawn(BLACK))
    //     .count()
    //     > 6
    // {
    //     MIDGAME
    // } else {
    //     ENDGAME
    // };
    let game_time = MIDGAME;
    // Count Pieces and score pieces on position
    let mut piece_counts: HashMap<Pieces, i32> = HashMap::new();
    for (i, line) in game.get_board_as_2d().iter().enumerate() {
        for (j, piece) in line.iter().enumerate() {
            *piece_counts.entry(*piece).or_default() += 1;
            //Get positional advantage of the piece
            result += get_position_value(piece, [i, j], game_time);
        }
    }
    // Add material value to evaluation
    for (piece, number) in piece_counts.iter() {
        match piece {
            Pieces { piece_type: PieceTypes::Null, .. } => {}
            Pieces { piece_type: PieceTypes::Empty, .. } => {}
            Pieces { piece_type: Queen, .. } => result += [QUEEN_VAL, -QUEEN_VAL][piece.get_color() as usize] * number,
            Pieces { piece_type: Rook, .. } => result += [ROOK_VAL, -ROOK_VAL][piece.get_color() as usize] * number,
            Pieces { piece_type: Bishop, ..} => result += [BISHOP_VAL, -BISHOP_VAL][piece.get_color() as usize] * number,
            Pieces { piece_type: Knight, ..} => result += [KNIGHT_VAL, -KNIGHT_VAL][piece.get_color() as usize] * number,
            Pieces { piece_type: King, ..} => result += [KING_VAL, -KING_VAL][piece.get_color() as usize] * number,
            Pieces { piece_type: Pawn, ..} => result += [PAWN_VAL, -PAWN_VAL][piece.get_color() as usize] * number,
        }
    }
    // Consider checks
    if let Some(player) = &game.get_check() {
        match *player {
            BLACK => result += 50,
            WHITE => result -= 50,
        }
    }
    // Prefer to have two bishops
    if let Some(x) = piece_counts.get(&Pieces { piece_type: Bishop, color: White }) {
        if x > &1 {
            result += 20;
        }
    }
    if let Some(x) = piece_counts.get(&Pieces { piece_type: Bishop, color: Black }) {
        if x > &1 {
            result -= 20;
        }
    }
    //Add value to having castle rights
    let rights = game.get_castle_rights();
    if rights[0] {
        result += 30;
    }
    if rights[1] {
        result += 30;
    }
    if rights[2] {
        result -= 30;
    }
    if rights[3] {
        result -= 30;
    }

    result
}
