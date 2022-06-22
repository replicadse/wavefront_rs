//! Contains the entity types that are used when working with the [`wavefront obj`] format.
//!
//! [`wavefront obj`]: https://en.wikipedia.org/wiki/Wavefront_.obj_file
//!

use crate::obj::parser::*;
use crate::obj::writer::*;
use std::io::{BufReader, BufWriter, Cursor};

pub type Format = String;

/// Contains all possible entities that can exist in an OBJ format.
#[derive(Debug, Clone, PartialEq)]
pub enum Entity {
    Comment {
        content: String,
    },
    Object {
        name: String,
    },
    Group {
        name: String,
    },
    SmoothingGroup {
        name: String,
    },
    MergingGroup {
        name: String,
    },
    MtlLib {
        name: String,
    },
    UseMtl {
        name: String,
    },
    /// Vertex consists of `x`, `y`, `z` and `w` whereas `w` is optional.\
    /// Example xyzw: `v 0.1 1.2 2.3 3.4`\
    /// Example xyz: `v 0.1 1.2 2.3`
    Vertex {
        x: f64,
        y: f64,
        z: f64,
        w: Option<f64>,
    },
    /// VertexNormal consists of `x`, `y`and `z`. The normal will usually but is not required to be a unit vector.\
    /// Example: `vn 0.1 1.2 2.3`
    VertexNormal {
        x: f64,
        y: f64,
        z: f64,
    },
    /// VertexTexture consists of `u`, `v` and `w` whereas `u` and `w` are optional.\
    /// Example uvw: `vt 0.1 1.2 2.3`\
    /// Example uv: `vt 0.1 1.2`\
    /// Example u: `vt 0.1`
    VertexTexture {
        u: f64,
        v: Option<f64>,
        w: Option<f64>,
    },
    /// VertexParameter consists of `u`, `v` and `w` whereas `v` and `w` are optional.\
    /// `u` is a 1D control point in the parameter space of a curve.
    /// `u + v` is a 2D control point in the parameter space of a surface.\
    /// Control poins for non rational trimming curves require `u + v`.\
    /// Control points for rational trimming curves require `u + v + w`.\
    /// `w` defaults to 1.0 if not set.\
    /// Example uvw: `vp 0.1 1.2 2.3`\
    /// Example uv: `vp 0.1 1.2`\
    /// Example u: `vp 0.1`
    VertexParameter {
        u: f64,
        v: Option<f64>,
        w: Option<f64>,
    },
    /// The point entity contains a list of points in space.\
    /// Example (3 points): `p 0 1 2`\
    /// Example (1 point): `p 0`
    Point {
        vertices: Vec<i64>,
    },
    /// Line consists of an arbitrary number (whereas n >= 2) of vertices that describe the path.
    Line {
        vertices: Vec<i64>,
    },
    /// Face consists of an arbitrary number (whereas n >= 3) of complex vertices that describe the polygon.\
    /// Example (vertex): `f 0 3 6`\
    /// Example (vertex+texture+normal): `f 0/1/2 3/4/5 6/7/8`\
    /// Example (vertex+texture): `f 0/1 3/4 6/7`
    /// Example (vertex+normal): `f 0//2 3//5 6//8`\
    Face {
        vertices: Vec<FaceVertex>,
    },
}

impl Entity {
    pub fn token(&self) -> &str {
        match self {
            Self::Comment { .. } => "#",
            Self::Object { .. } => "o",
            Self::Group { .. } => "g",
            Self::SmoothingGroup { .. } => "s",
            Self::MergingGroup { .. } => "mg",
            Self::MtlLib { .. } => "mtllib",
            Self::UseMtl { .. } => "usemtl",
            Self::Vertex { .. } => "v",
            Self::VertexNormal { .. } => "vn",
            Self::VertexTexture { .. } => "vt",
            Self::VertexParameter { .. } => "vp",
            Self::Point { .. } => "p",
            Self::Line { .. } => "l",
            Self::Face { .. } => "f",
        }
    }
}

/// Describes a vertex in a face.
#[derive(Debug, Clone, PartialEq)]
pub struct FaceVertex {
    /// The vertex index itself.
    pub vertex: i64,
    /// The texture map info for the vertex (optional).
    pub texture: Option<i64>,
    /// The normal of the vertex (optional).
    pub normal: Option<i64>,
}

impl FaceVertex {
    pub fn new(vertex: i64) -> Self {
        Self {
            vertex,
            texture: None,
            normal: None,
        }
    }

    pub fn new_vtn(vertex: i64, texture: Option<i64>, normal: Option<i64>) -> Self {
        Self {
            vertex,
            texture,
            normal,
        }
    }
}

impl ToString for Entity {
    fn to_string(&self) -> String {
        let mut result = String::new();
        Writer::write(&mut BufWriter::new(unsafe { result.as_mut_vec() }), self).unwrap();
        result
    }
}

impl From<Format> for Entity {
    fn from(input: Format) -> Self {
        Parser::parse_line(&mut BufReader::new(Cursor::new(input))).unwrap()
    }
}

impl From<Entity> for String {
    fn from(e: Entity) -> Self {
        e.to_string()
    }
}
