//! Contains logic to read entities from a `BufRead` that returns OBJ formatted
//! strings.

use std::{
    io::BufRead,
    result::Result,
};

use crate::obj::entity::{
    Entity,
    FaceVertex,
};

/// Will read from a given `BufRead` and parse entities.
pub struct Parser {}

impl Parser {
    /// Will read from the given `BufRead`as long as it is not EOF.\
    /// When an entity is parsed, the given callback is invoked and the entity
    /// is inserted into it as parameter.\ Will return `Ok(())` if
    /// successful or an `Error` (if parsing failed).
    pub fn read_to_end<R: BufRead>(
        reader: &mut R,
        mut callback: impl FnMut(Entity),
    ) -> Result<(), Box<dyn std::error::Error>> {
        for l in reader.lines() {
            let s: String = l?;
            let mut split = s.split_whitespace();
            if let Some(x) = split.next() {
                match Self::parse_split(&mut split, x, s.as_ref()) {
                    | Ok(x) => callback(x),
                    | Err(x) => return Err(x),
                }
            }
        }
        Ok(())
    }

    /// Will read from the given `BufRead` until the first encountered
    /// linebreak.\ Will return `Ok(Entity)` if successful or an `Error` (if
    /// parsing failed).
    pub fn parse_line<R: BufRead>(reader: &mut R) -> Result<Entity, Box<dyn std::error::Error>> {
        let value = &mut String::new();
        match reader.read_line(value) {
            | Ok(x) => {
                if x > 0 {
                    let mut split = value.split_whitespace();
                    match split.next() {
                        | Some(x) => Self::parse_split(&mut split, x, value.as_ref()),
                        | None => Err(Box::new(crate::error::ParserError::new("invalid line"))),
                    }
                } else {
                    Err(Box::new(crate::error::ParserError::new("reached EOF")))
                }
            },
            | Err(..) => Err(Box::new(crate::error::ParserError::new("error reading line"))),
        }
    }

    fn parse_split(
        split: &mut std::str::SplitWhitespace,
        token: &str,
        line: &str,
    ) -> std::result::Result<Entity, Box<dyn std::error::Error>> {
        match token.to_lowercase().as_str() {
            | "#" => Ok(Entity::Comment {
                content: line.trim_start_matches("# ").to_owned(),
            }),
            | "o" => Ok(Entity::Object {
                name: line.trim_start_matches("o ").to_owned(),
            }),
            | "g" => Ok(Entity::Group {
                name: line.trim_start_matches("g ").to_owned(),
            }),
            | "s" => Ok(Entity::SmoothingGroup {
                name: line.trim_start_matches("s ").to_owned(),
            }),
            | "mg" => Ok(Entity::MergingGroup {
                name: line.trim_start_matches("mg ").to_owned(),
            }),
            | "v" => Self::parse_v(split),
            | "vn" => Self::parse_vn(split),
            | "vt" => Self::parse_vt_vp(true, split),
            | "vp" => Self::parse_vt_vp(false, split),
            | "f" => Self::parse_face(split),
            | "l" => Self::parse_polyline(split),
            | "p" => Self::parse_point(split),
            | "mtllib" => Ok(Entity::MtlLib {
                name: line.trim_start_matches("mtllib ").to_owned(),
            }),
            | "usemtl" => Ok(Entity::UseMtl {
                name: line.trim_start_matches("usemtl ").to_owned(),
            }),
            | _ => Err(Box::new(crate::error::ParserError::new(
                format!("unknown token \"{}\"", token).as_ref(),
            ))),
        }
    }

    fn parse_v(split: &mut std::str::SplitWhitespace) -> std::result::Result<Entity, Box<dyn std::error::Error>> {
        let xs = split.next();
        let ys = split.next();
        let zs = split.next();
        let ws = split.next();
        if xs == None || ys == None || zs == None {
            return Err(Box::new(crate::error::ParserError::new("invalid data for v")));
        }
        let x = xs.unwrap().parse::<f64>();
        let y = ys.unwrap().parse::<f64>();
        let z = zs.unwrap().parse::<f64>();

        let w = match ws {
            | Some(v) => match v.parse::<f64>() {
                | Ok(v) => Some(v),
                | Err(..) => return Err(Box::new(crate::error::ParserError::new("invalid data for v"))),
            },
            | None => None,
        };
        if x.is_err() || y.is_err() || z.is_err() {
            return Err(Box::new(crate::error::ParserError::new("invalid data for v")));
        }
        Ok(Entity::Vertex { x: x?, y: y?, z: z?, w })
    }

    fn parse_vt_vp(
        is_vt: bool,
        split: &mut std::str::SplitWhitespace,
    ) -> std::result::Result<Entity, Box<dyn std::error::Error>> {
        let us = split.next();
        let vs = split.next();
        let ws = split.next();
        if us == None {
            return Err(Box::new(crate::error::ParserError::new(
                format!("invalid data for {}", if is_vt { "vt" } else { "vp" }).as_ref(),
            )));
        }
        let u = us.unwrap().parse::<f64>();
        let v = match vs {
            | Some(v) => match v.parse::<f64>() {
                | Ok(v) => Some(v),
                | Err(..) => {
                    return Err(Box::new(crate::error::ParserError::new(
                        format!("invalid data for {}", if is_vt { "vt" } else { "vp" }).as_ref(),
                    )))
                },
            },
            | None => None,
        };
        let w = match ws {
            | Some(v) => match v.parse::<f64>() {
                | Ok(v) => Some(v),
                | Err(..) => {
                    return Err(Box::new(crate::error::ParserError::new(
                        format!("invalid data for {}", if is_vt { "vt" } else { "vp" }).as_ref(),
                    )))
                },
            },
            | None => None,
        };
        if u.is_err() {
            return Err(Box::new(crate::error::ParserError::new(
                format!("invalid data for {}", if is_vt { "vt" } else { "vp" }).as_ref(),
            )));
        }

        if is_vt {
            Ok(Entity::VertexTexture { u: u?, v, w })
        } else {
            Ok(Entity::VertexParameter { u: u?, v, w })
        }
    }

    fn parse_vn(split: &mut std::str::SplitWhitespace) -> std::result::Result<Entity, Box<dyn std::error::Error>> {
        let xs = split.next();
        let ys = split.next();
        let zs = split.next();
        if xs == None || ys == None || zs == None {
            return Err(Box::new(crate::error::ParserError::new("invalid data for vn")));
        }
        let x = xs.unwrap().parse::<f64>();
        let y = ys.unwrap().parse::<f64>();
        let z = zs.unwrap().parse::<f64>();
        if x.is_err() || y.is_err() || z.is_err() {
            return Err(Box::new(crate::error::ParserError::new("invalid data for vn")));
        }
        Ok(Entity::VertexNormal { x: x?, y: y?, z: z? })
    }

    fn parse_face(split: &mut std::str::SplitWhitespace) -> std::result::Result<Entity, Box<dyn std::error::Error>> {
        let mut face = Vec::new();
        for vtn in split {
            let mut vtns = vtn.split('/');
            if let Some(v) = vtns.next() {
                let v_parsed = v.parse::<i64>()?;
                let mut vertex = FaceVertex::new(v_parsed);
                if let Some(vt) = vtns.next() {
                    if !vt.is_empty() {
                        vertex.texture = Some(vt.parse::<i64>()?);
                    }
                }
                if let Some(vn) = vtns.next() {
                    if !vn.is_empty() {
                        vertex.normal = Some(vn.parse::<i64>()?);
                    }
                }
                face.push(vertex);
            } else {
                return Err(Box::new(crate::error::ParserError::new("could not parse face")));
            }
        }
        Ok(Entity::Face { vertices: face })
    }

    fn parse_polyline(
        split: &mut std::str::SplitWhitespace,
    ) -> std::result::Result<Entity, Box<dyn std::error::Error>> {
        let mut vertices = Vec::new();
        for x in split {
            vertices.push(x.parse::<i64>()?)
        }
        Ok(Entity::Line { vertices })
    }

    fn parse_point(split: &mut std::str::SplitWhitespace) -> std::result::Result<Entity, Box<dyn std::error::Error>> {
        let mut vertices = Vec::new();
        for x in split {
            vertices.push(x.parse::<i64>()?)
        }
        Ok(Entity::Point { vertices })
    }
}
