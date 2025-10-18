#![allow(dead_code)]
use crate::board_structs::board_types::mailbox::Mailbox;
use crate::utils::gamemove1d::GameMove1d;
use crate::utils::time::TimeInfo;

pub struct MailboxMinimax;

impl MailboxMinimax {
    pub(crate) fn uci_infinite_find_move(
        game: Mailbox,
        available_moves: Option<Vec<GameMove1d>>,
    ) -> GameMove1d {
        // TODO: Calculate the allowed search time for this move

        // TODO: set up search thread, and listen for both stop command and search information
        // If stop command is issued, immediately stop search and return most recent best move
        let best_move: GameMove1d = Default::default();

        // TODO: if search ends normally, return best move
        best_move
    }
    pub(crate) fn uci_find_move(
        game: Mailbox,
        search_time: u128,
        available_moves: Option<Vec<GameMove1d>>,
        max_plies: Option<usize>,
        max_nodes: Option<usize>,
    ) -> GameMove1d {
        // TODO: set up search thread, and listen for both stop command and search information
        // If stop command is issued, immediately stop search and return most recent best move
        let best_move: GameMove1d = Default::default();

        // TODO: if search ends normally, return best move
        best_move
    }
    pub(crate) fn uci_search_mate(
        game: Mailbox,
        search_time: u128,
        available_moves: Option<Vec<GameMove1d>>,
        mate_moves: Option<usize>,
    ) -> GameMove1d {
        // TODO: set up search thread, and listen for both stop command and search information
        // If stop command is issued, immediately stop search and return most recent best move
        let best_move: GameMove1d = Default::default();

        // TODO: if search ends normally, return best move
        best_move
    }
}
