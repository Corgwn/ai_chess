use std::{
    fmt::{self, Display},
    ops::Not,
};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum PieceTypes {
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
    Pawn,
    Empty,
    Offboard,
}

impl PieceTypes {
    pub(crate) const fn value(self) -> i32 {
        match self {
            Self::Knight => 320,
            Self::Rook => 500,
            Self::Bishop => 330,
            Self::Queen => 900,
            Self::King => 20000,
            Self::Pawn => 100,
            Self::Empty | Self::Offboard => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum PieceColors {
    Black,
    White,
    Empty,
}

impl Not for PieceColors {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
            Self::Empty => Self::Empty,
        }
    }
}

impl Display for PieceColors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Black => write!(f, "Black"),
            Self::White => write!(f, "White"),
            Self::Empty => write!(f, ""),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Pieces {
    pub piece_type: PieceTypes,
    pub color: PieceColors,
}

impl Pieces {
    #[must_use]
    pub const fn from(piece: &char) -> Self {
        match piece {
            'r' => Self {
                piece_type: PieceTypes::Rook,
                color: PieceColors::Black,
            },
            'n' => Self {
                piece_type: PieceTypes::Knight,
                color: PieceColors::Black,
            },
            'b' => Self {
                piece_type: PieceTypes::Bishop,
                color: PieceColors::Black,
            },
            'q' => Self {
                piece_type: PieceTypes::Queen,
                color: PieceColors::Black,
            },
            'k' => Self {
                piece_type: PieceTypes::King,
                color: PieceColors::Black,
            },
            'p' => Self {
                piece_type: PieceTypes::Pawn,
                color: PieceColors::Black,
            },
            'R' => Self {
                piece_type: PieceTypes::Rook,
                color: PieceColors::White,
            },
            'N' => Self {
                piece_type: PieceTypes::Knight,
                color: PieceColors::White,
            },
            'B' => Self {
                piece_type: PieceTypes::Bishop,
                color: PieceColors::White,
            },
            'Q' => Self {
                piece_type: PieceTypes::Queen,
                color: PieceColors::White,
            },
            'K' => Self {
                piece_type: PieceTypes::King,
                color: PieceColors::White,
            },
            'P' => Self {
                piece_type: PieceTypes::Pawn,
                color: PieceColors::White,
            },
            _ => Self {
                piece_type: PieceTypes::Empty,
                color: PieceColors::Empty,
            },
        }
    }

    pub fn get_color(&self) -> bool {
        match self.color {
            PieceColors::Black => true,
            PieceColors::White | PieceColors::Empty => false,
        }
    }
}

impl fmt::Display for Pieces {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece = match self {
            Pieces {
                piece_type: PieceTypes::Rook,
                color: PieceColors::Black,
            } => "r",
            Pieces {
                piece_type: PieceTypes::Knight,
                color: PieceColors::Black,
            } => "n",
            Pieces {
                piece_type: PieceTypes::Bishop,
                color: PieceColors::Black,
            } => "b",
            Pieces {
                piece_type: PieceTypes::Queen,
                color: PieceColors::Black,
            } => "q",
            Pieces {
                piece_type: PieceTypes::King,
                color: PieceColors::Black,
            } => "k",
            Pieces {
                piece_type: PieceTypes::Pawn,
                color: PieceColors::Black,
            } => "p",
            Pieces {
                piece_type: PieceTypes::Rook,
                color: PieceColors::White,
            } => "R",
            Pieces {
                piece_type: PieceTypes::Knight,
                color: PieceColors::White,
            } => "N",
            Pieces {
                piece_type: PieceTypes::Bishop,
                color: PieceColors::White,
            } => "B",
            Pieces {
                piece_type: PieceTypes::Queen,
                color: PieceColors::White,
            } => "Q",
            Pieces {
                piece_type: PieceTypes::King,
                color: PieceColors::White,
            } => "K",
            Self {
                piece_type: PieceTypes::Pawn,
                color: PieceColors::White,
            } => "P",
            Self {
                piece_type: PieceTypes::Offboard,
                ..
            } => "O",
            Self {
                piece_type: PieceTypes::Empty,
                ..
            } => " ",
            _ => "",
        };
        write!(f, "{piece}")
    }
}
