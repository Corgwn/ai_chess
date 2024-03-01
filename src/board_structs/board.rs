use crate::utils::game_move::GameMove;
use crate::utils::pieces::Pieces;

pub trait Board: Clone {
    fn read_from_fen(fen: String) -> Self;
    fn get_valid_moves(&self) -> Vec<GameMove>;
    fn make_move(&self, mov: GameMove) -> Self;
    fn get_check(&self) -> Option<bool>;
    fn get_curr_player(&self) -> bool;
    fn get_board_as_2d(&self) -> [[Pieces; 8]; 8];
}
