//! Contains logic to read entities from a `BufRead` that returns OBJ formatted strings.
//!

use crate::obj::entity::Entity;
use crate::obj::line_parser::LineParser;
use std::error::Error;
use std::io::BufRead;
use std::result::Result;

/// Will read from a given `BufRead` and parse entities.
pub struct Parser {}

impl Parser {
    /// Will read from the given `BufRead`as long as it is not EOF.\
    /// When an entity is parsed, the given callback is invoked and the entity is inserted into it as parameter.\
    /// Will return `Ok(())` if successful or an `Error` (if parsing failed).
    pub fn read_to_end<R: BufRead>(
        reader: &mut R,
        mut callback: impl FnMut(Entity),
    ) -> Result<(), Box<dyn Error>> {
        for l in reader.lines() {
            let s: String = l?;
            let mut split = s.split_whitespace();
            if let Some(x) = split.next() {
                match LineParser::parse_line(&mut split, x, s.as_ref()) {
                    Ok(x) => callback(x),
                    Err(x) => return Err(x),
                }
            }
        }
        Ok(())
    }

    /// Will read from the given `BufRead` until the first encountered linebreak.\
    /// Will return `Ok(Entity)` if successful or an `Error` (if parsing failed).
    pub fn read_line<R: BufRead>(reader: &mut R) -> Result<Entity, Box<dyn Error>> {
        let value = &mut String::new();
        match reader.read_line(value) {
            Ok(x) => {
                if x > 0 {
                    let mut split = value.split_whitespace();
                    match split.next() {
                        Some(x) => LineParser::parse_line(&mut split, x, value.as_ref()),
                        None => Err(Box::new(crate::error::GenericError::new("invalid line"))),
                    }
                } else {
                    Err(Box::new(crate::error::GenericError::new("reached EOF")))
                }
            }
            Err(_) => Err(Box::new(crate::error::GenericError::new(
                "error reading line",
            ))),
        }
    }
}
