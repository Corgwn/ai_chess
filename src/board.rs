#[derive(Clone, Copy, Debug)]
pub struct Position {
  rank: usize,
  file: char,
}

#[derive(Clone, Copy, Debug)]
pub struct GameState {
  board: [[char; 8]; 8],
  curr_move: bool,
  castling_rights: [bool; 4],
  en_passant: Option<Position>,
  half_moves: usize,
  full_moves: usize,
}

impl GameState {
    pub fn from_fen (fen: &String) -> GameState {
      let mut fields = fen.split_ascii_whitespace();
      //Read board positions
      let temp = fields.next().unwrap();
      let board = temp.split('/');
      let mut board_state: [[char; 8]; 8] = [[' '; 8]; 8];
      for (index, line) in board.enumerate() {
        let temp_arr: [char; 8];
        let mut file: usize = 0;
        for piece in line.chars() {
          //Move the index forward when there are spaces
          if piece.is_digit(10) {
            file += piece.to_digit(10).unwrap() as usize;
          }
          //Add piece to board
          else {
            board_state[7 - index][file] = piece;
            file += 1;
          }
        }
      }

      //Read current move
      let player: bool;
      match fields.next().unwrap() {
        "b" => { player = true; },
        "w" => { player = false; },
        _ => panic!("Invalid FEN String")
      }

      //Read castling rights
      let temp = fields.next().unwrap();
      let rights: [bool; 4] = [temp.contains('K'), temp.contains('Q'), temp.contains('k'), temp.contains('q')];

      //Read En Passant targets
      let temp = fields.next().unwrap();
      let passant: Option<Position>;
      if !temp.eq("-") {
        passant = Some(Position {rank: temp.chars().nth(1).unwrap().to_digit(10).unwrap() as usize, file: temp.chars().nth(0).unwrap()});
      }
      else {
        passant = None
      }

      //Read moves
      let half_moves = fields.next().unwrap().parse::<usize>().unwrap();
      let full_moves = fields.next().unwrap().parse::<usize>().unwrap();

      GameState {board: board_state, curr_move: player, castling_rights: rights, en_passant: passant, half_moves: half_moves, full_moves: full_moves}
    }

    pub fn valid_moves (&self) -> Vec<String> {
      let moves = Vec::new();



      moves
    }
}

fn rook_moves (state: &GameState, start: [usize; 2], player: bool) {

}

fn knight_moves (state: &GameState, start: [usize; 2], player: bool) {

}

fn bishop_moves (state: &GameState, start: [usize; 2], player: bool) {

}

fn queen_moves (state: &GameState, start: [usize; 2], player: bool) {

}

fn king_moves (state: &GameState, start: [usize; 2], player: bool) {

}

fn pawn_moves (state: &GameState, start: [usize; 2], player: bool) {
  
}