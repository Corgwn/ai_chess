use rusty_chess::{
    board::mailbox::Mailbox,
    utils::gamemove1d::{GameMove1d, PassantTypes},
};
use std::{env, fmt::Display, ops::AddAssign, str::FromStr};

fn perft(depth: usize, game: Mailbox) -> usize {
    let mut nodes: usize = 0;
    let moves = game.get_valid_moves();

    if depth == 0 {
        return 1;
    }
    if depth == 1 {
        return moves.len();
    }

    for mov in moves {
        let new_game = game.make_move(&mov);
        nodes += perft(depth - 1, new_game);
    }

    nodes
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let depth: usize = args[1].parse::<usize>().unwrap();
    let fen: String = args[2].clone();
    let mut game = Mailbox::setup_board(Some(&fen)).unwrap();
    if args.len() == 4 {
        let moves: Vec<GameMove1d> = args[3]
            .split(" ")
            .map(|x| GameMove1d::from_str(x).unwrap())
            .collect();
        for mov in moves {
            game = game.make_move(&mov);
        }
    }

    //Depth of 0, forced value of 1
    if depth == 0 {
        println!("1");
    }

    let mut total_nodes: usize = 0;
    // Other depths, run perft for every possible move at depth-1
    for mov in game.get_valid_moves() {
        let node_count = perft(depth - 1, game.make_move(&mov));
        println!("{} {}", mov, node_count);
        total_nodes += node_count;
    }
    println!("\n{}", total_nodes);
}

#[test]
fn test_perft_start_pos() {
    let game = Mailbox::setup_board(None).unwrap();

    let start_pos_expected: [usize; 5] = [
        20, 400, 8_902, 197_281, 4_865_609,
        // 119_060_324,
    ];
    for (index, value) in start_pos_expected.iter().enumerate() {
        let nodes = perft(index + 1, game.clone());
        assert!(
            nodes == *value,
            "Perft Start Position Test Failed:\nDepth: {}\nExpected: {}\nComputed: {}",
            index + 1,
            value,
            nodes
        );
    }
}

#[test]
fn test_perft_kiwipete() {
    let game = Mailbox::setup_board(Some(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    ))
    .unwrap();

    let kiwipete_pos_expected: [usize; 5] = [
        48,
        2_039,
        97_862,
        4_085_603,
        193_690_690,
        // 8_031_647_685,
    ];
    for (index, value) in kiwipete_pos_expected.iter().enumerate() {
        let nodes = perft(index + 1, game.clone());
        assert!(
            nodes == *value,
            "Perft Kiwipete Test Failed:\nDepth: {}\nExpected: {}\nComputed: {}",
            index + 1,
            value,
            nodes
        );
    }
}
