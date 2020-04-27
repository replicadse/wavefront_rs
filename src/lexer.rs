use crate::entity::Entity;
use crate::line_parser::LineParser;
use std::result::Result;
use std::io::BufRead;
use crate::error::ParseError;

pub trait Lexer {
    fn read_to_end<R: BufRead>(&self, r: &mut R, callback: impl Fn(Entity)) -> Result<(), ParseError>;
    fn read_line<R: BufRead>(&self, r: &mut R) -> Result<Entity, ParseError>;
}

#[derive(Default)]
pub struct ReadLexer {
}

impl ReadLexer {
    pub fn new() -> ReadLexer {
        ReadLexer {
        }
    }
}

impl Lexer for ReadLexer {
    fn read_to_end<R: BufRead>(&self, reader: &mut R, callback: impl Fn(Entity)) -> Result<(), ParseError> {
        let parser = LineParser{};
        for l in reader.lines() {
            let s: String = l?;
            let mut split = s.split_whitespace();
            if let Some(x) = split.next() {
                match parser.parse_line(&mut split, x, s.as_ref()) {
                    Ok(x) => callback(x),
                    Err(x) => return Err(x),
                }
            }
        }
        Ok(())
    }

    fn read_line<R: BufRead>(&self, reader: &mut R) -> Result<Entity, ParseError> {
        let parser = LineParser{};
        let value = &mut String::new();
        match reader.read_line(value) {
            Ok(x) => {
                if x > 0 {
                    let mut split = value.split_whitespace();
                    match split.next() {
                        Some(x) => {
                            parser.parse_line(&mut split, x, value.as_ref())
                        }
                        None => {
                            Err(ParseError::new("invalid line"))
                        }
                    }
                } else {
                    Err(ParseError::new("reached EOF"))
                }
            }
            Err(x) => Err(ParseError::from(x)),
        }
    }
}
