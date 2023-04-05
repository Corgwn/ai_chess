#[derive(Clone, Copy, Debug)]
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
        Some([temp.chars().nth(1).unwrap().to_digit(10).unwrap() as usize, to_num(temp.chars().next().unwrap())])
      }
      else {
        None
      };

      //Read move numbers
      let half_moves = fields.next().unwrap().parse::<usize>().unwrap();
      let full_moves = fields.next().unwrap().parse::<usize>().unwrap();

      //Calculate if a king is in check
      let check = is_in_check(board_state);

      GameState {board: board_state, curr_move: player, castling_rights: rights, en_passant: passant, half_moves, full_moves, check}
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

fn is_in_bounds (mov: [i32; 2]) -> bool {
  if mov[0] > 7 || mov[0] < 0 || mov[1] > 7 || mov[1] < 0 {
    return false;
  }
  true
}

fn is_in_check (board: [[Pieces; 8]; 8]) -> Option<bool> {
  let result = None;


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
