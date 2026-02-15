use crate::board::array2d::Array2D;
use crate::utils::gamemove2d::GameMove2d;
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub(crate) struct Random2D {}

impl Random2D {
    pub fn find_move(&self, game: Array2D, _: bool, _: u128) -> GameMove2d {
        let valid_moves = game.get_valid_moves();
        *valid_moves.choose(&mut thread_rng()).unwrap()
    }
}
