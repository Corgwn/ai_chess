use crate::utils::pieces::{Pieces, Pieces::*, BLACK, WHITE};

use std::collections::HashMap;
use crate::utils::game_move::GameMove;

pub fn is_terminal<T: crate::board_structs::board::Board>(game: &T, available_moves: &Vec<GameMove>) -> Option<i32> {
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
    [5, 10, 10,-20,-20, 10, 10,  5],
    [5, -5,-10,  0,  0,-10, -5,  5],
    [0,  0,  0, 20, 20,  0,  0,  0],
    [5,  5, 10, 25, 25, 10,  5,  5],
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
    [-50,-40,-30,-30,-30,-30,-40,-50],
    [-40,-20,  0,  5,  5,  0,-20,-40],
    [-30,  5, 10, 15, 15, 10,  5,-30],
    [-30,  0, 15, 20, 20, 15,  0,-30],
    [-30,  5, 15, 20, 20, 15,  5,-30],
    [-30,  0, 10, 15, 15, 10,  0,-30],
    [-40,-20,  0,  0,  0,  0,-20,-40],
    [-50,-40,-30,-30,-30,-30,-40,-50],
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
    [-20,-10,-10,-10,-10,-10,-10,-20],
    [-10,  5,  0,  0,  0,  0,  5,-10],
    [-10, 10, 10, 10, 10, 10, 10,-10],
    [-10,  0, 10, 10, 10, 10,  0,-10],
    [-10,  5,  5, 10, 10,  5,  5,-10],
    [-10,  0,  5, 10, 10,  5,  0,-10],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-20,-10,-10,-10,-10,-10,-10,-20],
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
    [0,  0,  0,  5,  5,  0,  0,  0],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [5, 10, 10, 10, 10, 10, 10,  5],
    [0,  0,  0,  0,  0,  0,  0,  0],
];
const EG_ROOK_TABLE: [[i32; 8]; 8] = [
    [1,  0,  0,  5,  5,  0,  0,  0],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [5, 10, 10, 10, 10, 10, 10,  5],
    [0,  0,  0,  0,  0,  0,  0,  0],
];
const MG_QUEEN_TABLE: [[i32; 8]; 8] = [
    [-20,-10,-10, -5, -5,-10,-10,-20],
    [-10,  0,  5,  0,  0,  0,  0,-10],
    [-10,  5,  5,  5,  5,  5,  0,-10],
    [0,  0,  5,  5,  5,  5,  0, -5],
    [-5,  0,  5,  5,  5,  5,  0, -5],
    [-10,  0,  5,  5,  5,  5,  0,-10],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-20,-10,-10, -5, -5,-10,-10,-20],
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
    [20, 30, 10,  0,  0, 10, 30, 20],
    [20, 20,  0,  0,  0,  0, 20, 20],
    [-10,-20,-20,-20,-20,-20,-20,-10],
    [-20,-30,-30,-40,-40,-30,-30,-20],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
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
fn get_position_value(piece: &Pieces, [row, col]: [usize; 2], game_time: bool) -> i32 {
    if game_time == ENDGAME {
        match piece {
            Empty => 0,
            Pawn(WHITE) => EG_PAWN_TABLE[row][col],
            Pawn(BLACK) => -EG_PAWN_TABLE[7 - row][col],
            King(WHITE) => EG_KING_TABLE[row][col],
            King(BLACK) => -EG_KING_TABLE[7 - row][col],
            Queen(WHITE) => EG_QUEEN_TABLE[row][col],
            Queen(BLACK) => -EG_QUEEN_TABLE[7 - row][col],
            Rook(WHITE) => EG_ROOK_TABLE[row][col],
            Rook(BLACK) => -EG_ROOK_TABLE[7 - row][col],
            Bishop(WHITE) => EG_BISHOP_TABLE[row][col],
            Bishop(BLACK) => -EG_BISHOP_TABLE[7 - row][col],
            Knight(WHITE) => EG_KNIGHT_TABLE[row][col],
            Knight(BLACK) => -EG_KNIGHT_TABLE[7 - row][col],
        }
    } else {
        match piece {
            Empty => 0,
            Pawn(WHITE) => MG_PAWN_TABLE[row][col],
            Pawn(BLACK) => -MG_PAWN_TABLE[7 - row][col],
            King(WHITE) => MG_KING_TABLE[row][col],
            King(BLACK) => -MG_KING_TABLE[7 - row][col],
            Queen(WHITE) => MG_QUEEN_TABLE[row][col],
            Queen(BLACK) => -MG_QUEEN_TABLE[7 - row][col],
            Rook(WHITE) => MG_ROOK_TABLE[row][col],
            Rook(BLACK) => -MG_ROOK_TABLE[7 - row][col],
            Bishop(WHITE) => MG_BISHOP_TABLE[row][col],
            Bishop(BLACK) => -MG_BISHOP_TABLE[7 - row][col],
            Knight(WHITE) => MG_KNIGHT_TABLE[row][col],
            Knight(BLACK) => -MG_KNIGHT_TABLE[7 - row][col],
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
            Empty => {}
            Queen(player) => result += [QUEEN_VAL, -QUEEN_VAL][*player as usize] * number,
            Rook(player) => result += [ROOK_VAL, -ROOK_VAL][*player as usize] * number,
            Bishop(player) => result += [BISHOP_VAL, -BISHOP_VAL][*player as usize] * number,
            Knight(player) => result += [KNIGHT_VAL, -KNIGHT_VAL][*player as usize] * number,
            King(player) => result += [KING_VAL, -KING_VAL][*player as usize] * number,
            Pawn(player) => result += [PAWN_VAL, -PAWN_VAL][*player as usize] * number,
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
    match piece_counts.get(&Bishop(game.get_curr_player())) {
        None => {}
        Some(x) => {
            if x > &1 {
                match game.get_curr_player() {
                    WHITE => result += 20,
                    BLACK => result -= 20,
                }
            }
        }
    }
    result
}
