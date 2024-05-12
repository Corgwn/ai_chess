use crate::utils::game_move::GameMove;
use crate::utils::pieces::Pieces;

pub trait Board: Clone {
    fn setup_board(fen: Option<&str>) -> Self;
    fn get_valid_moves(&self) -> Vec<GameMove>;
    fn make_move(&self, mov: &GameMove) -> Self;
    fn get_check(&self) -> Option<bool>;
    fn get_curr_player(&self) -> bool;
    fn get_board_as_2d(&self) -> [[Pieces; 8]; 8];
    fn get_castle_rights(&self) -> [bool; 4];
}
