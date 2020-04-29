use crate::obj::entity::Entity;
use crate::obj::line_parser::LineParser;
use std::result::Result;
use std::io::BufRead;
use crate::error::Error;

#[derive(Default)]
pub struct ReadLexer {
}

impl ReadLexer {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn read_to_end<R: BufRead>(reader: &mut R, callback: impl Fn(Entity)) -> Result<(), Error> {
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

    pub fn read_line<R: BufRead>(reader: &mut R) -> Result<Entity, Error> {
        let value = &mut String::new();
        match reader.read_line(value) {
            Ok(x) => {
                if x > 0 {
                    let mut split = value.split_whitespace();
                    match split.next() {
                        Some(x) => {
                            LineParser::parse_line(&mut split, x, value.as_ref())
                        }
                        None => {
                            Err(Error::new("invalid line"))
                        }
                    }
                } else {
                    Err(Error::new("reached EOF"))
                }
            }
            Err(x) => Err(Error::from(x)),
        }
    }
}
