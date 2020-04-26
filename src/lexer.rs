use crate::entity::Entity;
use crate::line_parser::LineParser;
use std::result::Result;
use std::io::BufRead;
use crate::error::LexerError;

pub trait Lexer {
    fn read<R: BufRead>(&self, r: &mut R, callback: impl Fn(Entity)) -> Result<(), LexerError>;
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
    fn read<R: BufRead>(&self, reader: &mut R, mut callback: impl FnMut(Entity)) -> Result<(), LexerError> {
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
}
