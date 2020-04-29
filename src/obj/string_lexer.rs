use crate::obj::entity::Entity;
use crate::obj::line_parser::LineParser;
use std::result::Result;
use crate::error::Error;

#[derive(Default)]
pub struct StringLexer {
}

impl StringLexer {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn read_line(line: &str) -> Result<Entity, Error> {
        let mut split = line.split_whitespace();
        match split.next() {
            Some(x) => {
                LineParser::parse_line(&mut split, x, line)
            }
            None => {
                Err(Error::new("invalid line"))
            }
        }
    }
}
