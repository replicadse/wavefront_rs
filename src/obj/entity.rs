use std::io::{Cursor, BufReader, BufWriter};
use crate::obj::read_lexer::*;
use crate::obj::format_writer::*;

pub type Format = String;

#[derive(Debug, PartialEq)]
pub enum Entity {
    Comment{content: String},
    Object{name: String},
    Group{name: String},
    SmoothingGroup{name: String},
    Mtllib{name: String},
    Usemtl{name: String},
    Vertex{x: f64, y: f64, z: f64, w: Option<f64>},
    VertexNormal{x: f64, y: f64, z: f64},
    VertexTexture{x: f64, y: f64, z: Option<f64>},
    Face{vertices: Vec<FaceVertex>},
    Line{vertices: Vec<i64>},
}

impl Entity {
    pub fn token(&self) -> &str {
        match self {
            Self::Comment{..} => "#",
            Self::Object{..} => "o",
            Self::Group{..} => "g",
            Self::SmoothingGroup{..} => "s",
            Self::Mtllib{..} => "mtllib",
            Self::Usemtl{..} => "usemtl",
            Self::Vertex{..} => "v",
            Self::VertexNormal{..} => "vn",
            Self::VertexTexture{..} => "vt",
            Self::Face{..} => "f",
            Self::Line{..} => "l",
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FaceVertex {
    pub vertex: i64,
    pub normal: Option<i64>,
    pub texture: Option<i64>,
}

impl FaceVertex {
    pub fn new(vertex: i64) -> Self {
        Self {
            vertex,
            normal: None,
            texture: None,
        }
    }

    pub fn new2(vertex: i64, normal: Option<i64>, texture: Option<i64>) -> Self {
        Self {
            vertex,
            normal,
            texture,
        }
    }
}

impl ToString for Entity {
    fn to_string(&self) -> String {
        let mut result = String::new();
        FormatWriter::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), &self);
        result
    }
}

impl From<Format> for Entity {
    fn from(input: Format) -> Self {
        ReadLexer::read_line(&mut BufReader::new(Cursor::new(input))).unwrap()
    }
}

impl Into<Format> for Entity {
    fn into(self) -> String {
        self.to_string()
    }
}
