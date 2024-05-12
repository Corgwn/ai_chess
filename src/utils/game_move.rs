use crate::utils::pieces::{Pieces, PieceColors};
use std::fmt;
use crate::utils::pieces::PieceColors::{Black, White};
use crate::utils::pieces::PieceTypes::{Bishop, Knight, Queen, Rook};

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct GameMove {
    pub start: [usize; 2],
    pub end: [usize; 2],
    pub castle: Option<CastleTypes>,
    pub promote: Option<Pieces>,
    pub passant: Option<PassantTypes>,
    pub capture: bool,
}

impl GameMove {
    pub(crate) fn from_str(input: &&str) -> GameMove {
        //println!("Converting str to move");
        let chars: Vec<char> = input.chars().collect();
        //println!("Chars: {:?}", chars);
        // Get start and end spaces
        let start = [
            chars[1].to_digit(10).unwrap().checked_sub(1).unwrap() as usize,
            to_num(chars[0]),
        ];
        let end = [
            chars[3].to_digit(10).unwrap().checked_sub(1).unwrap() as usize,
            to_num(chars[2]),
        ];
        //println!("Start: {:?} End: {:?}", start, end);
        // Get promotion if needed
        let promote = if input.len() == 5 {
            let color: PieceColors = if chars[3] == '8' { White } else { Black };
            match chars[4] {
                'b' => Some(Pieces { piece_type: Bishop, color}),
                'k' => Some(Pieces { piece_type: Knight, color}),
                'r' => Some(Pieces { piece_type: Rook, color}),
                'q' => Some(Pieces { piece_type: Queen, color}),
                _ => None,
            }
        } else {
            None
        };
        //println!("Promotion: {:?}", promote);
        // Check for castle moves
        let castle = match *input {
            "e1g1" => Some(CastleTypes::WhiteKing),
            "e1c1" => Some(CastleTypes::WhiteQueen),
            "e8g8" => Some(CastleTypes::BlackKing),
            "e8c8" => Some(CastleTypes::BlackQueen),
            _ => None,
        };
        //println!("Castle: {:?}", castle);

        GameMove {
            start,
            end,
            castle,
            promote,
            passant: None,
            capture: false,
        }
    }
}

impl fmt::Display for GameMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let promotion = match self.promote {
            Some(piece) => piece.to_string(),
            None => "".to_string(),
        };
        write!(
            f,
            "{}{}{}{}{}",
            to_let(self.start[1]),
            self.start[0] + 1,
            to_let(self.end[1]),
            self.end[0] + 1,
            promotion
        )
    }
}

impl fmt::Debug for GameMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Ord for GameMove {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut mov1_score = 0;
        let mut mov2_score = 0;
        //Find self's score
        // if let(check) = self.check {
        //  mov1_score += 1;
        // }
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
                PassantTypes::PassantAvailable(_) => {}
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
                PassantTypes::PassantAvailable(_) => {}
                PassantTypes::PassantCapture(_) => mov2_score += 15,
            }
        }
        mov1_score.cmp(&mov2_score)
    }
}

impl PartialOrd for GameMove {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PassantTypes {
    PassantCapture([usize; 2]),
    PassantAvailable([usize; 2]),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CastleTypes {
    WhiteKing,
    WhiteQueen,
    BlackKing,
    BlackQueen,
}

pub(crate) fn to_let(num: usize) -> char {
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

pub(crate) fn to_num(letter: char) -> usize {
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_str_to_game_move() {
        assert_eq!(
            GameMove::from_str(&"e2e4"),
            GameMove {
                start: [1, 4],
                end: [3, 4],
                castle: None,
                promote: None,
                passant: None,
                capture: false,
            }
        );
    }
}
