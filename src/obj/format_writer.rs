//! Contains the logic to transform entities to OBJ formatted strings.
//!

use crate::obj::entity::Entity;
use std::io::Write;

/// Will write entities to a `Write` trait.
pub struct FormatWriter {}

impl FormatWriter {
    /// Writes the given entity to the given `Write` trait as OBJ format representation of that `Entity`.
    pub fn write<W: Write>(writer: &mut W, e: &Entity) {
        match e {
            Entity::Comment { content } => {
                writer
                    .write_all(format!("{} {}", e.token(), content).as_ref())
                    .unwrap();
            }
            Entity::Face { vertices } => {
                writer.write_all(e.token().as_ref()).unwrap();
                for v in vertices {
                    writer.write_all(" ".as_ref()).unwrap();
                    writer.write_all(format!("{}", v.vertex).as_ref()).unwrap();
                    if let Some(x) = v.texture {
                        writer.write_all(format!("/{}", x).as_ref()).unwrap();
                    }
                    if let Some(x) = v.normal {
                        writer
                            .write_all(
                                format!(
                                    "{}{}",
                                    match v.texture {
                                        None => "//",
                                        Some(_) => "/",
                                    },
                                    x
                                )
                                .as_ref(),
                            )
                            .unwrap();
                    }
                }
            }
            Entity::Point { vertices } => {
                writer.write_all(e.token().as_ref()).unwrap();
                for v in vertices {
                    writer.write_all(format!(" {}", v).as_ref()).unwrap();
                }
            }
            Entity::Line { vertices } => {
                writer.write_all(e.token().as_ref()).unwrap();
                for v in vertices {
                    writer.write_all(format!(" {}", v).as_ref()).unwrap();
                }
            }
            Entity::Group { name } => {
                writer
                    .write_all(format!("{} {}", e.token(), name).as_ref())
                    .unwrap();
            }
            Entity::Mtllib { name } => {
                writer
                    .write_all(format!("{} {}", e.token(), name).as_ref())
                    .unwrap();
            }
            Entity::Object { name } => {
                writer
                    .write_all(format!("{} {}", e.token(), name).as_ref())
                    .unwrap();
            }
            Entity::SmoothingGroup { name } => {
                writer
                    .write_all(format!("{} {}", e.token(), name).as_ref())
                    .unwrap();
            }
            Entity::MergingGroup { name } => {
                writer
                    .write_all(format!("{} {}", e.token(), name).as_ref())
                    .unwrap();
            }
            Entity::Usemtl { name } => {
                writer
                    .write_all(format!("{} {}", e.token(), name).as_ref())
                    .unwrap();
            }
            Entity::Vertex { x, y, z, w } => {
                writer
                    .write_all(format!("{} {} {} {}", e.token(), x, y, z).as_ref())
                    .unwrap();
                if let Some(v) = w {
                    writer.write_all(format!(" {}", v).as_ref()).unwrap();
                }
            }
            Entity::VertexNormal { x, y, z } => {
                writer
                    .write_all(format!("{} {} {} {}", e.token(), x, y, z).as_ref())
                    .unwrap();
            }
            Entity::VertexTexture { u, v, w } => {
                writer
                    .write_all(format!("{} {}", e.token(), u).as_ref())
                    .unwrap();
                if let Some(v) = v {
                    writer.write_all(format!(" {}", v).as_ref()).unwrap();
                    if let Some(w) = w {
                        writer.write_all(format!(" {}", w).as_ref()).unwrap();
                    }
                }
            }
            Entity::VertexParameter { u, v, w } => {
                writer
                    .write_all(format!("{} {}", e.token(), u).as_ref())
                    .unwrap();
                if let Some(v) = v {
                    writer.write_all(format!(" {}", v).as_ref()).unwrap();
                    if let Some(w) = w {
                        writer.write_all(format!(" {}", w).as_ref()).unwrap();
                    }
                }
            }
        }
    }
}
