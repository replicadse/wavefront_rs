#[derive(Debug)]
pub struct ParseError {
    details: String
}

impl ParseError {
    pub fn new(msg: &str) -> ParseError {
        ParseError{details: msg.to_string()}
    }
}

impl std::convert::From<std::io::Error> for ParseError {
    fn from(e: std::io::Error) -> ParseError {
        ParseError::new(&e.to_string())
    }
}
