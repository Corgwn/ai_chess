use crate::{board_structs::board::Board, utils::game_move::GameMove};

pub(crate) struct Manual {}

impl Manual {
    pub fn find_move<T: Board>(&self, game: T, _: bool, _: u128) -> GameMove {
        let mut command_full = String::new();
        std::io::stdin().read_line(&mut command_full).unwrap();
        let args: Vec<&str> = command_full.trim().split(' ').collect();
        GameMove::from_str(&args[0])
    }
}
