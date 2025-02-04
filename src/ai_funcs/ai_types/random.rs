use crate::board_structs::board::Board;
use crate::utils::gamemove2d::GameMove2d;
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub(crate) struct Random {}

impl Random {
    pub fn find_move<T: Board>(&self, game: T, _: bool, _: u128) -> GameMove2d {
        let valid_moves = game.get_valid_moves();
        *valid_moves.choose(&mut thread_rng()).unwrap()
    }
}
