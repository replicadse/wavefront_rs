//! Contains the logic to transform entities to OBJ formatted strings.
//! 

use crate::obj::entity::Entity;
use std::io::Write;

/// Will write entities to a `Write` trait.
pub struct FormatWriter {
}

impl FormatWriter {
    /// Writes the given entity to the given `Write` trait as OBJ format representation of that `Entity`.
    pub fn write<W: Write>(writer: &mut W, e: &Entity) {
        match e {
            Entity::Comment{content} => {
                writer.write_all(format!("# {}", content).as_ref()).unwrap();
            }
            Entity::Face{vertices} => {
                writer.write_all("f".as_ref()).unwrap();
                for v in vertices {
                    writer.write_all(" ".as_ref()).unwrap();
                    writer.write_all(format!("{}", v.vertex).as_ref()).unwrap();
                    if let Some(x) = v.normal {
                        writer.write_all(format!("/{}", x).as_ref()).unwrap();
                    }
                    if let Some(x) = v.texture {
                        if v.normal.is_none() {
                            writer.write_all("/".as_ref()).unwrap();
                        }
                        writer.write_all(format!("/{}", x).as_ref()).unwrap();
                    }
                }
            }
            Entity::Line{vertices} => {
                writer.write_all("l".as_ref()).as_ref().unwrap();
                for v in vertices {
                    writer.write_all(format!(" {}", v).as_ref()).unwrap();
                }
            }
            Entity::Group{name} => {
                writer.write_all(format!("g {}", name).as_ref()).unwrap();
            }
            Entity::Mtllib{name} => {
                writer.write_all(format!("mtllib {}", name).as_ref()).unwrap();
            }
            Entity::Object{name} => {
                writer.write_all(format!("o {}", name).as_ref()).unwrap();
            }
            Entity::SmoothingGroup{name} => {
                writer.write_all(format!("s {}", name).as_ref()).unwrap();
            }
            Entity::Usemtl{name} => {
                writer.write_all(format!("usemtl {}", name).as_ref()).unwrap();
            }
            Entity::Vertex{x, y, z, w} => {
                writer.write_all(format!("v {} {} {}", x, y, z).as_ref()).unwrap();
                if let Some(v) = w {
                    writer.write_all(format!(" {}", v).as_ref()).unwrap();
                }
            }
            Entity::VertexNormal{x, y, z} => {
                writer.write_all(format!("vn {} {} {}", x, y, z).as_ref()).unwrap();
            }
            Entity::VertexTexture{x, y, z} => {
                writer.write_all(format!("vt {} {}", x, y).as_ref()).unwrap();
                if let Some(v) = z {
                    writer.write_all(format!(" {}", v).as_ref()).unwrap();
                }
            }
        }
    }
}
