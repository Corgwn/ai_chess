use std::str::FromStr;

use crate::utils::{gamemove1d::GameMove1d, gamemove2d::GameMove2d};

pub struct Manual {}

impl Manual {
    pub fn find_move_2d() -> GameMove2d {
        let mut command_full = String::new();
        std::io::stdin().read_line(&mut command_full).unwrap();
        let args: Vec<&str> = command_full.trim().split(' ').collect();
        GameMove2d::from_str(&args[0])
    }
    pub fn find_move_1d() -> GameMove1d {
        let mut command_full = String::new();
        std::io::stdin().read_line(&mut command_full).unwrap();
        let args: Vec<&str> = command_full.trim().split(' ').collect();
        GameMove1d::from_str(args[0]).unwrap()
    }
}
