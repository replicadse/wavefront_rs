//! Contains logic to read entities from OBJ formatted strings.
//!

use std::error::Error;
use crate::obj::entity::Entity;
use crate::obj::line_parser::LineParser;
use std::result::Result;

/// Will parse obj formatted string.
#[derive(Default)]
pub struct StringLexer {}

impl StringLexer {
    /// Tries to read parse an `Entity` from the given string until the first encountered linebreak.\
    /// For parsing multiple entities, consider using the `ReadLexer`.
    pub fn read_line(line: &str) -> Result<Entity, Box<dyn Error>> {
        let mut split = line.split_whitespace();
        match split.next() {
            Some(x) => LineParser::parse_line(&mut split, x, line),
            None => Err(Box::new(crate::error::GenericError::new("invalid line"))),
        }
    }
}
