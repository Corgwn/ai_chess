use std::fmt;

pub(crate) const WHITE: bool = false;
pub(crate) const BLACK: bool = true;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Pieces {
    Knight(bool),
    Rook(bool),
    Bishop(bool),
    Queen(bool),
    King(bool),
    Pawn(bool),
    Empty,
}

impl Pieces {
    pub fn from(piece: &char) -> Pieces {
        match piece {
            'r' => Pieces::Rook(BLACK),
            'n' => Pieces::Knight(BLACK),
            'b' => Pieces::Bishop(BLACK),
            'q' => Pieces::Queen(BLACK),
            'k' => Pieces::King(BLACK),
            'p' => Pieces::Pawn(BLACK),
            'P' => Pieces::Pawn(WHITE),
            'K' => Pieces::King(WHITE),
            'Q' => Pieces::Queen(WHITE),
            'B' => Pieces::Bishop(WHITE),
            'N' => Pieces::Knight(WHITE),
            'R' => Pieces::Rook(WHITE),
            _ => Pieces::Empty,
        }
    }
}

impl fmt::Display for Pieces {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece = match self {
            Pieces::Rook(_) => "r",
            Pieces::Knight(_) => "n",
            Pieces::Bishop(_) => "b",
            Pieces::Queen(_) => "q",
            Pieces::King(_) => "k",
            Pieces::Pawn(_) => "p",
            _ => "",
        };
        write!(f, "{}", piece)
    }
}
