#[derive(Debug)]
pub struct Error {
    details: String
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error{details: msg.to_string()}
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::new(&e.to_string())
    }
}
