use crate::utils::chess_errors::ChessError;
use crate::utils::pieces::PieceColors::{Black, White};
use crate::utils::pieces::PieceTypes::{Bishop, Knight, Queen, Rook};
use crate::utils::pieces::{PieceColors, Pieces};
use std::fmt;

use super::position::Position;

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub(crate) struct GameMove1d {
    pub(crate) start: Position,
    pub(crate) end: Position,
    pub(crate) castle: Option<CastleTypes>,
    pub(crate) promote: Option<Pieces>,
    pub(crate) passant: Option<PassantTypes>,
    pub(crate) capture: bool,
}

impl GameMove1d {
    pub(crate) fn from_str(input: &str) -> Result<Self, ChessError> {
        if ![4 as usize, 5 as usize].contains(&input.len()) {
            return Err(ChessError::MoveParseLengthError);
        }
        if !input.chars().all(char::is_alphanumeric) {
            return Err(ChessError::MoveParseAlphaNumError);
        }
        let squares = if input.len() == 4 {
            input.split_at(2)
        } else {
            input[..4].split_at(2)
        };
        println!("Squares: {}, {}", squares.0, squares.1);

        // Get start and end spaces
        let start = to_num(squares.0);
        let end = to_num(squares.1);
        println!("Start: {}, End: {}", start, end);

        // Get promotion if needed
        println!("{}", input.len());
        let promote = if input.len() == 5 {
            let color: PieceColors = match input.chars().nth(3).unwrap() {
                '8' => White,
                '1' => Black,
                _ => PieceColors::Empty,
            };
            match input.chars().nth(4).unwrap() {
                'b' => Some(Pieces {
                    piece_type: Bishop,
                    color,
                }),
                'k' => Some(Pieces {
                    piece_type: Knight,
                    color,
                }),
                'r' => Some(Pieces {
                    piece_type: Rook,
                    color,
                }),
                'q' => Some(Pieces {
                    piece_type: Queen,
                    color,
                }),
                _ => None,
            }
        } else {
            None
        };
        println!("Promotion: {:?}", promote);
        // Check for castle moves
        let castle = match input {
            "e1g1" => Some(CastleTypes::WhiteKing),
            "e1c1" => Some(CastleTypes::WhiteQueen),
            "e8g8" => Some(CastleTypes::BlackKing),
            "e8c8" => Some(CastleTypes::BlackQueen),
            _ => None,
        };
        println!("Castle: {:?}", castle);

        Ok(GameMove1d {
            start: Position { value: start },
            end: Position { value: end },
            castle,
            promote,
            passant: None,
            capture: false,
        })
    }
}

impl fmt::Display for GameMove1d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let promotion = match self.promote {
            Some(piece) => piece.to_string().to_lowercase(),
            None => "".to_string(),
        };
        write!(
            f,
            "{}{}{}",
            to_str(self.start.value),
            to_str(self.end.value),
            promotion
        )
    }
}

impl fmt::Debug for GameMove1d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PassantTypes {
    PassantCapture(usize),
    PassantAvailable(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CastleTypes {
    WhiteKing,
    WhiteQueen,
    BlackKing,
    BlackQueen,
}

pub(crate) fn to_str(num: usize) -> String {
    let temp = num - 20;
    let row = (temp / 10) + 1;
    let col = temp.rem_euclid(10);
    let col_char = match col {
        1 => 'a',
        2 => 'b',
        3 => 'c',
        4 => 'd',
        5 => 'e',
        6 => 'f',
        7 => 'g',
        8 => 'h',
        _ => panic!(),
    };
    return col_char.to_string() + &row.to_string();
}

pub(crate) fn to_num(pos: &str) -> usize {
    let mut chars = pos.chars();
    let col: usize = match chars.next().unwrap() {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!(),
    };
    let row: usize = match chars.next().unwrap() {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        _ => panic!(),
    };
    21 + col + (10 * row)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_str_to_pos() {
        assert_eq!(to_num("a1"), 21);
        assert_eq!(to_num("h8"), 98);
        assert_eq!(to_num("e7"), 85);
    }

    #[test]
    fn test_pos_to_str() {
        assert_eq!(to_str(22), "b1");
        assert_eq!(to_str(84), "d7");
        assert_eq!(to_str(98), "h8");
    }

    #[test]
    fn test_str_to_game_move() {
        assert_eq!(
            GameMove1d::from_str(&"e2e4").unwrap(),
            GameMove1d {
                start: Position { value: 35 },
                end: Position { value: 55 },
                castle: None,
                promote: None,
                passant: None,
                capture: false,
            }
        );
        assert_eq!(
            GameMove1d::from_str(&"a7a8q").unwrap(),
            GameMove1d {
                start: Position { value: 81 },
                end: Position { value: 91 },
                castle: None,
                promote: Some(Pieces {
                    piece_type: Queen,
                    color: White
                }),
                passant: None,
                capture: false,
            }
        );
        assert_eq!(
            GameMove1d::from_str(&"d2d1k").unwrap(),
            GameMove1d {
                start: Position { value: 34 },
                end: Position { value: 24 },
                castle: None,
                promote: Some(Pieces {
                    piece_type: Knight,
                    color: Black
                }),
                passant: None,
                capture: false,
            }
        )
    }
}
