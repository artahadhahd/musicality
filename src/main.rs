use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ParseError {
    Int(usize),
    EOL(usize),
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        type E = ParseError;
        match self {
            E::Int(l) => write!(f, "Line {l}: Expected an unsigned number"),
            E::EOL(l) => write!(f, "Line {l}: Expected end of line"),
        }
    }
}

#[derive(Debug)]
pub enum MusicalValues {}

pub struct Parser<'a> {
    input: &'a str,
    cursor: usize,
    lines: usize,
    size: usize,
}

pub trait ParsingFunctions {
    fn has_next(&self) -> bool;
    fn skip_whitespace(&mut self);
    fn ident(&mut self) -> String;
    fn unsigned_int(&mut self) -> Result<usize, ParseError>;
    fn symbol(&mut self, c: char) -> bool;
    fn force_end(&mut self) -> Result<(), ParseError>;
    fn parse(&mut self) -> Result<(), ParseError>;
}

impl<'a> From<&'a str> for Parser<'a> {
    fn from(value: &'a str) -> Self {
        let input = value;
        let cursor = 0;
        let lines = 1;
        let size = input.len();
        Self {
            input,
            cursor,
            lines,
            size,
        }
    }
}

impl<'a> ParsingFunctions for Parser<'a> {
    fn has_next(&self) -> bool {
        self.cursor < self.size
    }

    fn skip_whitespace(&mut self) {
        while self.has_next() {
            match self.input.chars().nth(self.cursor) {
                Some(' ' | '\t' | '\r') => self.cursor += 1,
                Some('\n') => {
                    self.cursor += 1;
                    self.lines += 1;
                }
                Some('#') => self.skip_comment(),
                _ => break,
            }
        }
    }

    fn ident(&mut self) -> String {
        self.skip_whitespace();
        let mut out = "".to_string();
        loop {
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
        }
    }

    fn unsigned_int(&mut self) -> Result<usize, ParseError> {
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
        }
        .parse::<usize>();
        if num.is_err() {
            Err(ParseError::Int(self.lines))
        } else {
            Ok(num.unwrap())
        }
    }

    fn symbol(&mut self, c: char) -> bool {
        self.skip_whitespace();
        if let Some(m) = self.input.chars().nth(self.cursor) {
            self.cursor += 1;
            m == c
        } else {
            false
        }
    }

    fn force_end(&mut self) -> Result<(), ParseError> {
        self.skip_whitespace();
        if let Some(c) = self.input.chars().nth(self.cursor) {
            if c != '\n' && c != ';' {
                return Err(ParseError::EOL(self.lines));
            }
        }
        Ok(())
    }

    fn parse(&mut self) -> Result<(), ParseError> {
        let _ident = self.ident();
        let _dur = self.duration()?;
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

    fn duration(&mut self) -> Result<f32, ParseError> {
        self.skip_whitespace();
        dbg!(self.input.chars().nth(self.cursor));
        let numerator = self.unsigned_int()? as f32;
        let denominator = if self.symbol('/') {
            self.unsigned_int()?
        } else {
            1
        } as f32;
        Ok(numerator / denominator)
    }
    
    fn _note(&mut self) -> () {

    }
}

fn main() -> Result<(), ParseError> {
    let mut parser = Parser::from(include_str!("../test.musical"));
    let parsed = parser.parse();
    if let Err(e) = parsed {
        println!("{e}");
    }
    Ok(())
}
