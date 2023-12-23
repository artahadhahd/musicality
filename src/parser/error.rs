use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub enum ParseResponse {
    Int(usize),
    EOL(usize),
    Ident(usize),
    Unexpected(usize),
    NotPossible,
    Done,
}

impl Error for ParseResponse {}

impl fmt::Display for ParseResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        type E = ParseResponse;
        match self {
            E::Int(l) => write!(f, "Line {l}: Expected an unsigned number"),
            E::EOL(l) => write!(f, "Line {l}: Expected end of line"),
            E::Ident(l) => write!(f, "Line {l}: Expected an identifier"),
            E::Unexpected(l) => write!(f, "Line {l}: Unexpected character(s)"),
            E::Done => write!(f, ""),
            E::NotPossible => unreachable!(),
        }
    }
}
