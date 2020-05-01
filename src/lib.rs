//! [`wavefront_rs`] is a RUST implementation for parsing and writing the wavefront obj and mtl format.
//!
//! [`wavefront_rs`]: https://github.com/replicadse/wavefront_rs
//!

#![crate_type = "lib"]

pub mod error;

pub mod obj {
    //! The obj module contains all the types that are used when working with the [`wavefront obj`]
    //! format including entities, lexer, writer etc.
    //!
    //! [`wavefront obj`]: https://en.wikipedia.org/wiki/Wavefront_.obj_file
    //!

    pub mod entity;
    pub mod format_writer;
    pub mod read_lexer;
    pub mod string_lexer;

    mod line_parser;
}
