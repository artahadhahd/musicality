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

#[derive(Debug)]
pub enum NoteModifier {
    Flat,
    Sharp,
    None,
}

#[derive(Debug)]
pub enum NoteName {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug)]
pub struct Note {
    pub note: NoteName,
    pub modifier: NoteModifier,
}

#[derive(Debug)]
pub struct Chord {
    pub notes: Vec<Note>,
    pub duration: f32,
}

#[derive(Debug)]
pub enum MusicalValues {
    Label(String),
    Duration(f32),
    Note(Note),
    Chord(Chord),
}

pub struct Parser<'a> {
    input: &'a str,
    cursor: usize,
    lines: usize,
    size: usize,
}

pub trait ParsingFunctions {
    fn has_next(&self) -> bool;
    fn skip_whitespace(&mut self);
    fn ident(&mut self) -> Result<String, ParseResponse>;
    fn unsigned_int(&mut self) -> Result<usize, ParseResponse>;
    fn symbol(&mut self, c: char) -> bool;
    fn force_end(&mut self) -> Result<(), ParseResponse>;
    fn parse(&mut self) -> Result<(), ParseResponse>;
}

impl<'a> From<&'a str> for Parser<'a> {
    fn from(value: &'a str) -> Self {
        let input = value;
        let cursor = 0;
        let lines = 0;
        let size = input.len();
        Self {
            input,
            cursor,
            lines,
            size,
        }
    }
}

macro_rules! try_to_parse {
    ($f:expr, $t:expr) => {
        match $f.map($t) {
            Ok(v) => return Ok(v),
            Err(ParseResponse::NotPossible) => (),
            Err(e) => return Err(e),
        }
    };
}

impl<'a> ParsingFunctions for Parser<'a> {
    fn has_next(&self) -> bool {
        self.cursor < self.size
    }

    fn skip_whitespace(&mut self) {
        while self.has_next() {
            match self.input.chars().nth(self.cursor) {
                Some(' ' | '\t' | '\r' | ';') => self.cursor += 1,
                Some('\n') => {
                    self.cursor += 1;
                    self.lines += 1;
                }
                Some('#') => self.skip_comment(),
                Some('<') => self.skip_ml_comment(),
                _ => break,
            }
        }
    }

    fn ident(&mut self) -> Result<String, ParseResponse> {
        self.skip_whitespace();
        let mut out = "".to_string();
        let identifier = loop {
            if !self.has_next() {
                break out;
            }
            match self.input.chars().nth(self.cursor) {
                Some(c) => {
                    if c.is_alphabetic() || c == '.' || c == '_' {
                        out.push(c);
                        self.cursor += 1;
                    } else {
                        break out;
                    }
                }
                _ => break out,
            }
        };
        if identifier.is_empty() {
            Err(ParseResponse::NotPossible)
        } else {
            Ok(identifier)
        }
    }

    fn unsigned_int(&mut self) -> Result<usize, ParseResponse> {
        self.skip_whitespace();
        let mut out = "".to_string();
        let num = loop {
            if !self.has_next() {
                break out;
            }
            match self.input.chars().nth(self.cursor) {
                Some(c) => {
                    if c.is_numeric() {
                        out.push(c);
                        self.cursor += 1;
                    } else {
                        break out;
                    }
                }
                _ => {
                    break out;
                }
            }
        };
        if num.is_empty() {
            Err(ParseResponse::NotPossible)
        } else {
            if let Ok(num) = num.parse::<usize>() {
                Ok(num)
            } else {
                Err(ParseResponse::Int(self.cursor))
            }
        }
    }

    fn symbol(&mut self, c: char) -> bool {
        self.skip_whitespace();
        if let Some(m) = self.input.chars().nth(self.cursor) {
            let is = m == c;
            if is {
                self.cursor += 1;
            }
            is
        } else {
            false
        }
    }

    fn force_end(&mut self) -> Result<(), ParseResponse> {
        self.skip_whitespace();
        if let Some(c) = self.input.bytes().nth(self.cursor) {
            if c != b'\n' && c != b';' {
                return Err(ParseResponse::EOL(self.lines));
            }
        }
        self.cursor += 1;
        Ok(())
    }

    fn parse(&mut self) -> Result<(), ParseResponse> {
        // let _main = self.label();
        // let _ident = self.ident();
        // let _dur = self.duration()?;
        // dbg!(_main);
        while self.has_next() {
            let next = self.next()?;
            println!("{next:?}");
        }
        Ok(())
    }
}

impl<'a> Parser<'a> {
    fn skip_comment(&mut self) {
        while self.has_next() {
            match self.input.chars().nth(self.cursor) {
                Some('\n') => {
                    self.lines += 1;
                    break;
                }
                None => break,
                _ => self.cursor += 1,
            }
        }
    }

    fn skip_ml_comment(&mut self) {
        while self.has_next() {
            match self.input.chars().nth(self.cursor) {
                Some('\n') => {
                    self.cursor += 1;
                    self.lines += 1;
                }
                Some('>') => {
                    self.cursor += 1;
                    break;
                }
                None => break,
                _ => self.cursor += 1,
            }
        }
    }

    fn duration(&mut self) -> Result<f32, ParseResponse> {
        let numerator = self.unsigned_int()? as f32;
        let denominator = if self.symbol('/') {
            self.unsigned_int()?
        } else {
            1
        } as f32;
        Ok(numerator / denominator)
    }

    fn label(&mut self) -> Result<String, ParseResponse> {
        if self.symbol('@') {
            let ident = self.ident();
            match ident {
                Err(ParseResponse::NotPossible) => Err(ParseResponse::Ident(self.lines)),
                Ok(v) => Ok(v),
                _ => unreachable!(),
            }
        } else {
            Err(ParseResponse::NotPossible)
        }
    }

    fn get_note(&mut self) -> Result<Note, ParseResponse> {
        let note = if self.symbol('A') {
            NoteName::A
        } else if self.symbol('B') || self.symbol('H') {
            NoteName::B
        } else if self.symbol('C') {
            NoteName::C
        } else if self.symbol('D') {
            NoteName::D
        } else if self.symbol('E') {
            NoteName::E
        } else if self.symbol('F') {
            NoteName::F
        } else if self.symbol('G') {
            NoteName::G
        } else {
            return Err(ParseResponse::NotPossible);
        };

        let modifier = match self.input.chars().nth(self.cursor) {
            Some(c) => match c {
                '#' => {
                    self.cursor += 1;
                    NoteModifier::Sharp
                }
                'b' => {
                    self.cursor += 1;
                    NoteModifier::Flat
                }
                _ => {
                    self.skip_whitespace();
                    NoteModifier::None
                }
            },
            None => return Err(ParseResponse::NotPossible),
        };
        Ok(Note {
            note,
            modifier,
        })
    }

    fn chord(&mut self) -> Result<Chord, ParseResponse> {
        let mut notes: Vec<Note> = vec![];
        loop {
            let note = self.get_note();
            if note.is_err() {
                break;
            }
            notes.push(note.unwrap());
        };
        if notes.is_empty() {
            return Err(ParseResponse::NotPossible);
        }
        let duration = self.duration()?;
        Ok(Chord {
            notes,
            duration,
        })
    }

    pub fn next(&mut self) -> Result<MusicalValues, ParseResponse> {
        try_to_parse!(self.label(), MusicalValues::Label);
        try_to_parse!(self.chord(), MusicalValues::Chord);
        self.skip_whitespace();
        if !self.has_next() {
            Err(ParseResponse::Done)
        } else {
            Err(ParseResponse::Unexpected(self.lines))
        }
    }
}

fn main() -> Result<(), ParseResponse> {
    let mut parser = Parser::from(include_str!("../test.musical"));
    // let parsed = parser.parse();
    // if let Err(e) = parsed {
    //     if e != ParseError::Done {
    //         println!("{e}");
    //     }
    // } else {
    //     println!("{:?}", parsed.unwrap());
    // }
    // println!("{:?}", parser.next());
    // println!("{:?}", parser.next());
    parser.parse()?;
    Ok(())
}
