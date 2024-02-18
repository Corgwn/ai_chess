use crate::board_structs::board_types::array2d::GameState;
use crate::utils::pieces::{Pieces, Pieces::*, BLACK, WHITE};

use std::collections::HashMap;

pub fn term(game: &GameState) -> Option<i32> {
    if game.available_moves.is_empty() {
        if game.check.is_some() {
            Some([i32::MIN + 1, i32::MAX - 1][game.curr_move as usize])
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
    [0, 0, 0, -5, -5, 0, 0, 0],
    [0, 2, 3, 4, 4, 3, 2, 0],
    [0, 4, 6, 10, 10, 6, 4, 0],
    [0, 6, 9, 10, 10, 9, 6, 0],
    [4, 8, 12, 16, 16, 12, 8, 4],
    [5, 10, 15, 20, 20, 15, 10, 5],
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
    [-105, -21, -58, -33, -17, -28, -19, -23],
    [-29, -53, -12, -3, -1, 18, -14, -19],
    [-23, -9, 12, 10, 19, 17, 25, -16],
    [-13, 4, 16, 13, 28, 19, 21, -8],
    [-9, 17, 19, 53, 37, 69, 18, 22],
    [-47, 60, 37, 65, 84, 129, 73, 44],
    [-73, -41, 72, 36, 23, 62, 7, -17],
    [-167, -89, -34, -49, 61, -97, -15, -107],
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
    [-33, -3, -14, -21, -13, -12, -39, -21],
    [4, 15, 16, 0, 7, 21, 33, 1],
    [0, 15, 15, 15, 14, 27, 18, 10],
    [-6, 13, 13, 26, 34, 12, 10, 4],
    [-4, 5, 19, 50, 37, 37, 7, -2],
    [-16, 37, 43, 40, 35, 50, 37, -2],
    [-26, 16, -18, -13, 30, 59, 18, -47],
    [-29, 4, -82, -37, -25, -42, 7, -8],
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
    [-19, -13, 1, 17, 16, 7, -37, -26],
    [-44, -16, -20, -9, -1, 11, -6, -71],
    [-45, -25, -16, -17, 3, 0, -5, -33],
    [-36, -26, -12, -1, 9, -7, 6, -23],
    [-24, -11, 7, 26, 24, 35, -8, -20],
    [-5, 19, 26, 36, 17, 45, 61, 16],
    [27, 32, 58, 62, 80, 67, 26, 44],
    [32, 42, 32, 51, 63, 9, 31, 43],
];
const EG_ROOK_TABLE: [[i32; 8]; 8] = [
    [-9, 2, 3, -1, -5, -13, 4, -20],
    [-6, -6, 0, 2, -9, -9, -11, -3],
    [-4, 0, -5, -1, -7, -12, -8, -16],
    [3, 5, 8, 4, -5, -6, -8, -11],
    [4, 3, 13, 1, 2, 1, -1, 2],
    [7, 7, 7, 5, 4, -3, -5, -3],
    [11, 13, 13, 11, -3, 3, 8, 3],
    [13, 10, 18, 15, 12, 12, 8, 5],
];
const MG_QUEEN_TABLE: [[i32; 8]; 8] = [
    [-1, -18, -9, 10, -15, -25, -31, -50],
    [-35, -8, 11, 2, 8, 15, -3, 1],
    [-14, 2, -11, -2, -5, 2, 14, 5],
    [-9, -26, -9, -10, -2, -4, 3, -3],
    [-27, -27, -16, -16, -1, 17, -2, 1],
    [-13, -17, 7, 8, 29, 56, 47, 57],
    [-24, -39, -5, 1, -16, 57, 28, 54],
    [-28, 0, 29, 12, 59, 44, 43, 45],
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
    [-15, 36, 12, -54, 8, -28, 24, 14],
    [1, 7, -8, -64, -43, -16, 9, 8],
    [-14, -14, -22, -46, -44, -30, -15, -27],
    [-49, -1, -27, -39, -46, -44, -33, -51],
    [-17, -20, -12, -27, -30, -25, -14, -36],
    [-9, 24, 2, -16, -20, 6, 22, -22],
    [29, -1, -20, -7, -8, -4, -38, -29],
    [-65, 23, 16, -15, -56, -34, 2, 13],
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

pub fn heuristic(game: &GameState) -> i32 {
    let mut result = 0;
    let game_time = if game
        .board
        .iter()
        .flatten()
        .filter(|x| x != &&Empty && x != &&Pawn(WHITE) && x != &&Pawn(BLACK))
        .count()
        > 6
    {
        MIDGAME
    } else {
        ENDGAME
    };
    //Count Pieces and score pieces on position
    let mut piece_counts: HashMap<Pieces, i32> = HashMap::new();
    for (i, line) in game.board.iter().enumerate() {
        for (j, piece) in line.iter().enumerate() {
            *piece_counts.entry(*piece).or_default() += 1;
            //Get positional advantage of the piece
            result += get_position_value(piece, [i, j], game_time);
        }
    }
    //Add material value ot evaluation
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
    //Consider checks
    if let Some(player) = &game.check {
        if *player == game.curr_move {
            match *player {
                BLACK => result += 20,
                WHITE => result -= 20,
            }
        }
    }
    //Prefer to have two bishops
    if piece_counts.contains_key(&Bishop(game.curr_move)) && piece_counts.get(&Bishop(game.curr_move)).unwrap() > &1 {
        result += 20;
    }
    result
}
