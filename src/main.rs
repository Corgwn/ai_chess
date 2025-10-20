use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

use ai_funcs::ai_types::abminimax2d::ABMinimax;
use lazy_static::lazy_static;
use regex::Regex;

use crate::ai_funcs::ai_types::negamax1d;
use crate::board_structs::board::Board;
use crate::board_structs::board_types::array2d::Array2D;
use crate::board_structs::board_types::mailbox::Mailbox;
use crate::utils::gamemove1d::GameMove1d;
use crate::utils::pieces::{PieceColors, BLACK, WHITE};

pub mod ai_funcs;
pub mod board_structs;
pub mod utils;

struct Engine {
    handle: JoinHandle<()>,
    transmit: Sender<&'static str>,
}

fn filter_uci_moves(args: &[&str]) -> Vec<GameMove1d> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[abcdefgh]\d[abcdefgh]\d[qrkb]?").unwrap();
    }
    args.iter()
        .filter(|x| RE.is_match(x))
        .map(GameMove1d::from_str)
        .collect()
}

fn uci_engine() {
    // Print engine id
    println!("id name rustyai");
    println!("id author Corgwn");

    // Build Engine structs
    let mut board = Mailbox::setup_board(None).unwrap();
    let mut engine_handle: Option<Engine> = None;
    let mut _move_count = 0;

    // Ready UCI terminal, and start command input
    println!("uciok");
    loop {
        let mut command_full = String::new();
        std::io::stdin().read_line(&mut command_full).unwrap();
        let args: Vec<&str> = command_full.trim().split(' ').collect();
        match args[0] {
            "isready" => println!("readyok"),
            "setoption" => {}
            "register" => {}
            "ucinewgame" => {
                board = Mailbox::setup_board(None).unwrap();
                _move_count = 0
            }
            "position" if args.contains(&"startpos") => {
                board = Mailbox::setup_board(None).unwrap();
                _move_count = 0;
                if args.contains(&"moves") {
                    let x = filter_uci_moves(&args);
                    for input_move in x {
                        board = board.make_move(&input_move);
                        _move_count += 1;
                    }
                }
            }
            "position" if args.contains(&"fen") => {
                board = Mailbox::setup_board(Some(args[2])).unwrap();
                if args.contains(&"moves") {
                    for input_move in filter_uci_moves(&args) {
                        board = board.make_move(&input_move);
                        _move_count += 1;
                    }
                }
            }
            "go" if args.contains(&"movetime") => {
                // Find time to move
                let move_time_index = args.iter().position(|&r| r == "movetime").unwrap();
                let time_to_move = args[move_time_index + 1].parse().unwrap();

                // Parse searchmoves
                let searchmoves = if args.contains(&"searchmoves") {
                    Some(filter_uci_moves(&args))
                } else {
                    None
                };

                // Parse max depth
                let mut max_plies = None;
                if let Some(depth_index) = args.iter().position(|&r| r == "depth") {
                    max_plies = Some(args[depth_index + 1].parse().unwrap());
                }

                // Parse max nodes
                let mut max_nodes = None;
                if let Some(max_nodes_index) = args.iter().position(|&r| r == "nodes") {
                    max_nodes = Some(args[max_nodes_index + 1].parse().unwrap());
                }

                // Start engine and save thread handle to later join if needed
                let game = board.clone();
                let (tx, rx) = mpsc::channel();
                let handle = thread::spawn(move || {
                    let best_move = negamax1d::MailboxNegamax::uci_find_move(
                        game,
                        time_to_move,
                        searchmoves,
                        max_plies,
                        max_nodes,
                        rx,
                    );
                    println!("bestmove {}", best_move)
                });
                engine_handle = Some(Engine {
                    handle,
                    transmit: tx,
                });
            }
            "go" if args.contains(&"infinite") => {
                // Parse searchmoves
                let searchmoves = if args.contains(&"searchmoves") {
                    Some(filter_uci_moves(&args))
                } else {
                    None
                };

                // Start engine and save thread handle to later join if needed
                let game = board.clone();
                let (tx, rx) = mpsc::channel();
                let handle = thread::spawn(move || {
                    let best_move =
                        negamax1d::MailboxNegamax::uci_infinite_find_move(game, rx, searchmoves);
                    println!("bestmove {}", best_move)
                });
                engine_handle = Some(Engine {
                    handle,
                    transmit: tx,
                });
            }
            "go" => {
                // Find time to move
                // Get time remaining
                let time_remaining: u128 = if board.get_curr_player() == PieceColors::White {
                    let move_time_index = args.iter().position(|&r| r == "wtime").unwrap();
                    args[move_time_index + 1].parse().unwrap()
                } else {
                    let move_time_index = args.iter().position(|&r| r == "btime").unwrap();
                    args[move_time_index + 1].parse().unwrap()
                };
                let increment: u128 = if board.get_curr_player() == PieceColors::White
                    && args.contains(&"winc")
                {
                    let increment_index = args.iter().position(|&r| r == "winc").unwrap();
                    args[increment_index + 1].parse().unwrap()
                } else if board.get_curr_player() == PieceColors::Black && args.contains(&"binc") {
                    let increment_index = args.iter().position(|&r| r == "binc").unwrap();
                    args[increment_index + 1].parse().unwrap()
                } else {
                    0
                };
                // Calculate time to search
                let time_to_move = time_remaining / 20 + increment / 2;

                // Parse searchmoves
                let searchmoves = if args.contains(&"searchmoves") {
                    Some(filter_uci_moves(&args))
                } else {
                    None
                };

                // Parse max depth
                let mut max_plies = None;
                if let Some(depth_index) = args.iter().position(|&r| r == "depth") {
                    max_plies = Some(args[depth_index + 1].parse().unwrap());
                }

                // Parse max nodes
                let mut max_nodes = None;
                if let Some(max_nodes_index) = args.iter().position(|&r| r == "nodes") {
                    max_nodes = Some(args[max_nodes_index + 1].parse().unwrap());
                }

                // Start engine and save thread handle to later join if needed
                let game = board.clone();
                let (tx, rx) = mpsc::channel();
                let handle = thread::spawn(move || {
                    let best_move = negamax1d::MailboxNegamax::uci_find_move(
                        game,
                        time_to_move,
                        searchmoves,
                        max_plies,
                        max_nodes,
                        rx,
                    );
                    println!("bestmove {}", best_move)
                });
                engine_handle = Some(Engine {
                    handle,
                    transmit: tx,
                });
            }
            "stop" => {
                if let Some(x) = engine_handle {
                    if !x.handle.is_finished() {
                        let _ = x.transmit.send("stop");
                        let _ = x.handle.join();
                    }
                    engine_handle = None;
                }
            }
            "ponder" => {}
            "ponderhit" => {}
            "quit" => break,
            _ => {}
        }
    }
}

fn run_sample_game() {
    let mut game = Array2D::setup_board(None);
    let player1 = ABMinimax {};
    let player2 = ABMinimax {};

    let mut turn_num: usize = 0;
    while !game.get_valid_moves().is_empty() {
        let turn: bool = (turn_num % 2) != 0;
        let next_move = match turn {
            WHITE => player1.find_move(game, WHITE, 30000000000),
            BLACK => player2.find_move(game, BLACK, 30000000000),
        };

        let turn_color = match turn {
            WHITE => "WHITE",
            BLACK => "BLACK",
        };

        turn_num += 1;
        game = game.make_move(&next_move);
        println!(
            "\nTurn number: {} | Player: {} | Move: {}\n",
            turn_num, turn_color, next_move
        );
        print_board(&game);
    }
}

fn main() {
    // let args = Args::parse();
    loop {
        let mut mode = String::new();
        std::io::stdin().read_line(&mut mode).unwrap();
        match mode.trim() {
            "uci" => {
                uci_engine();
                break;
            }
            "man" => {
                run_sample_game();
                break;
            }
            "quit" => break,
            _ => println!("Engine type not supported"),
        }
    }
}

fn print_board<T: Board>(game: &T) {
    let board = game.get_board_as_2d();
    println!("-----------------");
    for row in board.iter().rev() {
        print!("|");
        for col in row {
            print!("{}|", col);
        }
        println!("\n-----------------");
    }
}
