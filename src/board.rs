#![allow(dead_code)]
use std::{fmt, collections::HashMap};

const WHITE: bool = false;
const BLACK: bool = true;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Pieces {
  Knight (bool),
  Rook (bool),
  Bishop (bool),
  Queen (bool),
  King (bool),
  Pawn (bool),
  Empty,
}
use Pieces::*;

impl Pieces {
  pub fn from (piece: &char) -> Pieces {
    match piece {
      'r' => Rook(BLACK),
      'n' => Knight(BLACK),
      'b' => Bishop(BLACK),
      'q' => Queen(BLACK),
      'k' => King(BLACK),
      'p' => Pawn(BLACK),
      'P' => Pawn(WHITE),
      'K' => King(WHITE),
      'Q' => Queen(WHITE),
      'B' => Bishop(WHITE),
      'N' => Knight(WHITE),
      'R' => Rook(WHITE),
      _ => Empty
    }
  }
}

impl fmt::Display for Pieces {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let piece = match self {
      Rook(_) => "r",
      Knight(_) => "n",
      Bishop(_) => "b",
      Queen(_) => "q",
      King(_) => "k",
      Pawn(_) => "p",
      _ => "",
    };
    write!(f, "{}", piece)
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PassantTypes {
  PassantCapture ([usize; 2]),
  PassantAvailable ([usize; 2]),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CastleTypes {
  WhiteKing,
  WhiteQueen,
  BlackKing,
  BlackQueen,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Move {
  pub start: [usize; 2],
  pub end: [usize; 2],
  pub castle: Option<CastleTypes>,
  pub promote: Option<Pieces>,
  pub passant: Option<PassantTypes>,
  pub capture: bool,
}

impl fmt::Display for Move {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let promotion;
      if let Some(piece) = self.promote {
        promotion = piece.to_string();
      }
      else {
        promotion = "".to_owned();
      }
      write!(f, "{}{}{}{}{}", to_let(self.start[1]), self.start[0] + 1, to_let(self.end[1]), self.end[0] + 1, promotion)
  }
}

impl Ord for Move {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    let mut mov1_score = 0;
    let mut mov2_score = 0;
    //Find self's score
    //if let(check) = self.check {
    //  mov1_score += 1;
    //}
    if self.castle.is_some() {
      mov1_score += 2;
    }
    if self.capture {
      mov1_score += 3;
    }
    if self.promote.is_some() {
      mov1_score += 2;
    }
    if let Some(passant) = self.passant {
      match passant {
        PassantTypes::PassantAvailable(_) => {},
        PassantTypes::PassantCapture(_) => mov1_score += 15,
      }
    }
    //Find other's score
    if other.castle.is_some() {
      mov2_score += 2;
    }
    if other.capture {
      mov2_score += 3;
    }
    if other.promote.is_some() {
      mov2_score += 2;
    }
    if let Some(passant) = other.passant {
      match passant {
        PassantTypes::PassantAvailable(_) => {},
        PassantTypes::PassantCapture(_) => mov2_score += 15,
      }
    }
    mov1_score.cmp(&mov2_score)
  }
}

impl PartialOrd for Move {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

/* Game state representation and member functions */
#[derive(Clone, Debug)]
pub struct GameState {
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
  pub available_moves: Vec<Move>,
}

impl GameState {
    pub fn from_fen (fen: &str) -> GameState {
      let mut fields = fen.split_ascii_whitespace();
      //Read board positions
      let temp = fields.next().unwrap();
      let board = temp.split('/');
      let mut board_state: [[Pieces; 8]; 8] = [[Empty; 8]; 8];
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
        _ => panic!("Invalid FEN String")
      };

      //Read castling rights
      let temp = fields.next().unwrap();
      let rights: [bool; 4] = [temp.contains('K'), temp.contains('Q'), temp.contains('k'), temp.contains('q')];

      //Read En Passant targets
      let temp = fields.next().unwrap();
      let passant: Option<[usize; 2]> = if !temp.eq("-") {
        let mut chars = temp.chars();
        let col = to_num(chars.next().unwrap());
        let row = (chars.next().unwrap().to_digit(10).unwrap() - 1) as usize;
        Some([row, col])
      }
      else {
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
            King(BLACK) => black_king = [i, j],
            King(WHITE) => white_king = [i, j],
            _ => continue,
          }
        }
      }

      let mut game = GameState {board: board_state, curr_move: player, castling_rights: rights, en_passant: passant, half_moves, full_moves, check: None, white_king, black_king, available_moves: vec![]};

      //Calculate if a king is in check
      game.check = is_in_check(&game);

      //Calculate valid moves
      game.available_moves = game.valid_moves();
      game
    }

    pub fn valid_moves (&self) -> Vec<Move> {
      let mut moves = Vec::new();

      for i in 0..8 {
        for j in 0..8 {
          match self.board[i][j] {
            Empty => continue,
            Rook(player) => moves.append(&mut rook_moves(self, [i, j], player)),
            Knight(player) => moves.append(&mut knight_moves(self, [i, j], player)),
            Bishop(player) => moves.append(&mut bishop_moves(self, [i, j], player)),
            Queen(player) => moves.append(&mut queen_moves(self, [i, j], player)),
            King(player) => moves.append(&mut king_moves(self, [i, j], player)),
            Pawn(player) => moves.append(&mut pawn_moves(self, [i, j], player)),
          }
        }
      }
      moves
    }

    pub fn test_move (&self, mov: Move) -> GameState {
      //Expects mov to be a valid move
      let mut result = self.clone();
      let piece = self.board[mov.start[0]][mov.start[1]];
      let capture = result.board[mov.end[0]][mov.end[1]] != Empty;
      //Move the piece
      result.board[mov.end[0]][mov.end[1]] = piece;
      result.board[mov.start[0]][mov.start[1]] = Empty;
      //Change player
      result.curr_move = !result.curr_move;
      //Check if it was a castle and move the corresponding rook accordingly
      if let Some(castle_type) = mov.castle {
        match castle_type {
          CastleTypes::WhiteKing => result.make_move(Move {start: [0, 7], end: [0, 5], ..Default::default()}),
          CastleTypes::WhiteQueen => result.make_move(Move {start: [0, 0], end: [0, 3], ..Default::default()}),
          CastleTypes::BlackKing => result.make_move(Move {start: [7, 7], end: [7, 5], ..Default::default()}),
          CastleTypes::BlackQueen => result.make_move(Move {start: [7, 0], end: [7, 3], ..Default::default()}),
        };
      }
      //Check if it was a passant move and remove the pawn accordingly
      if let Some(passant_type) = mov.passant {
        match passant_type {
          PassantTypes::PassantAvailable(pos) => result.en_passant = Some(pos),
          PassantTypes::PassantCapture([row, col]) => result.board[row][col] = Empty,
        }
      }
      //Check if it was a promotion and replace the piece accordingly
      if let Some(promotion) = mov.promote {
        result.board[mov.end[0]][mov.end[1]] = promotion;
      }
      //Update the kings position
      match piece {
        King(BLACK) => result.black_king = mov.end,
        King(WHITE) => result.white_king = mov.end,
        _ => {},
      }
      //Update who is in check
      result.check = is_in_check(&result);
      //Update castling rights
      match piece {
        King(BLACK) => {
          result.castling_rights[2] = false;
          result.castling_rights[3] = false; 
        },
        King(WHITE) => {
          result.castling_rights[0] = false;
          result.castling_rights[1] = false; 
        },
        Rook(BLACK) => {
          if mov.start == [7, 7] {
            result.castling_rights[2] = false;
          }
          else if mov.start == [7, 0] {
            result.castling_rights[3] = false;
          }
        },
        Rook(WHITE) => {
          if mov.start == [0, 7] {
            result.castling_rights[0] = false;
          }
          else if mov.start == [0, 0] {
            result.castling_rights[1] = false;
          }
        },
        _ => {},
      }
      //Update half moves
      if capture || Pawn(WHITE) == piece || Pawn(BLACK) == piece {
        result.half_moves = 0;
      }
      else {
        result.half_moves += 1;
      }
      //Update full moves
      if !result.curr_move {
        result.full_moves += 1
      }
      //Reset available moves
      result.available_moves = vec![];
      result
    }

    pub fn make_move (&self, mov: Move) -> GameState {
      let mut result = self.test_move(mov);
      //Update valid moves
      result.available_moves = result.valid_moves();
      result
    }
}

/* Supporting Functions */
fn rook_moves (game: &GameState, start: [usize; 2], player: bool) -> Vec<Move> {
  let mut moves = Vec::new();
  if player != game.curr_move {
    return moves;
  }
  //Add moves for each direction the rook can move
  'dir: for direction in [[0, 1], [0, -1], [1, 0], [-1, 0]] {
    let mut row = start[0] as i32 + direction[0];
    let mut col = start[1] as i32 + direction[1];
    if !is_in_bounds([row, col]) {
        continue 'dir;
    }
    //Add moves until the direction hits a piece or goes off the board
    while game.board[row as usize][col as usize] == Empty {
      let mov = Move {start, end: [row as usize, col as usize], ..Default::default()};
      if !player && !is_white_checked(&game.test_move(mov)){
        moves.push(mov);
      }
      if player && !is_black_checked(&game.test_move(mov)){
        moves.push(mov);
      }
      row += direction[0];
      col += direction[1];
      if !is_in_bounds([row, col]) {
        continue 'dir;
      }
    }
    //Find which piece we've hit and add moves acccordingly
    let mov = Move {start, end: [row as usize, col as usize], ..Default::default()};
    if !player && is_white_checked(&game.test_move(mov)){
      continue 'dir;
    }
    if player && is_black_checked(&game.test_move(mov)){
      continue 'dir;
    }
    match game.board[row as usize][col as usize] {
      //Black piece
      Rook(BLACK) | Queen(BLACK) | Bishop(BLACK) | King(BLACK) | Knight(BLACK) | Pawn(BLACK) => {
        //White's turn
        if !player {
          moves.push(Move {start, end: [row as usize, col as usize], capture: true, ..Default::default()});
        }
      },
      //White's piece
      Rook(WHITE) | Queen(WHITE) | Bishop(WHITE) | King(WHITE) | Knight(WHITE) | Pawn(WHITE) => {
        //Black's turn
        if player {
          moves.push(Move {start, end: [row as usize, col as usize], capture: true, ..Default::default()});
        }
      },
      _ => {},
    }
  }

  moves
}

fn knight_moves (game: &GameState, start: [usize; 2], player: bool) -> Vec<Move> {
  let mut moves = Vec::new();
  if player != game.curr_move {
    return moves;
  }
  //Add moves for each move a knight can make
  for direction in [[2, 1], [2, -1], [-2, 1], [-2, -1], [1, 2], [1, -2], [-1, 2], [-1, -2]] {
    let row = start[0] as i32 + direction[0];
    let col = start[1] as i32 + direction[1];
    //Check if the space is in bounds
    if is_in_bounds([row, col]) {
      //Check for a piece at the position and react accordingly
      match game.board[row as usize][col as usize] {
        //Black piece
        Rook(BLACK) | Queen(BLACK) | Bishop(BLACK) | King(BLACK) | Knight(BLACK) | Pawn(BLACK) => {
          //White's turn
          if !player {
            let mov = Move {start, end: [row as usize, col as usize], ..Default::default()};
            if !is_white_checked(&game.test_move(mov)){
              moves.push(mov);
            }
          }
        },
        //White piece
        Rook(WHITE) | Queen(WHITE) | Bishop(WHITE) | King(WHITE) | Knight(WHITE) | Pawn(WHITE) => {
          //Black's turn
          if player {
            let mov = Move {start, end: [row as usize, col as usize], capture: true, ..Default::default()};
            if !is_black_checked(&game.test_move(mov)){
              moves.push(mov);
            }
          }
        },
        //If it's an empty space, either player can move there
        Empty => {
          let mov = Move {start, end: [row as usize, col as usize], capture: true, ..Default::default()};
          if !player && !is_white_checked(&game.test_move(mov)){
            moves.push(mov);
          }
          if player && !is_black_checked(&game.test_move(mov)){
            moves.push(mov);
          }
        },
      }
    }
  }
  moves
}

fn bishop_moves (game: &GameState, start: [usize; 2], player: bool) -> Vec<Move> {
  let mut moves = Vec::new();
  if player != game.curr_move {
    return moves;
  }
  //Generate moves in each direction a bishop can move
  'dir: for direction in [[1, 1], [1, -1], [-1, 1], [-1, -1]] {
    let mut row = start[0] as i32 + direction[0];
    let mut col = start[1] as i32 + direction[1];
    if !is_in_bounds([row, col]) {
        continue 'dir;
    }
    //Add moves until the direction hits a piece or goes off the board
    while game.board[row as usize][col as usize] == Empty {
      let mov = Move {start, end: [row as usize, col as usize], ..Default::default()};
      if !player && !is_white_checked(&game.test_move(mov)){
        moves.push(mov);
      }
      if player && !is_black_checked(&game.test_move(mov)){
        moves.push(mov);
      }
      row += direction[0];
      col += direction[1];
      if !is_in_bounds([row, col]) {
        continue 'dir;
      }
    }
    //Find which piece we hit and add moves accordingly
    let mov = Move {start, end: [row as usize, col as usize], capture: true, ..Default::default()};
    if !player && is_white_checked(&game.test_move(mov)){
      continue 'dir;
    }
    if player && is_black_checked(&game.test_move(mov)){
      continue 'dir;
    }
    match game.board[row as usize][col as usize] {
      //Black piece
      Rook(BLACK) | Queen(BLACK) | Bishop(BLACK) | King(BLACK) | Knight(BLACK) | Pawn(BLACK) => {
        //White's turn
        if !player {
          moves.push(mov);
        }
      },
      //White piece
      Rook(WHITE) | Queen(WHITE) | Bishop(WHITE) | King(WHITE) | Knight(WHITE) | Pawn(WHITE) => {
        //Black's turn
        if player {
          moves.push(mov);
        }
      },
      _ => {},
    }
  }
  moves
}

fn queen_moves (game: &GameState, start: [usize; 2], player: bool) -> Vec<Move> {
  let mut moves = Vec::new();
  if player != game.curr_move {
    return moves;
  }
  moves.append(&mut rook_moves(game, start, player));
  moves.append(&mut bishop_moves(game, start, player));
  moves
}

fn king_moves (game: & GameState, start: [usize; 2], player: bool) -> Vec<Move> {
  let mut moves = Vec::new();
  if player != game.curr_move {
    return moves;
  }
  //Generate moves in every direction the king can move
  for direction in [[0, 1], [0, -1], [1, 0], [-1, 0], [1, 1], [1, -1], [-1, 1], [-1, -1]] {
    let row = start[0] as i32 + direction[0];
    let col = start[1] as i32 + direction[1];
    if !is_in_bounds([row, col]) {
        continue;
    }
    //Check which piece type is at the desired move position
    match game.board[row as usize][col as usize] {
      //If it's a black piece
      Rook(BLACK) | Queen(BLACK) | Bishop(BLACK) | King(BLACK) | Knight(BLACK) | Pawn(BLACK) => {
        //If it's white's turn
        if !player {
          let mov = Move {start, end: [row as usize, col as usize], ..Default::default()};
          if !is_white_checked(&game.test_move(mov)){
            moves.push(mov);
          }
        }
      },
      //If it's a white piece
      Rook(WHITE) | Queen(WHITE) | Bishop(WHITE) | King(WHITE) | Knight(WHITE) | Pawn(WHITE) => {
        //If it's black's turn
        if player {
          let mov = Move {start, end: [row as usize, col as usize], ..Default::default()};
          if !is_black_checked(&game.test_move(mov)){
            moves.push(mov);
          }
        }
      },
      //If it's an empty space, either player can move there
      Empty => {
        let mov = Move {start, end: [row as usize, col as usize], ..Default::default()};
        if !player && !is_white_checked(&game.test_move(mov)){
          moves.push(mov);
        }
        if player && !is_black_checked(&game.test_move(mov)){
          moves.push(mov);
        }
      },
    }
  }
  //If someone is in check, either the game is over or it's the current player and therefore can't castle
  if game.check.is_some() {
    return moves;
  }
  match player {
    //White Castles
    WHITE => {
      //King side Castle
      if game.castling_rights[0] && game.board[0][5] == Empty && game.board[0][6] == Empty {
        let mov1 = Move {start, end: [0, 5], ..Default::default()};
        let mov2 = Move {start, end: [0, 6], ..Default::default()};
        if !is_white_checked(&game.test_move(mov1)) && !is_white_checked(&game.test_move(mov2)) {
          moves.push(Move {start, end: [0, 6], castle: Some(CastleTypes::WhiteKing), ..Default::default()});
        }
      }
      //Queen side Castle
      if game.castling_rights[1] && game.board[0][1] == Empty && game.board[0][2] == Empty && game.board[0][3] == Empty {
        let mov1 = Move {start, end: [0, 3], ..Default::default()};
        let mov2 = Move {start, end: [0, 2], ..Default::default()};
        if !is_white_checked(&game.test_move(mov1)) && !is_white_checked(&game.test_move(mov2)) {
          moves.push(Move {start, end: [0, 2], castle: Some(CastleTypes::WhiteQueen), ..Default::default()});
        }
      }
    },
    //Black Castles
    BLACK => {
      //King side Castle
      if game.castling_rights[2] && game.board[7][5] == Empty && game.board[7][6] == Empty {
        let mov1 = Move {start, end: [7, 5], ..Default::default()};
        let mov2 = Move {start, end: [7, 6], ..Default::default()};
        if !is_black_checked(&game.test_move(mov1)) && !is_black_checked(&game.test_move(mov2)) {
          moves.push(Move {start, end: [7, 6], castle: Some(CastleTypes::BlackKing), ..Default::default()});
        }
      }
      //Queen side Castle
      if game.castling_rights[3] && game.board[7][1] == Empty && game.board[7][2] == Empty && game.board[7][3] == Empty {
        let mov1 = Move {start, end: [7, 3], ..Default::default()};
        let mov2 = Move {start, end: [7, 2], ..Default::default()};
        if !is_black_checked(&game.test_move(mov1)) && !is_black_checked(&game.test_move(mov2)) {
          moves.push(Move {start, end: [7, 2], castle: Some(CastleTypes::BlackQueen), ..Default::default()});
        }
      }
    },
  }
  moves
}

fn pawn_moves (game: &GameState, start: [usize; 2], player: bool) -> Vec<Move> {
  let mut moves = Vec::new();
  let forward = if player { -1 } else { 1 };
  let player_int = if player { 1 } else { 0 };
  let no_capture = if player { [7, 7] } else { [0, 0] };
  let check_function = if player { is_black_checked } else { is_white_checked };
  if player != game.curr_move {
    return moves;
  }
  //Pawn moves forward one - available if square ahead is empty and in bounds
  let row = start[0] as i32 + forward;
  let col = start[1] as i32;
  if is_in_bounds([row, col]) && game.board[row as usize][col as usize] == Empty {
    let mov = Move {start, end: [row as usize, col as usize], ..Default::default()};
    if !check_function(&game.test_move(mov)){
      if row == [7, 0][player_int] {
        moves.push(Move { start, end: [row as usize, col as usize], promote: Some(Queen(game.curr_move)), ..Default::default() });
        moves.push(Move { start, end: [row as usize, col as usize], promote: Some(Rook(game.curr_move)), ..Default::default() });
        moves.push(Move { start, end: [row as usize, col as usize], promote: Some(Knight(game.curr_move)), ..Default::default() });
        moves.push(Move { start, end: [row as usize, col as usize], promote: Some(Bishop(game.curr_move)), ..Default::default() });
      }
      else {
        moves.push(mov);
      }
    }
  }
  //Pawn moves forward two - available only if the pawn is on it's starting square, then the two squares in front must be empty. Cannot promote with this move
  if start[0] == [1, 6][player_int] {
    let row = start[0] as i32 + forward * 2;
    let col = start[1] as i32;
    if game.board[(row - forward) as usize][col as usize] == Empty && game.board[row as usize][col as usize] == Empty {
      let passant: Option<PassantTypes> = Some(PassantTypes::PassantAvailable([(row - forward) as usize, col as usize]));
      let mov = Move {start, end: [row as usize, col as usize], passant, ..Default::default()};
      if !check_function(&game.test_move(mov)) {
        moves.push(mov);
      } 
    }
  }
  //Pawn captures to the left - available only if there is a piece on that square
  let row = start[0] as i32 + forward;
  let col = start[1] as i32 - 1;
  //If a passant capture is available, get the space coordinates - otherwise the square that can't be captured by current player's pawns
  let passant_pos = if let Some(passant_pos) = game.en_passant { passant_pos } else { no_capture };
  let passant_cap = [row as usize, col as usize] == passant_pos;
  if is_in_bounds([row, col]) && (game.board[row as usize][col as usize] != Empty || passant_cap) {
    let mov = Move {start, end: [row as usize, col as usize], ..Default::default()};
    if passant_cap && !check_function(&game.test_move(mov)) {
      //Passant captures cannot promote
      moves.push(Move { start, end: passant_pos, passant: Some(PassantTypes::PassantCapture([(row + -forward) as usize, col as usize])), ..Default::default()});
    }
    else {
      match game.board[row as usize][col as usize] {
        Rook(player) | Queen(player) | Bishop(player) | King(player) | Knight(player) | Pawn(player) => {
          if player != game.curr_move && !check_function(&game.test_move(mov)){
            if row == [7, 0][player_int] {
              moves.push(Move { start, end: [row as usize, col as usize], promote: Some(Queen(game.curr_move)), ..Default::default() });
              moves.push(Move { start, end: [row as usize, col as usize], promote: Some(Rook(game.curr_move)), ..Default::default() });
              moves.push(Move { start, end: [row as usize, col as usize], promote: Some(Knight(game.curr_move)), ..Default::default() });
              moves.push(Move { start, end: [row as usize, col as usize], promote: Some(Bishop(game.curr_move)), ..Default::default() });
            }
            else {
              moves.push(mov);
            }
          }
        },
        _ => {},
      };
    }
  }
  //Pawn captures to the right - available only if there is a piece on that square
  let row = start[0] as i32 + forward;
  let col = start[1] as i32 + 1;
  //If a passant capture is available, get the space coordinates - otherwise the square that can't be captured by current player's pawns
  let passant_pos = if let Some(passant_pos) = game.en_passant { passant_pos } else { no_capture };
  let passant_cap = [row as usize, col as usize] == passant_pos;
  if is_in_bounds([row, col]) && (game.board[row as usize][col as usize] != Empty || passant_cap) {
    let mov = Move {start, end: [row as usize, col as usize], ..Default::default()};
    if passant_cap && !check_function(&game.test_move(mov)) {
      //Passant captures cannot promote
      moves.push(Move { start, end: passant_pos, passant: Some(PassantTypes::PassantCapture([(row + -forward) as usize, col as usize])), ..Default::default()});
    }
    else {
      match game.board[row as usize][col as usize] {
        Rook(player) | Queen(player) | Bishop(player) | King(player) | Knight(player) | Pawn(player) => {
          if player != game.curr_move && !check_function(&game.test_move(mov)){
            if row == [7, 0][player_int] {
              moves.push(Move { start, end: [row as usize, col as usize], promote: Some(Queen(game.curr_move)), ..Default::default() });
              moves.push(Move { start, end: [row as usize, col as usize], promote: Some(Rook(game.curr_move)), ..Default::default() });
              moves.push(Move { start, end: [row as usize, col as usize], promote: Some(Knight(game.curr_move)), ..Default::default() });
              moves.push(Move { start, end: [row as usize, col as usize], promote: Some(Bishop(game.curr_move)), ..Default::default() });
            }
            else {
              moves.push(mov);
            }
          }
        },
        _ => {},
      };
    } 
  }
  moves
}

fn is_in_bounds (pos: [i32; 2]) -> bool {
  if pos[0] > 7 || pos[0] < 0 || pos[1] > 7 || pos[1] < 0 {
    return false;
  }
  true
}

fn is_white_checked (game: &GameState) -> bool {
  //Orthogonal attacks
  'dir: for direction in [[0, 1], [0, -1], [1, 0], [-1, 0]] {
    let mut row = game.white_king[0] as i32 + direction[0];
    let mut col = game.white_king[1] as i32 + direction[1];
    if !is_in_bounds([row, col]) {
        continue 'dir;
    } 
    while game.board[row as usize][col as usize] == Empty {
      row += direction[0];
      col += direction[1];
      if !is_in_bounds([row, col]) {
        continue 'dir;
      } 
    }
    match game.board[row as usize][col as usize] {
      Rook(BLACK) | Queen(BLACK) => return true,
      _ => {},
    }
  }
  //Diagonal attacks
  'dir: for direction in [[1, 1], [1, -1], [-1, 1], [-1, -1]] {
    let mut row = game.white_king[0] as i32 + direction[0];
    let mut col = game.white_king[1] as i32 + direction[1];
    if !is_in_bounds([row, col]) {
        continue 'dir;
    } 
    while game.board[row as usize][col as usize] == Empty {
      row += direction[0];
      col += direction[1];
      if !is_in_bounds([row, col]) {
        continue 'dir;
      } 
    }
    match game.board[row as usize][col as usize] {
      Bishop(BLACK) | Queen(BLACK) => return true,
      Pawn(BLACK) => if row == game.white_king[0] as i32 + 1 { return true },
      _ => {},
    }
  }
  //Knight attacks
  for direction in [[2, 1], [2, -1], [-2, 1], [-2, -1], [1, 2], [1, -2], [-1, 2], [-1, -2]] {
    let row = game.white_king[0] as i32 + direction[0];
    let col = game.white_king[1] as i32 + direction[1];
    if is_in_bounds([row, col]) && game.board[row as usize][col as usize] == Knight(BLACK) {
      return true;
    }
  }
  //King attacks
  for direction in [[1, 1], [1, -1], [-1, 1], [-1, -1], [0, 1], [0, -1], [1, 0], [-1, 0]] {
    let row = game.white_king[0] as i32 + direction[0];
    let col = game.white_king[1] as i32 + direction[1];
    if is_in_bounds([row, col]) && game.board[row as usize][col as usize] == King(BLACK) {
      return true;
    }
  }
  false
}

fn is_black_checked (game: &GameState) -> bool {
  //Orthogonal attacks
  'dir: for direction in [[0, 1], [0, -1], [1, 0], [-1, 0]] {
    let mut row = game.black_king[0] as i32 + direction[0];
    let mut col = game.black_king[1] as i32 + direction[1];
    if !is_in_bounds([row, col]) {
        continue 'dir;
    } 
    while game.board[row as usize][col as usize] == Empty {
      row += direction[0];
      col += direction[1];
      if !is_in_bounds([row, col]) {
        continue 'dir;
      } 
    }
    match game.board[row as usize][col as usize] {
      Rook(WHITE) | Queen(WHITE) => return true,
      _ => {},
    }
  }
  //Diagonal attacks
  'dir: for direction in [[1, 1], [1, -1], [-1, 1], [-1, -1]] {
    let mut row = game.black_king[0] as i32 + direction[0];
    let mut col = game.black_king[1] as i32 + direction[1];
    if !is_in_bounds([row, col]) {
        continue 'dir;
    } 
    while game.board[row as usize][col as usize] == Empty {
      row += direction[0];
      col += direction[1];
      if !is_in_bounds([row, col]) {
        continue 'dir;
      } 
    }
    match game.board[row as usize][col as usize] {
      Bishop(WHITE) | Queen(WHITE) => return true,
      Pawn(WHITE) => if row == game.black_king[0] as i32 - 1 { return true },
      _ => {},
    }
  }
  //Knight attacks
  for direction in [[2, 1], [2, -1], [-2, 1], [-2, -1], [1, 2], [1, -2], [-1, 2], [-1, -2]] {
    let row = game.black_king[0] as i32 + direction[0];
    let col = game.black_king[1] as i32 + direction[1];
    if is_in_bounds([row, col]) && game.board[row as usize][col as usize] == Knight(WHITE) {
      return true;
    }
  }
  //King attacks
  for direction in [[1, 1], [1, -1], [-1, 1], [-1, -1], [0, 1], [0, -1], [1, 0], [-1, 0]] {
    let row = game.black_king[0] as i32 + direction[0];
    let col = game.black_king[1] as i32 + direction[1];
    if is_in_bounds([row, col]) && game.board[row as usize][col as usize] == King(WHITE) {
      return true;
    }
  }
  false
}

fn is_in_check (game: &GameState) -> Option<bool> {
  //White King
  if is_white_checked(game) {
    return Some(false);
  }
  //Black King
  if is_black_checked(game) {
    return Some(true);
  }
  None
}

fn to_let (num: usize) -> char {
  match num {
    0 => 'a',
    1 => 'b',
    2 => 'c',
    3 => 'd',
    4 => 'e',
    5 => 'f',
    6 => 'g',
    7 => 'h',
    _ => panic!(),
  }
}

fn to_num (letter: char) -> usize {
  match letter {
    'a' => 0,
    'b' => 1,
    'c' => 2,
    'd' => 3,
    'e' => 4,
    'f' => 5,
    'g' => 6,
    'h' => 7,
    _ => panic!(),
  }
}

pub fn term (game: &GameState) -> Option<i32> {
  if game.available_moves.is_empty() {
    if game.check.is_some() {
      Some([i32::MIN + 1, i32::MAX - 1][game.curr_move as usize])
    }
    else {
      Some(0)
    }
  } 
  else {
    None
  }
}

const MIDGAME: bool = false;
const ENDGAME: bool = true;

const PAWN_VAL: i32 = 100;
const BISHOP_VAL: i32 = 350;
const KNIGHT_VAL: i32 = 300;
const ROOK_VAL: i32 = 500;
const QUEEN_VAL: i32 = 900;
const KING_VAL: i32 = 400;

const MG_PAWN_TABLE: [[i32; 8]; 8] = [
  [0,  0,  0,  0,  0,  0,  0, 0],
  [0,  0,  0, -5, -5,  0,  0, 0],
  [0,  2,  3,  4,  4,  3,  2, 0],
  [0,  4,  6, 10, 10,  6,  4, 0],
  [0,  6,  9, 10, 10,  9,  6, 0],
  [4,  8, 12, 16, 16, 12,  8, 4],
  [5, 10, 15, 20, 20, 15, 10, 5],
  [0,  0,  0,  0,  0,  0,  0, 0],
];
const EG_PAWN_TABLE: [[i32; 8]; 8] = [
  [  0,   0,   0,   0,   0,   0,   0,   0],
  [ 13,   8,   8,  10,  13,   0,   2,  -7],
  [  4,   7,  -6,   1,   0,  -5,  -1,  -8],
  [ 13,   9,  -3,  -7,  -7,  -8,   3,  -1],
  [ 32,  24,  13,   5,  -2,   4,  17,  17],
  [ 94, 100,  85,  67,  56,  53,  82,  84],
  [178, 173, 158, 134, 147, 132, 165, 187],
  [  0,   0,   0,   0,   0,   0,   0,   0],
];
const MG_KNIGHT_TABLE: [[i32; 8]; 8] = [
  [-105, -21, -58, -33, -17, -28, -19,  -23],
  [ -29, -53, -12,  -3,  -1,  18, -14,  -19],
  [ -23,  -9,  12,  10,  19,  17,  25,  -16],
  [ -13,   4,  16,  13,  28,  19,  21,   -8],
  [  -9,  17,  19,  53,  37,  69,  18,   22],
  [ -47,  60,  37,  65,  84, 129,  73,   44],
  [ -73, -41,  72,  36,  23,  62,   7,  -17],
  [-167, -89, -34, -49,  61, -97, -15, -107],
];
const EG_KNIGHT_TABLE: [[i32; 8]; 8] = [
  [-29, -51, -23, -15, -22, -18, -50, -64],
  [-42, -20, -10,  -5,  -2, -20, -23, -44],
  [-23,  -3,  -1,  15,  10,  -3, -20, -22],
  [-18,  -6,  16,  25,  16,  17,   4, -18],
  [-17,   3,  22,  22,  22,  11,   8, -18],
  [-24, -20,  10,   9,  -1,  -9, -19, -41],
  [-25,  -8, -25,  -2,  -9, -25, -24, -52],
  [-58, -38, -13, -28, -31, -27, -63, -99],
];
const MG_BISHOP_TABLE: [[i32; 8]; 8] = [
  [-33,  -3, -14, -21, -13, -12, -39, -21],
  [  4,  15,  16,   0,   7,  21,  33,   1],
  [  0,  15,  15,  15,  14,  27,  18,  10],
  [ -6,  13,  13,  26,  34,  12,  10,   4],
  [ -4,   5,  19,  50,  37,  37,   7,  -2],
  [-16,  37,  43,  40,  35,  50,  37,  -2],
  [-26,  16, -18, -13,  30,  59,  18, -47],
  [-29,   4, -82, -37, -25, -42,   7,  -8],
];
const EG_BISHOP_TABLE: [[i32; 8]; 8] = [
  [-23,  -9, -23,  -5, -9, -16,  -5, -17],
  [-14, -18,  -7,  -1,  4,  -9, -15, -27],
  [-12,  -3,   8,  10, 13,   3,  -7, -15],
  [ -6,   3,  13,  19,  7,  10,  -3,  -9],
  [ -3,   9,  12,   9, 14,  10,   3,   2],
  [  2,  -8,   0,  -1, -2,   6,   0,   4],
  [ -8,  -4,   7, -12, -3, -13,  -4, -14],
  [-14, -21, -11,  -8, -7,  -9, -17, -24],
];
const MG_ROOK_TABLE: [[i32; 8]; 8] = [
  [-19, -13,   1,  17, 16,  7, -37, -26],
  [-44, -16, -20,  -9, -1, 11,  -6, -71],
  [-45, -25, -16, -17,  3,  0,  -5, -33],
  [-36, -26, -12,  -1,  9, -7,   6, -23],
  [-24, -11,   7,  26, 24, 35,  -8, -20],
  [ -5,  19,  26,  36, 17, 45,  61,  16],
  [ 27,  32,  58,  62, 80, 67,  26,  44],
  [ 32,  42,  32,  51, 63,  9,  31,  43],
];
const EG_ROOK_TABLE: [[i32; 8]; 8] = [
  [-9,  2,  3, -1, -5, -13,   4, -20],
  [-6, -6,  0,  2, -9,  -9, -11,  -3],
  [-4,  0, -5, -1, -7, -12,  -8, -16],
  [ 3,  5,  8,  4, -5,  -6,  -8, -11],
  [ 4,  3, 13,  1,  2,   1,  -1,   2],
  [ 7,  7,  7,  5,  4,  -3,  -5,  -3],
  [11, 13, 13, 11, -3,   3,   8,   3],
  [13, 10, 18, 15, 12,  12,   8,   5],
];
const MG_QUEEN_TABLE: [[i32; 8]; 8] = [
  [ -1, -18,  -9,  10, -15, -25, -31, -50],
  [-35,  -8,  11,   2,   8,  15,  -3,   1],
  [-14,   2, -11,  -2,  -5,   2,  14,   5],
  [ -9, -26,  -9, -10,  -2,  -4,   3,  -3],
  [-27, -27, -16, -16,  -1,  17,  -2,   1],
  [-13, -17,   7,   8,  29,  56,  47,  57],
  [-24, -39,  -5,   1, -16,  57,  28,  54],
  [-28,   0,  29,  12,  59,  44,  43,  45],
];
const EG_QUEEN_TABLE: [[i32; 8]; 8] = [
  [-33, -28, -22, -43,  -5, -32, -20, -41],
  [-22, -23, -30, -16, -16, -23, -36, -32],
  [-16, -27,  15,   6,   9,  17,  10,   5],
  [-18,  28,  19,  47,  31,  34,  39,  23],
  [  3,  22,  24,  45,  57,  40,  57,  36],
  [-20,   6,   9,  49,  47,  35,  19,   9],
  [-17,  20,  32,  41,  58,  25,  30,   0],
  [ -9,  22,  22,  27,  27,  19,  10,  20],
];
const MG_KING_TABLE: [[i32; 8]; 8] = [
  [-15,  36,  12, -54,   8, -28,  24,  14],
  [  1,   7,  -8, -64, -43, -16,   9,   8],
  [-14, -14, -22, -46, -44, -30, -15, -27],
  [-49,  -1, -27, -39, -46, -44, -33, -51],
  [-17, -20, -12, -27, -30, -25, -14, -36],
  [ -9,  24,   2, -16, -20,   6,  22, -22],
  [ 29,  -1, -20,  -7,  -8,  -4, -38, -29],
  [-65,  23,  16, -15, -56, -34,   2,  13],
];
const EG_KING_TABLE: [[i32; 8]; 8] = [
  [-53, -34, -21, -11, -28, -14, -24, -43],
  [-27, -11,   4,  13,  14,   4,  -5, -17],
  [-19,  -3,  11,  21,  23,  16,   7,  -9],
  [-18,  -4,  21,  24,  27,  23,   9, -11],
  [ -8,  22,  24,  27,  26,  33,  26,   3],
  [ 10,  17,  23,  15,  20,  45,  44,  13],
  [-12,  17,  14,  17,  17,  38,  23,  11],
  [-74, -35, -18, -18, -11,  15,   4, -17],
];
fn get_position_value (piece: &Pieces, [row, col]: [usize; 2], game_time: bool) -> i32 {
  if game_time == ENDGAME {
    match piece {
      Empty => 0,
      Pawn(WHITE) => EG_PAWN_TABLE[row][col],
      Pawn(BLACK) => -EG_PAWN_TABLE[7 - row][col],
      King(WHITE) => EG_KING_TABLE[row][col],
      King(BLACK) => -EG_KING_TABLE[7 - row][col],
      Queen(WHITE) => EG_QUEEN_TABLE[row][col],
      Queen(BLACK) => -EG_QUEEN_TABLE[7 - row][col],
      Rook(WHITE) => EG_ROOK_TABLE[row][col],
      Rook(BLACK) => -EG_ROOK_TABLE[7 - row][col],
      Bishop(WHITE) => EG_BISHOP_TABLE[row][col],
      Bishop(BLACK) => -EG_BISHOP_TABLE[7 - row][col],
      Knight(WHITE) => EG_KNIGHT_TABLE[row][col],
      Knight(BLACK) => -EG_KNIGHT_TABLE[7 - row][col],
    }
  }
  else {
    match piece {
      Empty => 0,
      Pawn(WHITE) => MG_PAWN_TABLE[row][col],
      Pawn(BLACK) => -MG_PAWN_TABLE[7 - row][col],
      King(WHITE) => MG_KING_TABLE[row][col],
      King(BLACK) => -MG_KING_TABLE[7 - row][col],
      Queen(WHITE) => MG_QUEEN_TABLE[row][col],
      Queen(BLACK) => -MG_QUEEN_TABLE[7 - row][col],
      Rook(WHITE) => MG_ROOK_TABLE[row][col],
      Rook(BLACK) => -MG_ROOK_TABLE[7 - row][col],
      Bishop(WHITE) => MG_BISHOP_TABLE[row][col],
      Bishop(BLACK) => -MG_BISHOP_TABLE[7 - row][col],
      Knight(WHITE) => MG_KNIGHT_TABLE[row][col],
      Knight(BLACK) => -MG_KNIGHT_TABLE[7 - row][col],
    }
  }
}

pub fn heuristic (game: &GameState) -> i32 {
  let mut result = 0;
  let game_time = if game.board.iter().flatten().filter(|x| x != &&Empty && x != &&Pawn(WHITE) && x != &&Pawn(BLACK)).count() > 6 { MIDGAME } else { ENDGAME };
  //Count Pieces and score pieces on position
  let mut piece_counts: HashMap<Pieces, i32> = HashMap::new();
  for (i, line) in game.board.iter().enumerate() {
      for (j, piece) in line.iter().enumerate() {
        *piece_counts.entry(*piece).or_default() += 1;
        //Get positional advantage of the piece
        result += get_position_value(piece, [i, j], game_time);
      }
  }
  //Add material value ot evaluation
  for (piece, number) in piece_counts.iter() {
    match piece {
      Empty => {},
      Queen(player) => result += [QUEEN_VAL, -QUEEN_VAL][*player as usize] * number,
      Rook(player) => result += [ROOK_VAL, -ROOK_VAL][*player as usize] * number,
      Bishop(player) => result += [BISHOP_VAL, -BISHOP_VAL][*player as usize] * number,
      Knight(player) => result += [KNIGHT_VAL, -KNIGHT_VAL][*player as usize] * number,
      King(player) => result += [KING_VAL, -KING_VAL][*player as usize] * number,
      Pawn(player) => result += [PAWN_VAL, -PAWN_VAL][*player as usize] * number,
    }
  }
  //Consider checks
  if let Some(player) = &game.check {
    if *player == game.curr_move {
      match *player {
        BLACK => result += 20,
        WHITE => result -= 20,
      }
    }
  }
  //Prefer to have two bishops
  if piece_counts.contains_key(&Bishop(game.curr_move)) {
    if piece_counts.get(&Bishop(game.curr_move)).unwrap() > &1 {
      result += 20;
    }
  }
  result
}

#[test]
fn digit_manipulation() {
  for (character, number) in [('a', 0), ('b', 1), ('c', 2), ('d', 3), ('e', 4), ('f', 5), ('g', 6), ('h', 7)] {
    assert!(to_num(character) == number);
    assert!(to_let(number) == character);
  }
}
