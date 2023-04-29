use std::fmt;

const WHITE: bool = false;
const BLACK: bool = true;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug)]
pub enum PassantTypes {
  PassantCapture ([usize; 2]),
  PassantAvailable ([usize; 2]),
}

#[derive(Clone, Copy, Debug)]
pub enum CastleTypes {
  WhiteKing,
  WhiteQueen,
  BlackKing,
  BlackQueen,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Move {
  pub start: [usize; 2],
  pub end: [usize; 2],
  pub castle: Option<CastleTypes>,
  pub promote: Option<Pieces>,
  pub passant: Option<PassantTypes>,
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
  pub valid_moves: Vec<Move>,
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

      let mut game = GameState {board: board_state, curr_move: player, castling_rights: rights, en_passant: passant, half_moves, full_moves, check: None, white_king, black_king, valid_moves: vec![]};

      //Calculate if a king is in check
      game.check = is_in_check(&game);

      //Calculate valid moves
      game.valid_moves = game.valid_moves();

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

    pub fn make_move (&self, mov: Move) -> GameState {
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
    //Check if moving in this direction would put the current player in check
    let mov = Move {start, end: [row as usize, col as usize], ..Default::default()};
    if !player && is_white_checked(&game.make_move(mov)){
      continue 'dir;
    }
    if player && is_black_checked(&game.make_move(mov)){
      continue 'dir;
    }
    //Add moves until we hit a piece or run off the board
    while game.board[row as usize][col as usize] == Empty {
      let mov = Move {start, end: [row as usize, col as usize], ..Default::default()};
      if !player && !is_white_checked(&game.make_move(mov)){
        moves.push(mov);
      }
      if player && !is_black_checked(&game.make_move(mov)){
        moves.push(mov);
      }
      row += direction[0];
      col += direction[1];
      if !is_in_bounds([row, col]) {
        continue 'dir;
      }
    }
    //Find which piece we've hit and add moves acccordingly
    match game.board[row as usize][col as usize] {
      //Black piece
      Rook(BLACK) | Queen(BLACK) | Bishop(BLACK) | King(BLACK) | Knight(BLACK) | Pawn(BLACK) => {
        //White's turn
        if !player {
          moves.push(Move {start, end: [row as usize, col as usize], ..Default::default()});
        }
      },
      //White's piece
      Rook(WHITE) | Queen(WHITE) | Bishop(WHITE) | King(WHITE) | Knight(WHITE) | Pawn(WHITE) => {
        //Black's turn
        if player {
          moves.push(Move {start, end: [row as usize, col as usize], ..Default::default()});
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
            if !is_white_checked(&game.make_move(mov)){
              moves.push(mov);
            }
          }
        },
        //White piece
        Rook(WHITE) | Queen(WHITE) | Bishop(WHITE) | King(WHITE) | Knight(WHITE) | Pawn(WHITE) => {
          //Black's turn
          if player {
            let mov = Move {start, end: [row as usize, col as usize], ..Default::default()};
            if !is_black_checked(&game.make_move(mov)){
              moves.push(mov);
            }
          }
        },
        //If it's an empty space, either player can move there
        Empty => {
          let mov = Move {start, end: [row as usize, col as usize], ..Default::default()};
          if !player && !is_white_checked(&game.make_move(mov)){
            moves.push(mov);
          }
          if player && !is_black_checked(&game.make_move(mov)){
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
      if !player && !is_white_checked(&game.make_move(mov)){
        moves.push(mov);
      }
      if player && !is_black_checked(&game.make_move(mov)){
        moves.push(mov);
      }
      row += direction[0];
      col += direction[1];
      if !is_in_bounds([row, col]) {
        continue 'dir;
      }
    }
    //Find which piece we hit and add moves accordingly
    let mov = Move {start, end: [row as usize, col as usize], ..Default::default()};
    if !player && is_white_checked(&game.make_move(mov)){
      continue 'dir;
    }
    if player && is_black_checked(&game.make_move(mov)){
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
          if !is_white_checked(&game.make_move(mov)){
            moves.push(mov);
          }
        }
      },
      //If it's a white piece
      Rook(WHITE) | Queen(WHITE) | Bishop(WHITE) | King(WHITE) | Knight(WHITE) | Pawn(WHITE) => {
        //If it's black's turn
        if player {
          let mov = Move {start, end: [row as usize, col as usize], ..Default::default()};
          if !is_black_checked(&game.make_move(mov)){
            moves.push(mov);
          }
        }
      },
      //If it's an empty space, either player can move there
      Empty => {
        let mov = Move {start, end: [row as usize, col as usize], ..Default::default()};
        if !player && !is_white_checked(&game.make_move(mov)){
          moves.push(mov);
        }
        if player && !is_black_checked(&game.make_move(mov)){
          moves.push(mov);
        }
      },
    }
  }
  //If someone is in check, either the game is over or it's the current player and therefore can't castle
  if game.check != None {
    return moves;
  }
  match player {
    //White Castles
    WHITE => {
      //King side Castle
      if game.castling_rights[0] && game.board[0][5] == Empty && game.board[0][6] == Empty {
        let mov1 = Move {start, end: [0, 5], ..Default::default()};
        let mov2 = Move {start, end: [0, 6], ..Default::default()};
        if !is_white_checked(&game.make_move(mov1)) && !is_white_checked(&game.make_move(mov2)) {
          moves.push(Move {start, end: [0, 6], castle: Some(CastleTypes::WhiteKing), ..Default::default()});
        }
      }
      //Queen side Castle
      if game.castling_rights[1] && game.board[0][1] == Empty && game.board[0][2] == Empty && game.board[0][3] == Empty {
        let mov1 = Move {start, end: [0, 3], ..Default::default()};
        let mov2 = Move {start, end: [0, 2], ..Default::default()};
        if !is_white_checked(&game.make_move(mov1)) && !is_white_checked(&game.make_move(mov2)) {
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
        if !is_black_checked(&game.make_move(mov1)) && !is_black_checked(&game.make_move(mov2)) {
          moves.push(Move {start, end: [7, 6], castle: Some(CastleTypes::BlackKing), ..Default::default()});
        }
      }
      //Queen side Castle
      if game.castling_rights[3] && game.board[7][1] == Empty && game.board[7][2] == Empty && game.board[7][3] == Empty {
        let mov1 = Move {start, end: [7, 3], ..Default::default()};
        let mov2 = Move {start, end: [7, 2], ..Default::default()};
        if !is_black_checked(&game.make_move(mov1)) && !is_black_checked(&game.make_move(mov2)) {
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
    if !check_function(&game.make_move(mov)){
      if row == [7, 0][game.curr_move as usize] {
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
  if start[0] == [1, 7][player as usize] {
    let row = start[0] as i32 + forward * 2;
    let col = start[1] as i32;
    if game.board[(row - 1) as usize][col as usize] == Empty && game.board[row as usize][col as usize] == Empty {
      let passant: Option<PassantTypes> = Some(PassantTypes::PassantAvailable([(row + -forward) as usize, col as usize]));
      let mov = Move {start, end: [row as usize, col as usize], passant, ..Default::default()};
      if !check_function(&game.make_move(mov)) {
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
    if passant_cap && !check_function(&game.make_move(mov)) {
      //Passant captures cannot promote
      moves.push(Move { start, end: passant_pos, passant: Some(PassantTypes::PassantCapture([(row + -forward) as usize, col as usize])), ..Default::default()});
    }
    else {
      match game.board[row as usize][col as usize] {
        Rook(player) | Queen(player) | Bishop(player) | King(player) | Knight(player) | Pawn(player) => {
          if player != game.curr_move && !check_function(&game.make_move(mov)){
            if row == [7, 0][game.curr_move as usize] {
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
    if passant_cap && !check_function(&game.make_move(mov)) {
      //Passant captures cannot promote
      moves.push(Move { start, end: passant_pos, passant: Some(PassantTypes::PassantCapture([(row + -forward) as usize, col as usize])), ..Default::default()});
    }
    else {
      match game.board[row as usize][col as usize] {
        Rook(player) | Queen(player) | Bishop(player) | King(player) | Knight(player) | Pawn(player) => {
          if player != game.curr_move && !check_function(&game.make_move(mov)){
            if row == [7, 0][game.curr_move as usize] {
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

pub fn term (game: &GameState) -> Option<f32> {
  if game.valid_moves.len() == 0 {
    if game.check != None {
      return Some([f32::MIN, f32::MAX][game.curr_move as usize]);
    }
    else {
      return Some(0.0);
    }
  } 
  else {
    None
  }
}

pub fn heuristic (game: &GameState) -> f32 {
  todo!()
}

#[test]
fn digit_manipulation() {
  for (character, number) in [('a', 0), ('b', 1), ('c', 2), ('d', 3), ('e', 4), ('f', 5), ('g', 6), ('h', 7)] {
    assert!(to_num(character) == number);
    assert!(to_let(number) == character);
  }
}
