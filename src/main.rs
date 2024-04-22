use std::ops::Index;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, TryRecvError};
use std::thread::{sleep, JoinHandle};
use std::time::Instant;
use std::{thread, time};

use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;

use crate::ai_funcs::ai_types::abminimax::ABMinimax;
use crate::board_structs::board::Board;
use crate::board_structs::board_types::array2d::Array2D;
use crate::utils::game_move::GameMove;
use crate::utils::pieces::{BLACK, WHITE};

pub mod ai_funcs;
pub mod board_structs;
pub mod utils;

/// UCI chess engine written in Rust and featuring large amounts of custom options for what AI features to have, board representation to use, etc.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // /// Name of board representation to use.
    // #[arg(short, long)]
    // board_type: Option<String>,
    //
    // /// AI Type to use
    // #[arg(short, long)]
    // ai_type: Option<String>,
}

struct Engine {
    handle: JoinHandle<()>,
    transmit: Sender<&'static str>,
}

fn filter_uci_moves(args: &Vec<&str>) -> Vec<GameMove> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[abcdefgh]\d[abcdefgh]\d[qrkb]?").unwrap();
    }
    args.iter()
        .filter(|x| RE.is_match(x))
        .map(GameMove::from_str)
        .collect()
}

fn uci_engine() {
    // Print engine id
    println!("id name rustyai");
    println!("id author Corgwn");

    // Build Engine structs
    let mut board = Array2D::setup_board(None);
    let mut engine_handle: Option<Engine> = None;

    // Ready UCI terminal, and start command input
    println!("uciok");
    loop {
        let mut command_full = String::new();
        std::io::stdin().read_line(&mut command_full).unwrap();
        let args: Vec<&str> = command_full.split(' ').collect();
        match args[0] {
            "isready" => println!("readyok"),
            "setoption" => {}
            "register" => {}
            "ucinewgame" => {
                board = Array2D::setup_board(None);
            }
            "position" if args.contains(&"startpos") => {
                board = Array2D::setup_board(None);
                if args.contains(&"moves") {
                    for input_move in filter_uci_moves(&args) {
                        board = board.make_move(input_move);
                    }
                }
            }
            "position" if args.contains(&"fen") => {
                board = Array2D::setup_board(Some(args[2]));
                if args.contains(&"moves") {
                    for input_move in filter_uci_moves(&args) {
                        board = board.make_move(input_move);
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

                // Start engine and save thread handle to later join if needed
                let (tx, rx) = mpsc::channel();
                let handle = thread::spawn(move || {
                    let best_move = ABMinimax::uci_timed_find_move(
                        &board,
                        time_to_move,
                        rx,
                        searchmoves,
                        max_plies,
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
                let (tx, rx) = mpsc::channel();
                let handle = thread::spawn(move || {
                    let best_move = ABMinimax::uci_infinite_find_move(&board, rx, searchmoves);
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
    let mut time_left: u128 = 900000000000;
    while !game.get_valid_moves().is_empty() {
        let start = Instant::now();
        let turn: bool = (turn_num % 2) != 0;
        let next_move = match turn {
            WHITE => player1.find_move(game, WHITE, time_left),
            BLACK => player2.find_move(game, BLACK, time_left),
        };

        let turn_color = match turn {
            WHITE => "WHITE",
            BLACK => "BLACK",
        };

        time_left = match time_left.checked_sub(start.elapsed().as_nanos()) {
            None => break,
            Some(x) => x,
        };
        turn_num += 1;
        game = game.make_move(next_move);
        println!(
            "\nTurn number: {} | Player: {} | Move: {} | Time Left: {}\n",
            turn_num, turn_color, next_move, time_left
        );
    }
}

fn main() {
    // let args = Args::parse();
    loop {
        let mut mode = String::new();
        std::io::stdin().read_line(&mut mode).unwrap();
        match mode.as_str() {
            "uci" => {
                uci_engine();
                break;
            }
            "test" => {
                run_sample_game();
                break;
            }
            "quit" => break,
            _ => println!("Engine type not supported"),
        }
    }
}
