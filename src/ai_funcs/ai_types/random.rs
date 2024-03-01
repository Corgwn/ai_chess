use rand::prelude::SliceRandom;
use rand::thread_rng;
use crate::board_structs::board::Board;
use crate::utils::game_move::GameMove;

pub(crate) struct Random {
}

impl Random {
    pub fn find_move<T: Board>(&self, game: T, _: bool, _: u128) -> GameMove {
        let valid_moves = game.get_valid_moves();
        *valid_moves.choose(&mut thread_rng()).unwrap()
    }
}

