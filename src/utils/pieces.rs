use std::fmt;


pub(crate) const WHITE: bool = false;
pub(crate) const BLACK: bool = true;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum PieceTypes {
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
    Pawn,
    Empty,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum PieceColors {
    Black,
    White,
    Empty,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Pieces {
    pub piece_type: PieceTypes,
    pub color: PieceColors,
}

impl Pieces {
    pub fn from(piece: &char) -> Pieces {
        match piece {
            'r' => Pieces { piece_type: PieceTypes::Rook, color: PieceColors::Black },
            'n' => Pieces { piece_type: PieceTypes::Knight, color: PieceColors::Black },
            'b' => Pieces { piece_type: PieceTypes::Bishop, color: PieceColors::Black },
            'q' => Pieces { piece_type: PieceTypes::Queen, color: PieceColors::Black },
            'k' => Pieces { piece_type: PieceTypes::King, color: PieceColors::Black },
            'p' => Pieces { piece_type: PieceTypes::Pawn, color: PieceColors::Black },
            'R' => Pieces { piece_type: PieceTypes::Rook, color: PieceColors::White },
            'N' => Pieces { piece_type: PieceTypes::Knight, color: PieceColors::White },
            'B' => Pieces { piece_type: PieceTypes::Bishop, color: PieceColors::White },
            'Q' => Pieces { piece_type: PieceTypes::Queen, color: PieceColors::White },
            'K' => Pieces { piece_type: PieceTypes::King, color: PieceColors::White },
            'P' => Pieces { piece_type: PieceTypes::Pawn, color: PieceColors::White },
            _ => Pieces { piece_type: PieceTypes::Empty, color:PieceColors::Empty },
        }
    }
    pub fn get_color(&self) -> bool {
        match self.color {
            PieceColors::Black => true,
            PieceColors::White => false,
            PieceColors::Empty => false,
        }
    }
}

impl fmt::Display for Pieces {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece = match self {
            Pieces { piece_type: PieceTypes::Rook, color: PieceColors::Black } => "r",
            Pieces { piece_type: PieceTypes::Knight, color: PieceColors::Black } => "n",
            Pieces { piece_type: PieceTypes::Bishop, color: PieceColors::Black } => "b",
            Pieces { piece_type: PieceTypes::Queen, color: PieceColors::Black } => "q",
            Pieces { piece_type: PieceTypes::King, color: PieceColors::Black } => "k",
            Pieces { piece_type: PieceTypes::Pawn, color: PieceColors::Black } => "p",
            Pieces { piece_type: PieceTypes::Rook, color: PieceColors::White } => "R",
            Pieces { piece_type: PieceTypes::Knight, color: PieceColors::White } => "N",
            Pieces { piece_type: PieceTypes::Bishop, color: PieceColors::White } => "B",
            Pieces { piece_type: PieceTypes::Queen, color: PieceColors::White } => "Q",
            Pieces { piece_type: PieceTypes::King, color: PieceColors::White } => "K",
            Pieces { piece_type: PieceTypes::Pawn, color: PieceColors::White } => "P",
            _ => " ",
        };
        write!(f, "{}", piece)
    }
}
