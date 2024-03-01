use rand::prelude::SliceRandom;
use rand::thread_rng;
use crate::board_structs::board::Board;

fn find_random_move<T: Board>(fen: String) -> Option<String> {
    let game = T::read_from_fen(fen);
    let valid_moves = game.get_valid_moves();
    let mov = valid_moves.choose(&mut thread_rng())?;
    Some(mov.to_string())
}