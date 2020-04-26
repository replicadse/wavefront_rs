#[derive(Debug)]
pub struct LexerError {
    details: String
}

impl LexerError {
    pub fn new(msg: &str) -> LexerError {
        LexerError{details: msg.to_string()}
    }
}

impl std::convert::From<std::io::Error> for LexerError {
    fn from(e: std::io::Error) -> LexerError {
        LexerError::new(&e.to_string())
    }
}
