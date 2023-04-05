use std::io::Empty;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pieces {
  Knight(bool),
  Rook(bool),
  Bishop(bool),
  Queen(bool),
  King(bool),
  Pawn(bool),
  Empty,
}

impl Pieces {
  pub fn from (piece: &char) -> Pieces {
    match piece {
      'r' => Pieces::Rook(true),
      'n' => Pieces::Knight(true),
      'b' => Pieces::Bishop(true),
      'q' => Pieces::Queen(true),
      'k' => Pieces::King(true),
      'p' => Pieces::Pawn(true),
      'P' => Pieces::Pawn(false),
      'K' => Pieces::King(false),
      'Q' => Pieces::Queen(false),
      'B' => Pieces::Bishop(false),
      'N' => Pieces::Knight(false),
      'R' => Pieces::Rook(false),
      _ => Pieces::Empty
    }
  }
}

#[derive(Clone, Copy, Debug)]
enum Castles {
  WhiteKing,
  WhiteQueen,
  BlackKing,
  BlackQueen,
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
  start: [usize; 2],
  end: [usize; 2],
  castle: Option<Castles>,
  promote: Option<Pieces>,
}

#[derive(Clone, Debug)]
pub struct GameState {
  board: [[Pieces; 8]; 8],
  //False represents white, true represents black
  curr_move: bool,
  //In order: white kingside, white queenside, black kingside, black queenside
  castling_rights: [bool; 4],
  //None if no en passant is possible, Some if possible by taking the position given with a pawn
  en_passant: Option<[usize; 2]>,
  half_moves: usize,
  full_moves: usize,
  check: Option<bool>,
  white_king: [usize; 2],
  black_king: [usize; 2],
}

impl GameState {
    pub fn from_fen (fen: &str) -> GameState {
      let mut fields = fen.split_ascii_whitespace();
      //Read board positions
      let temp = fields.next().unwrap();
      let board = temp.split('/');
      let mut board_state: [[Pieces; 8]; 8] = [[Pieces::Empty; 8]; 8];
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
        "b" => true,
        "w" => false,
        _ => panic!("Invalid FEN String")
      };

      //Read castling rights
      let temp = fields.next().unwrap();
      let rights: [bool; 4] = [temp.contains('K'), temp.contains('Q'), temp.contains('k'), temp.contains('q')];

      //Read En Passant targets
      let temp = fields.next().unwrap();
      let passant: Option<[usize; 2]> = if !temp.eq("-") {
        Some([to_num(temp.chars().nth(1).unwrap()), to_num(temp.chars().next().unwrap())])
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
      for i in 0..8 {
        for j in 0..8 {
          match board_state[i][j] {
            Pieces::King(true) => black_king = [i, j],
            Pieces::King(false) => white_king = [i, j],
            _ => continue,
          }
        }
      }

      let mut game = GameState {board: board_state, curr_move: player, castling_rights: rights, en_passant: passant, half_moves, full_moves, check: None, white_king, black_king};

      //Calculate if a king is in check
      game.check = is_in_check(game);

      game
    }

    pub fn valid_moves (&self) -> Vec<String> {
      let mut moves = Vec::new();

      for i in 0..8 {
        for j in 0..8 {
          match self.board[i][j] {
            Pieces::Empty => continue,
            Pieces::Rook(player) => moves.append(&mut rook_moves(self, [i, j], player)),
            Pieces::Knight(player) => moves.append(&mut knight_moves(self, [i, j], player)),
            Pieces::Bishop(player) => moves.append(&mut bishop_moves(self, [i, j], player)),
            Pieces::Queen(player) => moves.append(&mut queen_moves(self, [i, j], player)),
            Pieces::King(player) => moves.append(&mut king_moves(self, [i, j], player)),
            Pieces::Pawn(true) => moves.append(&mut black_pawn_moves(self, [i, j])),
            Pieces::Pawn(false) => moves.append(&mut white_pawn_moves(self, [i, j])),
          }
        }
      }
      moves
    }

    pub fn make_move (&mut self, mov: Move) {
      //Expects mov to be a valid move
      let piece = self.board[mov.start[0]][mov.start[1]];
      self.board[mov.end[0]][mov.end[1]] = piece;
      self.board[mov.start[0]][mov.start[1]] = Pieces::Empty;
      if let Some(castle_type) = mov.castle {
        match castle_type {
          Castles::WhiteKing => self.make_move(Move {start: [0, 7], end: [0, 5], castle: None, promote: None}),
          Castles::WhiteQueen => self.make_move(Move {start: [0, 0], end: [0, 3], castle: None, promote: None}),
          Castles::BlackKing => self.make_move(Move {start: [7, 7], end: [7, 5], castle: None, promote: None}),
          Castles::BlackQueen => self.make_move(Move {start: [7, 0], end: [7, 3], castle: None, promote: None}),
        }
      }
    }
}

fn rook_moves (state: &GameState, start: [usize; 2], player: bool) -> Vec<String> {
  let moves = Vec::new();
  if player != state.curr_move {
    return moves;
  }

  moves
}

fn knight_moves (state: &GameState, start: [usize; 2], player: bool) -> Vec<String> {
  let moves = Vec::new();
  if player != state.curr_move {
    return moves;
  }

  moves
}

fn bishop_moves (state: &GameState, start: [usize; 2], player: bool) -> Vec<String> {
  let moves = Vec::new();
  if player != state.curr_move {
    return moves;
  }

  moves
}

fn queen_moves (state: &GameState, start: [usize; 2], player: bool) -> Vec<String> {
  let moves = Vec::new();
  if player != state.curr_move {
    return moves;
  }

  moves
}

fn king_moves (state: &GameState, start: [usize; 2], player: bool) -> Vec<String> {
  let moves = Vec::new();
  if player != state.curr_move {
    return moves;
  }

  moves
}

fn white_pawn_moves (state: &GameState, start: [usize; 2]) -> Vec<String> {
  let moves = Vec::new();
  if state.curr_move {
    return moves;
  }
  
  moves
}

fn black_pawn_moves (state: &GameState, start: [usize; 2]) -> Vec<String> {
  let moves = Vec::new();
  if !state.curr_move {
    return moves;
  }
  
  moves
}

fn is_in_bounds (pos: [i32; 2]) -> bool {
  if pos[0] > 7 || pos[0] < 0 || pos[1] > 7 || pos[1] < 0 {
    return false;
  }
  true
}

fn is_in_check (game: GameState) -> Option<bool> {
  let result = None;

  //White King
  //Ray attacks
  'dir: for direction in [[0, 1], [0, -1], [1, 0], [-1, 0], [1, 1], [1, -1], [-1, 1], [-1, -1]] {
    let mut row = game.white_king[0] as i32 + direction[0];
    let mut col = game.white_king[1] as i32 + direction[1];
    if !is_in_bounds([row, col]) {
        continue 'dir;
    } 
    while game.board[row as usize][col as usize] == Pieces::Empty {
      row += direction[0];
      col += direction[1];
      if !is_in_bounds([row, col]) {
        continue 'dir;
      } 
    }
    match game.board[row as usize][col as usize] {
      Pieces::Bishop(true) | Pieces::Rook(true) | Pieces::Queen(true) | Pieces::Pawn(true) => return Some(false),
      _ => {},
    }
  }
  //Knight attacks
  

  result
}

fn to_let(num: usize) -> char {
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

fn to_num(letter: char) -> usize {
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
