use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum ChessError {
    #[snafu(display("Move given is not valid"))]
    MoveParseAlphaNumError,
    #[snafu(display("Move given is not correct length"))]
    MoveParseLengthError,
}
