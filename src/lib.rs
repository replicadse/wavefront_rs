#![crate_type = "lib"]

pub mod error;

pub mod obj {
    pub mod entity;
    pub mod read_lexer;
    pub mod string_lexer;
    pub mod format_writer;

    mod line_parser;
}
