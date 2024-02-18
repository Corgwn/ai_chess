use crate::utils::pieces::Pieces;
use std::fmt;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameMove {
    pub start: [usize; 2],
    pub end: [usize; 2],
    pub castle: Option<CastleTypes>,
    pub promote: Option<Pieces>,
    pub passant: Option<PassantTypes>,
    pub capture: bool,
}

impl fmt::Display for GameMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let promotion;
        if let Some(piece) = self.promote {
            promotion = piece.to_string();
        } else {
            promotion = "".to_owned();
        }
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

impl Ord for GameMove {
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
