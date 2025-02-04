use crate::{board_structs::board::Board, utils::gamemove2d::GameMove2d};

pub(crate) struct Manual {}

impl Manual {
    pub fn find_move<T: Board>(&self, game: T, _: bool, _: u128) -> GameMove2d {
        let mut command_full = String::new();
        std::io::stdin().read_line(&mut command_full).unwrap();
        let args: Vec<&str> = command_full.trim().split(' ').collect();
        GameMove2d::from_str(&args[0])
    }
}
