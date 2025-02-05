use crate::board_structs::board;
use crate::board_structs::board::Board;
use crate::utils::gamemove2d::to_let;
use crate::utils::gamemove2d::to_num;
use crate::utils::gamemove2d::CastleTypes;
use crate::utils::gamemove2d::GameMove2d;
use crate::utils::gamemove2d::PassantTypes;
use crate::utils::pieces::{
    PieceColors, PieceColors::*, PieceTypes, PieceTypes::*, Pieces, BLACK, WHITE,
};
use std::cmp::PartialEq;

const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const KNIGHT_OFFSETS: [[i32; 2]; 8] = [
    [-2, -1],
    [-2, 1],
    [2, -1],
    [2, 1],
    [-1, -2],
    [-1, 2],
    [1, -2],
    [1, 2],
];
const ROOK_OFFSETS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];
const BISHOP_OFFSETS: [[i32; 2]; 4] = [[-1, -1], [-1, 1], [1, -1], [1, 1]];
const ROYAL_OFFSETS: [[i32; 2]; 8] = [
    [-1, 0],
    [1, 0],
    [0, -1],
    [0, 1],
    [-1, -1],
    [-1, 1],
    [1, -1],
    [1, 1],
];
const EMPTY_PIECE: Pieces = Pieces {
    piece_type: PieceTypes::Empty,
    color: PieceColors::Empty,
};

/* Game state representation and member functions */
#[derive(Clone, Debug, Copy)]
pub struct Array2D {
    pub board: [[Pieces; 8]; 8],
    //False represents white, true represents black
    pub curr_move: bool,
    //In order: white kingside, white queenside, black kingside, black queenside
    castling_rights: [bool; 4],
    //None if no en passant is possible, Some if possible by taking the position given with a pawn
    en_passant: Option<[usize; 2]>,
    half_moves: usize,
    full_moves: usize,
    check: Option<bool>,
    white_king: [usize; 2],
    black_king: [usize; 2],
    white_attack_map: [[u8; 8]; 8],
    black_attack_map: [[u8; 8]; 8],
}

impl Array2D {
    fn generate_attack_maps(board: [[Pieces; 8]; 8]) -> [[[u8; 8]; 8]; 2] {
        let mut maps = [[[0; 8]; 8]; 2];
        // Generate white map
        for (i, row) in board.iter().enumerate() {
            for (j, piece) in row.iter().enumerate() {
                match piece {
                    Pieces {
                        piece_type: Knight, ..
                    } => {
                        for offset in KNIGHT_OFFSETS {
                            let test_pos: [i32; 2] = [i as i32 + offset[0], j as i32 + offset[1]];
                            if is_in_bounds(test_pos) {
                                maps[piece.get_color() as usize][test_pos[0] as usize]
                                    [test_pos[1] as usize] += 1;
                            }
                        }
                    }
                    Pieces {
                        piece_type: Rook, ..
                    } => {
                        'dir: for offset in ROOK_OFFSETS {
                            let mut test_pos: [i32; 2] =
                                [i as i32 + offset[0], j as i32 + offset[1]];
                            loop {
                                if !is_in_bounds(test_pos) {
                                    continue 'dir;
                                }
                                if board[test_pos[0] as usize][test_pos[1] as usize].piece_type
                                    != PieceTypes::Empty
                                {
                                    maps[piece.get_color() as usize][test_pos[0] as usize]
                                        [test_pos[1] as usize] += 1;
                                    continue 'dir;
                                }
                                maps[piece.get_color() as usize][test_pos[0] as usize]
                                    [test_pos[1] as usize] += 1;
                                test_pos = [test_pos[0] + offset[0], test_pos[1] + offset[1]];
                            }
                        }
                    }
                    Pieces {
                        piece_type: Bishop, ..
                    } => {
                        'dir: for offset in BISHOP_OFFSETS {
                            let mut test_pos: [i32; 2] =
                                [i as i32 + offset[0], j as i32 + offset[1]];
                            loop {
                                if !is_in_bounds(test_pos) {
                                    continue 'dir;
                                }
                                if board[test_pos[0] as usize][test_pos[1] as usize].piece_type
                                    != PieceTypes::Empty
                                {
                                    maps[piece.get_color() as usize][test_pos[0] as usize]
                                        [test_pos[1] as usize] += 1;
                                    continue 'dir;
                                }
                                maps[piece.get_color() as usize][test_pos[0] as usize]
                                    [test_pos[1] as usize] += 1;
                                test_pos = [test_pos[0] + offset[0], test_pos[1] + offset[1]];
                            }
                        }
                    }
                    Pieces {
                        piece_type: Queen, ..
                    } => {
                        'dir: for offset in ROYAL_OFFSETS {
                            let mut test_pos: [i32; 2] =
                                [i as i32 + offset[0], j as i32 + offset[1]];
                            loop {
                                if !is_in_bounds(test_pos) {
                                    continue 'dir;
                                }
                                if board[test_pos[0] as usize][test_pos[1] as usize].piece_type
                                    != PieceTypes::Empty
                                {
                                    maps[piece.get_color() as usize][test_pos[0] as usize]
                                        [test_pos[1] as usize] += 1;
                                    continue 'dir;
                                }
                                maps[piece.get_color() as usize][test_pos[0] as usize]
                                    [test_pos[1] as usize] += 1;
                                test_pos = [test_pos[0] + offset[0], test_pos[1] + offset[1]];
                            }
                        }
                    }
                    Pieces {
                        piece_type: King, ..
                    } => {
                        'dir: for offset in ROYAL_OFFSETS {
                            let test_pos: [i32; 2] = [i as i32 + offset[0], j as i32 + offset[1]];
                            if !is_in_bounds(test_pos) {
                                continue 'dir;
                            }
                            maps[piece.get_color() as usize][test_pos[0] as usize]
                                [test_pos[1] as usize] += 1;
                        }
                    }
                    Pieces {
                        piece_type: Pawn,
                        color: Black,
                    } => {
                        let test_pos = [i as i32 - 1, j as i32 - 1];
                        if is_in_bounds(test_pos) {
                            maps[piece.get_color() as usize][test_pos[0] as usize]
                                [test_pos[1] as usize] += 1;
                        }
                        let test_pos = [i as i32 - 1, j as i32 + 1];
                        if is_in_bounds(test_pos) {
                            maps[piece.get_color() as usize][test_pos[0] as usize]
                                [test_pos[1] as usize] += 1;
                        }
                    }
                    Pieces {
                        piece_type: Pawn,
                        color: White,
                    } => {
                        let test_pos = [i as i32 + 1, j as i32 - 1];
                        if is_in_bounds(test_pos) {
                            maps[piece.get_color() as usize][test_pos[0] as usize]
                                [test_pos[1] as usize] += 1;
                        }
                        let test_pos = [i as i32 + 1, j as i32 + 1];
                        if is_in_bounds(test_pos) {
                            maps[piece.get_color() as usize][test_pos[0] as usize]
                                [test_pos[1] as usize] += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        maps
    }
}

impl board::Board for Array2D {
    fn setup_board(fen: Option<&str>) -> Self {
        let mut fields = fen.unwrap_or(START_POSITION).split_ascii_whitespace();
        //Read board positions
        let temp = fields.next().unwrap();
        let board = temp.split('/');
        let mut board_state: [[Pieces; 8]; 8] = [[EMPTY_PIECE; 8]; 8];
        for (index, line) in board.enumerate() {
            let mut file: usize = 0;
            for piece in line.chars() {
                //Move the index forward when there are spaces
                if piece.is_ascii_digit() {
                    file += piece.to_digit(10).unwrap() as usize;
                }
                //Add piece to board
                else {
                    board_state[7 - index][file] = Pieces::from(&piece);
                    file += 1;
                }
            }
        }

        //Read current move
        let player: bool = match fields.next().unwrap() {
            "b" => BLACK,
            "w" => WHITE,
            _ => panic!("Invalid FEN String"),
        };

        //Read castling rights
        let temp = fields.next().unwrap();
        let rights: [bool; 4] = [
            temp.contains('K'),
            temp.contains('Q'),
            temp.contains('k'),
            temp.contains('q'),
        ];

        //Read En Passant targets
        let temp = fields.next().unwrap();
        let passant: Option<[usize; 2]> = if !temp.eq("-") {
            let mut chars = temp.chars();
            let col = to_num(chars.next().unwrap());
            let row = (chars.next().unwrap().to_digit(10).unwrap() - 1) as usize;
            Some([row, col])
        } else {
            None
        };

        //Read move numbers
        let half_moves = fields.next().unwrap().parse::<usize>().unwrap();
        let full_moves = fields.next().unwrap().parse::<usize>().unwrap();

        //Find king positions
        let mut black_king = [0, 0];
        let mut white_king = [0, 0];
        for (i, row) in board_state.iter().enumerate() {
            for (j, square) in row.iter().enumerate() {
                match square {
                    Pieces {
                        piece_type: King,
                        color: Black,
                    } => black_king = [i, j],
                    Pieces {
                        piece_type: King,
                        color: White,
                    } => white_king = [i, j],
                    _ => continue,
                }
            }
        }
        // Calculate Attack Maps
        let [white_attack_map, black_attack_map] = Array2D::generate_attack_maps(board_state);

        let mut game = Array2D {
            board: board_state,
            curr_move: player,
            castling_rights: rights,
            en_passant: passant,
            half_moves,
            full_moves,
            check: None,
            white_king,
            black_king,
            white_attack_map,
            black_attack_map,
        };

        //Calculate if a king is in check
        game.check = who_is_checked(&game);

        game
    }

    fn get_valid_moves(&self) -> Vec<GameMove2d> {
        let mut moves = Vec::new();

        for i in 0..8 {
            for j in 0..8 {
                let piece = self.board[i][j];
                match piece {
                    Pieces {
                        piece_type: PieceTypes::Null,
                        ..
                    } => continue,
                    Pieces {
                        piece_type: PieceTypes::Empty,
                        ..
                    } => continue,
                    Pieces {
                        piece_type: Rook, ..
                    } => moves.append(&mut find_moves(
                        self,
                        [i, j],
                        piece.get_color(),
                        ROOK_OFFSETS.to_vec(),
                        true,
                    )),
                    Pieces {
                        piece_type: Knight, ..
                    } => moves.append(&mut find_moves(
                        self,
                        [i, j],
                        piece.get_color(),
                        KNIGHT_OFFSETS.to_vec(),
                        false,
                    )),
                    Pieces {
                        piece_type: Bishop, ..
                    } => moves.append(&mut find_moves(
                        self,
                        [i, j],
                        piece.get_color(),
                        BISHOP_OFFSETS.to_vec(),
                        true,
                    )),
                    Pieces {
                        piece_type: Queen, ..
                    } => moves.append(&mut find_moves(
                        self,
                        [i, j],
                        piece.get_color(),
                        ROYAL_OFFSETS.to_vec(),
                        true,
                    )),
                    Pieces {
                        piece_type: King, ..
                    } => moves.append(&mut king_moves(self, [i, j], piece.get_color())),
                    Pieces {
                        piece_type: Pawn, ..
                    } => moves.append(&mut pawn_moves(self, [i, j], piece.get_color())),
                }
            }
        }
        moves
    }

    fn make_move(&self, mov: &GameMove2d) -> Array2D {
        //Expects mov to be a valid move
        let mut result = *self;
        let piece = self.board[mov.start[0]][mov.start[1]];
        let capture = result.board[mov.end[0]][mov.end[1]].piece_type != PieceTypes::Empty;

        //Move the piece
        result.board[mov.start[0]][mov.start[1]] = EMPTY_PIECE;
        result.board[mov.end[0]][mov.end[1]] = piece;

        //Change player
        result.curr_move = !result.curr_move;

        //Check if it was a castle and move the corresponding rook accordingly
        if let Some(castle_type) = mov.castle {
            match castle_type {
                CastleTypes::WhiteKing => {
                    let temp = result.board[0][7];
                    result.board[0][7] = EMPTY_PIECE;
                    result.board[0][5] = temp;
                }
                CastleTypes::WhiteQueen => {
                    let temp = result.board[0][0];
                    result.board[0][0] = EMPTY_PIECE;
                    result.board[0][3] = temp;
                }
                CastleTypes::BlackKing => {
                    let temp = result.board[7][7];
                    result.board[7][7] = EMPTY_PIECE;
                    result.board[7][5] = temp;
                }
                CastleTypes::BlackQueen => {
                    let temp = result.board[7][0];
                    result.board[7][0] = EMPTY_PIECE;
                    result.board[7][3] = temp;
                }
            }
        }

        //Check if it was a passant move and remove the pawn accordingly
        if let Some(passant_type) = mov.passant {
            match passant_type {
                PassantTypes::PassantAvailable(pos) => result.en_passant = Some(pos),
                PassantTypes::PassantCapture([row, col]) => result.board[row][col] = EMPTY_PIECE,
            }
        }

        //Check if it was a promotion and replace the piece accordingly
        if let Some(promotion) = mov.promote {
            result.board[mov.end[0]][mov.end[1]] = promotion;
        }

        //Update the kings position
        match piece {
            Pieces {
                piece_type: King,
                color: Black,
            } => result.black_king = mov.end,
            Pieces {
                piece_type: King,
                color: White,
            } => result.white_king = mov.end,
            _ => {}
        }

        //Update who is in check
        result.check = who_is_checked(&result);

        //Update castling rights
        if result.board[0][4].piece_type != King && result.board[0][4].color != White {
            result.castling_rights[0] = false;
            result.castling_rights[1] = false;
        }
        if result.board[7][4].piece_type != King && result.board[7][4].color != Black {
            result.castling_rights[2] = false;
            result.castling_rights[3] = false;
        }
        if result.board[0][0].piece_type != Rook && result.board[0][0].color != White {
            result.castling_rights[1] = false;
        }
        if result.board[0][7].piece_type != Rook && result.board[0][7].color != White {
            result.castling_rights[0] = false;
        }
        if result.board[7][0].piece_type != Rook && result.board[7][0].color != Black {
            result.castling_rights[3] = false;
        }
        if result.board[7][7].piece_type != Rook && result.board[7][7].color != Black {
            result.castling_rights[2] = false;
        }

        //Update half moves
        if capture || piece.piece_type == Pawn {
            result.half_moves = 0;
        } else {
            result.half_moves += 1;
        }
        //Update full moves
        if !result.curr_move {
            result.full_moves += 1
        }

        // Calculate Attack Maps
        [result.white_attack_map, result.black_attack_map] =
            Array2D::generate_attack_maps(result.board);

        result
    }

    fn get_check(&self) -> Option<bool> {
        self.check
    }

    fn get_curr_player(&self) -> bool {
        self.curr_move
    }

    fn get_board_as_2d(&self) -> [[Pieces; 8]; 8] {
        self.board
    }

    fn get_castle_rights(&self) -> [bool; 4] {
        self.castling_rights
    }
}

/* Supporting Functions */
fn find_moves(
    game: &Array2D,
    start: [usize; 2],
    player: bool,
    offsets: Vec<[i32; 2]>,
    ray: bool,
) -> Vec<GameMove2d> {
    let mut moves = Vec::new();
    if player != game.curr_move {
        return moves;
    }
    //Add moves for each direction the piece can move
    'dir: for direction in offsets {
        let mut row = start[0] as i32 + direction[0];
        let mut col = start[1] as i32 + direction[1];
        loop {
            if !is_in_bounds([row, col]) {
                continue 'dir;
            }
            let mov = GameMove2d {
                start,
                end: [row as usize, col as usize],
                ..Default::default()
            };
            if game.board[row as usize][col as usize].piece_type != PieceTypes::Empty {
                match game.board[row as usize][col as usize] {
                    Pieces { color: Black, .. } => {
                        if !player {
                            moves.push(GameMove2d {
                                start,
                                end: [row as usize, col as usize],
                                capture: true,
                                ..Default::default()
                            });
                        }
                    }
                    Pieces { color: White, .. } => {
                        if player {
                            moves.push(GameMove2d {
                                start,
                                end: [row as usize, col as usize],
                                capture: true,
                                ..Default::default()
                            });
                        }
                    }
                    _ => {}
                }
                continue 'dir;
            }
            moves.push(mov);
            if !ray {
                continue 'dir;
            }
            row += direction[0];
            col += direction[1];
        }
    }
    if player {
        moves
            .into_iter()
            .filter(|x| !is_black_checked(&game.make_move(x)))
            .collect()
    } else {
        moves
            .into_iter()
            .filter(|x| !is_white_checked(&game.make_move(x)))
            .collect()
    }
}

fn king_moves(game: &Array2D, start: [usize; 2], player: bool) -> Vec<GameMove2d> {
    let mut moves = Vec::new();
    if player != game.curr_move {
        return moves;
    }
    // Generate standard moves
    moves.append(&mut find_moves(
        game,
        start,
        player,
        ROYAL_OFFSETS.to_vec(),
        false,
    ));
    //If someone is in check, either the game is over or it's the current player and therefore can't castle
    if game.check.is_some() {
        return moves;
    }
    // Generate castle moves if no one is in check
    match player {
        //White Castles
        WHITE => {
            //King side Castle
            if game.castling_rights[0]
                && game.board[0][5].piece_type == PieceTypes::Empty
                && game.board[0][6].piece_type == PieceTypes::Empty
            {
                let mov1 = GameMove2d {
                    start,
                    end: [0, 5],
                    ..Default::default()
                };
                let mov2 = GameMove2d {
                    start,
                    end: [0, 6],
                    ..Default::default()
                };
                if !is_white_checked(&game.make_move(&mov1))
                    && !is_white_checked(&game.make_move(&mov2))
                {
                    moves.push(GameMove2d {
                        start,
                        end: [0, 6],
                        castle: Some(CastleTypes::WhiteKing),
                        ..Default::default()
                    });
                }
            }
            //Queen side Castle
            if game.castling_rights[1]
                && game.board[0][1].piece_type == PieceTypes::Empty
                && game.board[0][2].piece_type == PieceTypes::Empty
                && game.board[0][3].piece_type == PieceTypes::Empty
            {
                let mov1 = GameMove2d {
                    start,
                    end: [0, 3],
                    ..Default::default()
                };
                let mov2 = GameMove2d {
                    start,
                    end: [0, 2],
                    ..Default::default()
                };
                if !is_white_checked(&game.make_move(&mov1))
                    && !is_white_checked(&game.make_move(&mov2))
                {
                    moves.push(GameMove2d {
                        start,
                        end: [0, 2],
                        castle: Some(CastleTypes::WhiteQueen),
                        ..Default::default()
                    });
                }
            }
        }
        //Black Castles
        BLACK => {
            //King side Castle
            if game.castling_rights[2]
                && game.board[7][5].piece_type == PieceTypes::Empty
                && game.board[7][6].piece_type == PieceTypes::Empty
            {
                let mov1 = GameMove2d {
                    start,
                    end: [7, 5],
                    ..Default::default()
                };
                let mov2 = GameMove2d {
                    start,
                    end: [7, 6],
                    ..Default::default()
                };
                if !is_black_checked(&game.make_move(&mov1))
                    && !is_black_checked(&game.make_move(&mov2))
                {
                    moves.push(GameMove2d {
                        start,
                        end: [7, 6],
                        castle: Some(CastleTypes::BlackKing),
                        ..Default::default()
                    });
                }
            }
            //Queen side Castle
            if game.castling_rights[3]
                && game.board[7][1].piece_type == PieceTypes::Empty
                && game.board[7][2].piece_type == PieceTypes::Empty
                && game.board[7][3].piece_type == PieceTypes::Empty
            {
                let mov1 = GameMove2d {
                    start,
                    end: [7, 3],
                    ..Default::default()
                };
                let mov2 = GameMove2d {
                    start,
                    end: [7, 2],
                    ..Default::default()
                };
                if !is_black_checked(&game.make_move(&mov1))
                    && !is_black_checked(&game.make_move(&mov2))
                {
                    moves.push(GameMove2d {
                        start,
                        end: [7, 2],
                        castle: Some(CastleTypes::BlackQueen),
                        ..Default::default()
                    });
                }
            }
        }
    }
    moves
}

fn pawn_moves(game: &Array2D, start: [usize; 2], player: bool) -> Vec<GameMove2d> {
    let mut moves = Vec::new();
    if player != game.curr_move {
        return moves;
    }
    // Settings set up
    let forward = if player { -1 } else { 1 };
    let no_capture = if player { [7, 7] } else { [0, 0] };
    let check_function = if player {
        is_black_checked
    } else {
        is_white_checked
    };
    let color = if player { Black } else { White };

    //Pawn moves forward one - available if square ahead is empty and in bounds
    let row = start[0] as i32 + forward;
    let col = start[1] as i32;
    if is_in_bounds([row, col])
        && game.board[row as usize][col as usize].piece_type == PieceTypes::Empty
    {
        let mov = GameMove2d {
            start,
            end: [row as usize, col as usize],
            ..Default::default()
        };
        if !check_function(&game.make_move(&mov)) {
            if row == [7, 0][player as usize] {
                moves.push(GameMove2d {
                    start,
                    end: [row as usize, col as usize],
                    promote: Some(Pieces {
                        piece_type: Queen,
                        color,
                    }),
                    ..Default::default()
                });
                moves.push(GameMove2d {
                    start,
                    end: [row as usize, col as usize],
                    promote: Some(Pieces {
                        piece_type: Rook,
                        color,
                    }),
                    ..Default::default()
                });
                moves.push(GameMove2d {
                    start,
                    end: [row as usize, col as usize],
                    promote: Some(Pieces {
                        piece_type: Bishop,
                        color,
                    }),
                    ..Default::default()
                });
                moves.push(GameMove2d {
                    start,
                    end: [row as usize, col as usize],
                    promote: Some(Pieces {
                        piece_type: Knight,
                        color,
                    }),
                    ..Default::default()
                });
            } else {
                moves.push(mov);
            }
        }
    }
    //Pawn moves forward two - available only if the pawn is on it's starting square, then the two squares in front must be empty. Cannot promote with this move
    if start[0] == [1, 6][player as usize] {
        let row = start[0] as i32 + forward * 2;
        let col = start[1] as i32;
        if game.board[(row - forward) as usize][col as usize].piece_type == PieceTypes::Empty
            && game.board[row as usize][col as usize].piece_type == PieceTypes::Empty
        {
            let passant: Option<PassantTypes> = Some(PassantTypes::PassantAvailable([
                (row - forward) as usize,
                col as usize,
            ]));
            let mov = GameMove2d {
                start,
                end: [row as usize, col as usize],
                passant,
                ..Default::default()
            };
            if !check_function(&game.make_move(&mov)) {
                moves.push(mov);
            }
        }
    }
    //Pawn captures to the left - available only if there is an opposing piece on that square
    let row = start[0] as i32 + forward;
    let col = start[1] as i32 - 1;
    //If a passant capture is available, get the space coordinates - otherwise the square that can't be captured by current player's pawns
    let passant_pos = if let Some(passant_pos) = game.en_passant {
        passant_pos
    } else {
        no_capture
    };
    let passant_cap = [row as usize, col as usize] == passant_pos;
    if is_in_bounds([row, col])
        && (game.board[row as usize][col as usize].piece_type != PieceTypes::Empty || passant_cap)
    {
        let mov = GameMove2d {
            start,
            end: [row as usize, col as usize],
            ..Default::default()
        };
        if passant_cap && !check_function(&game.make_move(&mov)) {
            //Passant captures cannot promote
            moves.push(GameMove2d {
                start,
                end: passant_pos,
                passant: Some(PassantTypes::PassantCapture([
                    (row + -forward) as usize,
                    col as usize,
                ])),
                ..Default::default()
            });
        } else {
            match game.board[row as usize][col as usize] {
                Pieces { color: player, .. } => {
                    if player != color && !check_function(&game.make_move(&mov)) {
                        if row == [7, 0][player as usize] {
                            moves.push(GameMove2d {
                                start,
                                end: [row as usize, col as usize],
                                promote: Some(Pieces {
                                    piece_type: Queen,
                                    color,
                                }),
                                ..Default::default()
                            });
                            moves.push(GameMove2d {
                                start,
                                end: [row as usize, col as usize],
                                promote: Some(Pieces {
                                    piece_type: Rook,
                                    color,
                                }),
                                ..Default::default()
                            });
                            moves.push(GameMove2d {
                                start,
                                end: [row as usize, col as usize],
                                promote: Some(Pieces {
                                    piece_type: Bishop,
                                    color,
                                }),
                                ..Default::default()
                            });
                            moves.push(GameMove2d {
                                start,
                                end: [row as usize, col as usize],
                                promote: Some(Pieces {
                                    piece_type: Knight,
                                    color,
                                }),
                                ..Default::default()
                            });
                        } else {
                            moves.push(mov);
                        }
                    }
                }
            };
        }
    }
    //Pawn captures to the right - available only if there is a piece on that square
    let row = start[0] as i32 + forward;
    let col = start[1] as i32 + 1;
    //If a passant capture is available, get the space coordinates - otherwise the square that can't be captured by current player's pawns
    let passant_pos = if let Some(passant_pos) = game.en_passant {
        passant_pos
    } else {
        no_capture
    };
    let passant_cap = [row as usize, col as usize] == passant_pos;
    if is_in_bounds([row, col])
        && (game.board[row as usize][col as usize].piece_type != PieceTypes::Empty || passant_cap)
    {
        let mov = GameMove2d {
            start,
            end: [row as usize, col as usize],
            ..Default::default()
        };
        if passant_cap && !check_function(&game.make_move(&mov)) {
            //Passant captures cannot promote
            moves.push(GameMove2d {
                start,
                end: passant_pos,
                passant: Some(PassantTypes::PassantCapture([
                    (row + -forward) as usize,
                    col as usize,
                ])),
                ..Default::default()
            });
        } else {
            match game.board[row as usize][col as usize] {
                Pieces { color: player, .. } => {
                    if player != color && !check_function(&game.make_move(&mov)) {
                        if row == [7, 0][player as usize] {
                            moves.push(GameMove2d {
                                start,
                                end: [row as usize, col as usize],
                                promote: Some(Pieces {
                                    piece_type: Queen,
                                    color,
                                }),
                                ..Default::default()
                            });
                            moves.push(GameMove2d {
                                start,
                                end: [row as usize, col as usize],
                                promote: Some(Pieces {
                                    piece_type: Rook,
                                    color,
                                }),
                                ..Default::default()
                            });
                            moves.push(GameMove2d {
                                start,
                                end: [row as usize, col as usize],
                                promote: Some(Pieces {
                                    piece_type: Bishop,
                                    color,
                                }),
                                ..Default::default()
                            });
                            moves.push(GameMove2d {
                                start,
                                end: [row as usize, col as usize],
                                promote: Some(Pieces {
                                    piece_type: Knight,
                                    color,
                                }),
                                ..Default::default()
                            });
                        } else {
                            moves.push(mov);
                        }
                    }
                }
            };
        }
    }
    moves
}

fn is_in_bounds(pos: [i32; 2]) -> bool {
    if pos[0] > 7 || pos[0] < 0 || pos[1] > 7 || pos[1] < 0 {
        return false;
    }
    true
}

fn is_white_checked(game: &Array2D) -> bool {
    game.black_attack_map[game.white_king[0]][game.white_king[1]] > 0
}

fn is_black_checked(game: &Array2D) -> bool {
    game.white_attack_map[game.black_king[0]][game.black_king[1]] > 0
}

fn who_is_checked(game: &Array2D) -> Option<bool> {
    //White King
    if is_white_checked(game) {
        return Some(WHITE);
    }
    //Black King
    if is_black_checked(game) {
        return Some(BLACK);
    }
    None
}

#[test]
fn digit_manipulation() {
    for (character, number) in [
        ('a', 0),
        ('b', 1),
        ('c', 2),
        ('d', 3),
        ('e', 4),
        ('f', 5),
        ('g', 6),
        ('h', 7),
    ] {
        assert_eq!(to_num(character), number);
        assert_eq!(to_let(number), character);
    }
}

#[test]
fn test_move_generation() {
    let game = Array2D::setup_board(None);
    assert_eq!(game.get_valid_moves().len(), 20)
}

#[test]
fn test_attack_map_generation() {
    const STARTING_ATTACKS_WHITE: [[u8; 8]; 8] = [
        [0, 1, 1, 1, 1, 1, 1, 0],
        [1, 1, 1, 4, 4, 1, 1, 1],
        [2, 2, 3, 2, 2, 3, 2, 2],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    const STARTING_ATTACKS_BLACK: [[u8; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [2, 2, 3, 2, 2, 3, 2, 2],
        [1, 1, 1, 4, 4, 1, 1, 1],
        [0, 1, 1, 1, 1, 1, 1, 0],
    ];
    let game = Array2D::setup_board(None);
    assert_eq!(STARTING_ATTACKS_WHITE, game.white_attack_map);
    assert_eq!(STARTING_ATTACKS_BLACK, game.black_attack_map);
}
