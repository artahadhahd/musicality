use std::{error::Error, fmt};

#[derive(Debug)]
pub enum Modifier {
    Sharp,
    Flat,
    Normal,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Note {
    note: char,
    modifier: Modifier,
    duration: f32,
}

#[derive(Debug)]
pub enum ParseError {
    Expected(MusicalValues),
    ExpectedIdentifier,
    ExpectedNote,
}

impl Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseError::*;
        match self {
            Expected(c) => write!(f, "Expected {:?}", c),
            ExpectedIdentifier => write!(f, "Expected an identifier"),
            ExpectedNote => write!(f, "Expected a note"),
        }
    }
}

#[derive(Debug)]
pub enum MusicalValues {
    String(String),
    Fraction(u8, u8),
    Num(usize),
    Variable(String, usize),
    StrPair(String, String),
    Note(Note),
}

pub struct Parser<'a> {
    input: &'a str,
    cursor: usize,
    buffer_size: usize,
    line: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let cursor: usize = 0;
        let buffer_size = input.len();
        let line: usize = 0;
        Self {
            input,
            cursor,
            buffer_size,
            line,
        }
    }

    pub fn parse(&mut self) -> Result<(), ParseError> {
        // println!("{:?}", self.parse_identifier());
        // let ident = self.parse_identifier()?;
        // println!("{:?}", ident);
        println!("{:?}", self.parse_note());
        Ok(())
    }

    fn has_next(&self) -> bool {
        self.cursor < self.buffer_size
    }

    fn skip_whitespace(&mut self) {
        while self.has_next() {
            if let Some(c) = self.input.chars().nth(self.cursor) {
                if c == '\n' {
                    self.cursor += 1;
                    self.line += 1;
                } else if c == ' ' || c == '\t' {
                    self.cursor += 1;
                } else {
                    break;
                }
            }
        }
    }

    fn parse_identifier(&mut self) -> Result<MusicalValues, ParseError> {
        self.skip_whitespace();
        let mut string = "".to_string();
        while self.has_next() {
            if let Some(c) = self.input.chars().nth(self.cursor) {
                if c.is_alphabetic() || c == '_' {
                    string.push(c);
                } else {
                    break;
                }
                self.cursor += 1;
            }
        }
        if string.is_empty() {
            return Err(ParseError::ExpectedIdentifier);
        }
        Ok(MusicalValues::String(string))
    }

    fn parse_uint(&mut self) -> usize {
        self.skip_whitespace();
        let mut int = "".to_string();
        while self.has_next() {
            if let Some(c) = self.input.chars().nth(self.cursor) {
                if c.is_numeric() {
                    int.push(c);
                    self.cursor += 1;
                } else {
                    break;
                }
            }
        }
        let int: usize = int.parse().unwrap();
        int
    }

    fn parse_duration(&mut self) -> Result<(usize, usize), ParseError>
    {
        self.skip_whitespace();
        let nominator = self.parse_uint();
        self.skip_whitespace();
        match self.input.chars().nth(self.cursor) {
            Some('/') => {
                let denominator = self.parse_uint();
                return Ok((nominator, denominator))
            }
            None => return Ok((nominator, 1)),
            _ => Ok((nominator, 1))
        }
    }

    fn parse_note(&mut self) -> Result<MusicalValues, ParseError> {
        // self.skip_whitespace();
        let note = match self.input.chars().nth(self.cursor).unwrap_or(' ') {
            c @ ('A' | 'B' | 'H' | 'C' | 'D' | 'E' | 'F' | 'G') => Ok(c),
            _ => Err(ParseError::ExpectedNote),
            // c @ _ => Ok(c)
        }?;
        self.skip_whitespace();
        let modifier = match self.input.chars().nth(self.cursor) {
            Some('#') => Modifier::Sharp,
            Some('b') => Modifier::Flat,
            _ => Modifier::Normal,
        };
        self.skip_whitespace();
        let (nominator, denom) = self.parse_duration()?;
        let duration = nominator as f32 / denom as f32;
        Ok(MusicalValues::Note(Note {
            note,
            modifier,
            duration
        }))
    }
}

fn main() -> Result<(), ParseError> {
    let mut parser = Parser::new("A 1");
    parser.parse()?;
    Ok(())
}
