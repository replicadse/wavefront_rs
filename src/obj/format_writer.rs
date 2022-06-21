//! Contains the logic to transform entities to OBJ formatted strings.
//!

use std::error::Error;
use crate::obj::entity::Entity;
use std::io::Write;

/// Will write entities to a `Write` trait.
pub struct FormatWriter {}

impl FormatWriter {
    /// Writes the given entity to the given `Write` trait as OBJ format representation of that `Entity`.
    pub fn write<W: Write>(writer: &mut W, e: &Entity) -> std::result::Result<(), Box<dyn Error>> {
        match e {
            Entity::Comment { content } => {
                writer
                    .write_all(format!("{} {}", e.token(), content).as_ref())?;
            }
            Entity::Face { vertices } => {
                writer.write_all(e.token().as_ref())?;
                for v in vertices {
                    writer.write_all(" ".as_ref())?;
                    writer.write_all(format!("{}", v.vertex).as_ref())?;
                    if let Some(x) = v.texture {
                        writer.write_all(format!("/{}", x).as_ref())?;
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
                            )?;
                    }
                }
            }
            Entity::Point { vertices } => {
                writer.write_all(e.token().as_ref())?;
                for v in vertices {
                    writer.write_all(format!(" {}", v).as_ref())?;
                }
            }
            Entity::Line { vertices } => {
                writer.write_all(e.token().as_ref())?;
                for v in vertices {
                    writer.write_all(format!(" {}", v).as_ref())?;
                }
            }
            Entity::Group { name } => {
                writer
                    .write_all(format!("{} {}", e.token(), name).as_ref())?;
            }
            Entity::Mtllib { name } => {
                writer
                    .write_all(format!("{} {}", e.token(), name).as_ref())?;
            }
            Entity::Object { name } => {
                writer
                    .write_all(format!("{} {}", e.token(), name).as_ref())?;
            }
            Entity::SmoothingGroup { name } => {
                writer
                    .write_all(format!("{} {}", e.token(), name).as_ref())?;
            }
            Entity::MergingGroup { name } => {
                writer
                    .write_all(format!("{} {}", e.token(), name).as_ref())?;
            }
            Entity::Usemtl { name } => {
                writer
                    .write_all(format!("{} {}", e.token(), name).as_ref())?;
            }
            Entity::Vertex { x, y, z, w } => {
                writer
                    .write_all(format!("{} {} {} {}", e.token(), x, y, z).as_ref())?;
                if let Some(v) = w {
                    writer.write_all(format!(" {}", v).as_ref())?;
                }
            }
            Entity::VertexNormal { x, y, z } => {
                writer
                    .write_all(format!("{} {} {} {}", e.token(), x, y, z).as_ref())?;
            }
            Entity::VertexTexture { u, v, w } => {
                writer
                    .write_all(format!("{} {}", e.token(), u).as_ref())?;
                if let Some(v) = v {
                    writer.write_all(format!(" {}", v).as_ref())?;
                    if let Some(w) = w {
                       writer.write_all(format!(" {}", w).as_ref())?;
                    }
                }
            }
            Entity::VertexParameter { u, v, w } => {
                writer
                    .write_all(format!("{} {}", e.token(), u).as_ref())?;
                if let Some(v) = v {
                    writer.write_all(format!(" {}", v).as_ref())?;
                    if let Some(w) = w {
                        writer.write_all(format!(" {}", w).as_ref())?;
                    }
                }
            }
        }
        Ok(())
    }
}
