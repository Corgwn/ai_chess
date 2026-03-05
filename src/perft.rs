use rusty_chess::{
    board::mailbox::Mailbox,
    utils::gamemove1d::{GameMove1d, PassantTypes},
};
use std::{env, fmt::Display, ops::AddAssign, str::FromStr};

#[derive(PartialEq, Eq, Debug)]
struct PerftStats {
    nodes: usize,
    captures: usize,
    castles: usize,
    promotions: usize,
    // checks: usize,
    ep: usize,
}

impl AddAssign for PerftStats {
    fn add_assign(&mut self, rhs: Self) {
        self.nodes += rhs.nodes;
        self.captures += rhs.captures;
        self.castles += rhs.castles;
        self.promotions += rhs.promotions;
        // self.checks += rhs.checks;
        self.ep += rhs.ep;
    }
}

impl Display for PerftStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Nodes-{}, Captures-{}, Castles-{}, Promotions-{}, EP-{}",
            self.nodes, self.captures, self.castles, self.promotions, self.ep
        )
    }
}

fn get_stats(moves: Vec<GameMove1d>) -> PerftStats {
    let nodes = moves.len();
    let mut ep = 0usize;
    let mut captures = 0usize;
    let mut castles = 0usize;
    let mut promotions = 0usize;
    // let mut checks = 0usize;

    for mov in moves.iter() {
        if mov.capture {
            captures += 1;
        }
        if mov.promote.is_some() {
            promotions += 1;
        }
        if mov.castle.is_some() {
            castles += 1;
        }
        if let Some(PassantTypes::PassantCapture(_)) = mov.passant {
            ep += 1;
        }
        // if is_check() {
        //     checks += 1;
        // }
    }

    PerftStats {
        nodes,
        captures,
        castles,
        promotions,
        // checks,
        ep,
    }
}

fn perft(depth: usize, game: Mailbox) -> PerftStats {
    let mut stats: PerftStats = PerftStats {
        nodes: 0,
        captures: 0,
        castles: 0,
        promotions: 0,
        // checks: 0,
        ep: 0,
    };
    let moves = game.get_valid_moves();

    if depth == 0 {
        return PerftStats {
            nodes: 1,
            captures: 0,
            castles: 0,
            promotions: 0,
            ep: 0,
        };
    }
    if depth == 1 {
        return get_stats(moves);
    }

    for mov in moves {
        let new_game = game.make_move(&mov);
        stats += perft(depth - 1, new_game);
    }

    stats
}

fn run_perft_n(n: usize, game: Mailbox) -> PerftStats {
    perft(n, game)
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
        let stats = run_perft_n(depth - 1, game.make_move(&mov));
        println!("{} {}", mov, stats.nodes);
        total_nodes += stats.nodes;
    }
    println!("\n{}", total_nodes);
}

#[test]
fn test_perft_start_pos() {
    let game = Mailbox::setup_board(None).unwrap();

    let start_pos_expected: [PerftStats; 7] = [
        //1
        PerftStats {
            nodes: 20,
            captures: 0,
            castles: 0,
            promotions: 0,
            //checks:0,
            ep: 0,
        },
        //2
        PerftStats {
            nodes: 400,
            captures: 0,
            castles: 0,
            promotions: 0,
            //checks:12,
            ep: 0,
        },
        //3
        PerftStats {
            nodes: 8902,
            captures: 34,
            castles: 0,
            promotions: 0,
            //checks:12
            ep: 0,
        },
        //4
        PerftStats {
            nodes: 197281,
            captures: 1576,
            castles: 0,
            promotions: 0,
            //checks: 469,
            ep: 0,
        },
        //5
        PerftStats {
            nodes: 4865609,
            captures: 82719,
            castles: 0,
            promotions: 0,
            //checks: 27351,
            ep: 258,
        },
        //6
        PerftStats {
            nodes: 119060324,
            captures: 2812008,
            castles: 0,
            promotions: 0,
            //checks: 809099,
            ep: 5248,
        },
        //7
        PerftStats {
            nodes: 3195901860,
            captures: 108329926,
            castles: 883453,
            promotions: 0,
            //checks: 33103848,
            ep: 319617,
        },
    ];
    for (index, value) in start_pos_expected.iter().enumerate() {
        let stats = run_perft_n(index + 1, game.clone());
        assert!(
            stats == *value,
            "Perft Start Position Test Failed:\nDepth: {}\nExpected: {}\nComputed: {}",
            index + 1,
            value,
            stats
        );
    }
}
