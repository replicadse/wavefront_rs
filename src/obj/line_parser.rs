use crate::error::Error;
use crate::obj::entity::{Entity, FaceVertex};

pub struct LineParser {}

impl LineParser {
    pub fn parse_line(
        split: &mut std::str::SplitWhitespace,
        token: &str,
        line: &str,
    ) -> Result<Entity, Error> {
        match token.to_lowercase().as_str() {
            "#" => Ok(Entity::Comment {
                content: line.trim_start_matches("# ").to_owned(),
            }),
            "o" => Ok(Entity::Object {
                name: line.trim_start_matches("o ").to_owned(),
            }),
            "g" => Ok(Entity::Group {
                name: line.trim_start_matches("g ").to_owned(),
            }),
            "s" => Ok(Entity::SmoothingGroup {
                name: line.trim_start_matches("s ").to_owned(),
            }),
            "mg" => Ok(Entity::MergingGroup {
                name: line.trim_start_matches("mg ").to_owned(),
            }),
            "v" => Self::parse_v(split),
            "vn" => Self::parse_vn(split),
            "vt" => Self::parse_vt_vp(true, split),
            "vp" => Self::parse_vt_vp(false, split),
            "f" => Self::parse_face(split),
            "l" => Self::parse_polyline(split),
            "p" => Self::parse_point(split),
            "mtllib" => {
                if let Some(x) = split.next() {
                    Ok(Entity::Mtllib { name: x.to_owned() })
                } else {
                    Err(Error::new(
                        format!("could not parse line \"{}\"", token).as_ref(),
                    ))
                }
            }
            "usemtl" => {
                if let Some(x) = split.next() {
                    Ok(Entity::Usemtl { name: x.to_owned() })
                } else {
                    Err(Error::new(
                        format!("could not parse line \"{}\"", token).as_ref(),
                    ))
                }
            }
            _ => Err(Error::new(format!("unknown token \"{}\"", token).as_ref())),
        }
    }

    fn parse_v(split: &mut std::str::SplitWhitespace) -> Result<Entity, Error> {
        let xs = split.next();
        let ys = split.next();
        let zs = split.next();
        let ws = split.next();
        if xs == None || ys == None || zs == None {
            return Err(Error::new("invalid data for v"));
        }
        let x = xs.unwrap().parse::<f64>();
        let y = ys.unwrap().parse::<f64>();
        let z = zs.unwrap().parse::<f64>();
        let w = match ws {
            Some(v) => match v.parse::<f64>() {
                Ok(v) => Some(v),
                Err(_) => return Err(Error::new("invalid data for v")),
            },
            None => None,
        };
        if x.is_err() || y.is_err() || z.is_err() {
            return Err(Error::new("invalid data for v"));
        }
        Ok(Entity::Vertex {
            x: x.unwrap(),
            y: y.unwrap(),
            z: z.unwrap(),
            w,
        })
    }

    fn parse_vt_vp(is_vt: bool, split: &mut std::str::SplitWhitespace) -> Result<Entity, Error> {
        let us = split.next();
        let vs = split.next();
        let ws = split.next();
        if us == None {
            return Err(Error::new("invalid data for vt"));
        }
        let u = us.unwrap().parse::<f64>();
        let v = match vs {
            Some(v) => match v.parse::<f64>() {
                Ok(v) => Some(v),
                Err(_) => return Err(Error::new("invalid data for vt")),
            },
            None => None,
        };
        let w = match ws {
            Some(v) => match v.parse::<f64>() {
                Ok(v) => Some(v),
                Err(_) => return Err(Error::new("invalid data for vt")),
            },
            None => None,
        };
        if u.is_err() {
            return Err(Error::new("invalid data for vt"));
        }

        if is_vt {
            Ok(Entity::VertexTexture {
                u: u.unwrap(),
                v,
                w,
            })
        } else {
            Ok(Entity::VertexParameter {
                u: u.unwrap(),
                v,
                w,
            })
        }
    }

    fn parse_vn(split: &mut std::str::SplitWhitespace) -> Result<Entity, Error> {
        let xs = split.next();
        let ys = split.next();
        let zs = split.next();
        if xs == None || ys == None || zs == None {
            return Err(Error::new("invalid data for vn"));
        }
        let x = xs.unwrap().parse::<f64>();
        let y = ys.unwrap().parse::<f64>();
        let z = zs.unwrap().parse::<f64>();
        if x.is_err() || y.is_err() || z.is_err() {
            return Err(Error::new("invalid data for vn"));
        }
        Ok(Entity::VertexNormal {
            x: x.unwrap(),
            y: y.unwrap(),
            z: z.unwrap(),
        })
    }

    fn parse_face(split: &mut std::str::SplitWhitespace) -> Result<Entity, Error> {
        let mut face = Vec::new();
        for vnt in split {
            let mut vnts = vnt.split('/');
            if let Some(v) = vnts.next() {
                let v_parsed = v.parse::<i64>().unwrap();
                let mut vertex = FaceVertex::new(v_parsed);
                if let Some(vn) = vnts.next() {
                    if vn != "" {
                        vertex.normal = Some(vn.parse::<i64>().unwrap());
                    }
                }
                if let Some(vt) = vnts.next() {
                    if vt != "" {
                        vertex.texture = Some(vt.parse::<i64>().unwrap());
                    }
                }
                face.push(vertex);
            } else {
                return Err(Error::new("could not parse face"));
            }
        }
        Ok(Entity::Face { vertices: face })
    }

    fn parse_polyline(split: &mut std::str::SplitWhitespace) -> Result<Entity, Error> {
        let mut vertices = Vec::new();
        for x in split {
            vertices.push(x.parse::<i64>().unwrap())
        }
        Ok(Entity::Line { vertices })
    }

    fn parse_point(split: &mut std::str::SplitWhitespace) -> Result<Entity, Error> {
        let mut vertices = Vec::new();
        for x in split {
            vertices.push(x.parse::<i64>().unwrap())
        }
        Ok(Entity::Point { vertices })
    }
}
