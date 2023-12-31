use core::fmt;

use crate::parser::data::{Chord, Note, NoteModifier, NoteName, Variable};
use crate::parser::error::ParseResponse;
#[derive(Debug)]
pub enum MusicalValues {
    Label(String),
    Chord(Chord),
    Var(Variable),
    Pair((String, String)),
}

impl fmt::Display for MusicalValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Label(s) => write!(f, "@{s}"),
            Self::Pair(p) => write!(f, "pair '{} {}'", p.0, p.1),
            Self::Chord(_) => write!(f, "<chord>"),
            Self::Var(v) => write!(f, "{} = {}", v.name, v.value),
        }
    }
}

pub struct Parser<'a> {
    input: &'a str,
    cursor: usize,
    lines: usize,
    size: usize,
}

pub trait ParsingFunctions {
    fn has_next(&self) -> bool;
    fn next(&mut self) -> Result<MusicalValues, ParseResponse>;
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
        while self.has_next() {
            let next = self.next()?;
            println!("{next:?}");
        }
        Ok(())
    }

    fn next(&mut self) -> Result<MusicalValues, ParseResponse> {
        try_to_parse!(self.label(), MusicalValues::Label);
        try_to_parse!(self.chord(), MusicalValues::Chord);
        let old = self.cursor;
        // try_to_parse!(self.pair(), MusicalValues::Pair);
        match self.pair().map(MusicalValues::Pair) {
            Ok(v) => return Ok(v),
            Err(ParseResponse::NotPossible) => self.cursor = old,
            Err(e) => return Err(e),
        }
        try_to_parse!(self.variable(), MusicalValues::Var);
        self.skip_whitespace();
        if !self.has_next() {
            Err(ParseResponse::Done)
        } else {
            println!("{:?}", self.input.chars().nth(self.cursor));
            Err(ParseResponse::Unexpected(self.lines))
        }
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

    fn variable(&mut self) -> Result<Variable, ParseResponse> {
        let name = self.ident()?;
        if !self.symbol(':') {
            return Err(ParseResponse::NotPossible);
        }
        let value = self.unsigned_int()? as f32;
        Ok(Variable { name, value })
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

    fn pair(&mut self) -> Result<(String, String), ParseResponse> {
        let fst = self.ident()?;
        let snd = self.ident()?;
        Ok((fst, snd))
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
        Ok(Note { note, modifier })
    }

    fn chord(&mut self) -> Result<Chord, ParseResponse> {
        let mut notes: Vec<Note> = vec![];
        loop {
            let note = self.get_note();
            if note.is_err() {
                break;
            }
            notes.push(note.unwrap());
        }
        if notes.is_empty() {
            return Err(ParseResponse::NotPossible);
        }
        let duration = self.duration()?;
        Ok(Chord { notes, duration })
    }

    pub fn get_err_line(&self) -> String {
        let start = {
            let mut cursor = self.cursor;
            while cursor > 0 {
                if let Some(c) = self.input.bytes().nth(cursor) {
                    if c == b'\n' {
                        break;
                    }
                } else {
                    break;
                }
                cursor -= 1;
            }
            cursor
        };
        let end = {
            let mut cursor = self.cursor;
            while self.has_next() {
                if let Some(c) = self.input.bytes().nth(cursor) {
                    if c == b'\n' {
                        break;
                    }
                } else {
                    break;
                }
                cursor += 1;
            }
            cursor
        };
        let lines = {
            let mut out = "".to_string();
            for i in start..end {
                out.push(self.input.chars().nth(i).unwrap());
            }
            out
        };
        lines
    }
}
