use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::{thread, time};

use lazy_static::lazy_static;
use regex::Regex;

use crate::ai::manual;
use crate::ai::negamax_mailbox::MailboxNegamax;
use crate::board::mailbox::Mailbox;
use crate::utils::gamemove1d::GameMove1d;
use crate::utils::pieces::{PieceColors, BLACK, WHITE};

pub mod ai;
pub mod board;
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
                    MailboxNegamax::uci_find_move(
                        game,
                        time_to_move,
                        searchmoves,
                        max_plies,
                        max_nodes,
                        rx,
                    );
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
                    MailboxNegamax::uci_infinite_find_move(game, rx, searchmoves);
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
                    MailboxNegamax::uci_find_move(
                        game,
                        time_to_move,
                        searchmoves,
                        max_plies,
                        max_nodes,
                        rx,
                    );
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
    let mut game = Mailbox::setup_board(None).unwrap();

    let mut turn_num: usize = 0;
    let mut white_time: u128 = 60000;
    let mut black_time: u128 = 60000;
    let inc: u128 = 600;
    println!("Game starting!");
    println!("{}", game);
    while !game.get_valid_moves().is_empty() {
        let turn: bool = !turn_num.is_multiple_of(2);
        let turn_color = match turn {
            WHITE => "WHITE",
            BLACK => "BLACK",
        };

        let turn_start = time::Instant::now();
        let search_time = if turn_color == "WHITE" {
            white_time / 20 + inc / 2
        } else {
            black_time / 20 + inc / 2
        };
        let (_tx, rx) = mpsc::channel();
        println!(
            "Starting search for player {}, searching for {}ms",
            turn_color, search_time
        );
        let next_move =
            MailboxNegamax::uci_find_move(game.clone(), search_time, None, None, None, rx);
        let turn_duration = turn_start.elapsed().as_millis();

        if turn_color == "WHITE" {
            if white_time.checked_sub(turn_duration).is_some() {
                white_time = white_time + inc - turn_duration;
            } else {
                println!("White has run out of time, Black has won on time.");
                break;
            }
        } else if black_time.checked_sub(turn_duration).is_some() {
            black_time = black_time + inc - turn_duration;
        } else {
            println!("Black has run out of time, White has won on time.");
            break;
        }

        turn_num += 1;
        game = game.make_move(&next_move);
        println!(
            "\nTurn number: {turn_num} | Player: {turn_color} | Move: {next_move} | wtime: {white_time} | btime: {black_time} | inc: {inc}\n",
        );
        println!("{}", game);
    }
}
fn run_manual_game() {
    let mut game = Mailbox::setup_board(None).unwrap();

    let mut turn_num: usize = 0;
    println!("Game starting!");
    println!("{}", game);
    while !game.get_valid_moves().is_empty() {
        let turn: bool = !turn_num.is_multiple_of(2);
        let turn_color = match turn {
            WHITE => "WHITE",
            BLACK => "BLACK",
        };

        println!("Starting search for player {}", turn_color);
        let next_move = manual::Manual::find_move_1d();

        turn_num += 1;
        game = game.make_move(&next_move);
        println!("\nTurn number: {turn_num} | Player: {turn_color} | Move: {next_move}\n",);
        println!("{}", game);
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
                run_manual_game();
                break;
            }
            "sample" => {
                run_sample_game();
                break;
            }
            "quit" => break,
            _ => println!("Engine type not supported"),
        }
    }
}
