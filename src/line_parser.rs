use crate::entity::{Entity, FaceVertex};
use crate::error::ParseError;

pub struct LineParser {
}

impl LineParser {
    fn parse_v(&self, split: &mut std::str::SplitWhitespace) -> Result<Entity, ParseError> {
        let xs = split.next();
        let ys = split.next();
        let zs = split.next();
        let ws = split.next();
        if xs == None || ys == None || zs == None {
            return Err(ParseError::new("invalid data for v"))
        }
        let x = xs.unwrap().parse::<f64>();
        let y = ys.unwrap().parse::<f64>();
        let z = zs.unwrap().parse::<f64>();
        let w = match ws {
            Some(v) => match v.parse::<f64>() {
                Ok(v) => Some(v),
                Err(_) => return Err(ParseError::new("invalid data for v"))
            }
            None => None,
        };
        if x.is_err() || y.is_err() || z.is_err() {
            return Err(ParseError::new("invalid data for v"))
        }
        Ok(Entity::Vertex{x: x.unwrap(), y: y.unwrap(), z: z.unwrap(), w})
    }

    fn parse_vt(&self, split: &mut std::str::SplitWhitespace) -> Result<Entity, ParseError> {
        let xs = split.next();
        let ys = split.next();
        let zs = split.next();
        if xs == None || ys == None {
            return Err(ParseError::new("invalid data for vt"))
        }
        let x = xs.unwrap().parse::<f64>();
        let y = ys.unwrap().parse::<f64>();
        let z = match zs {
            Some(v) => match v.parse::<f64>() {
                Ok(v) => Some(v),
                Err(_) => return Err(ParseError::new("invalid data for vt"))
            }
            None => None,
        };
        if x.is_err() || y.is_err() {
            return Err(ParseError::new("invalid data for vt"))
        }
        Ok(Entity::VertexTexture{x: x.unwrap(), y: y.unwrap(), z})
    }

    fn parse_vn(&self, split: &mut std::str::SplitWhitespace) -> Result<Entity, ParseError> {
        let xs = split.next();
        let ys = split.next();
        let zs = split.next();
        if xs == None || ys == None || zs == None {
            return Err(ParseError::new("invalid data for vn"))
        }
        let x = xs.unwrap().parse::<f64>();
        let y = ys.unwrap().parse::<f64>();
        let z = zs.unwrap().parse::<f64>();
        if x.is_err() || y.is_err() || z.is_err() {
            return Err(ParseError::new("invalid data for vn"))
        }
        Ok(Entity::VertexNormal{x: x.unwrap(), y: y.unwrap(), z: z.unwrap()})
    }

    fn parse_face(&self, split: &mut std::str::SplitWhitespace) -> Result<Entity, ParseError> {
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
                return Err(ParseError::new("could not parse face"));
            }
        }
        Ok(Entity::Face{vertices: face})
    }

    fn parse_polyline(&self, split: &mut std::str::SplitWhitespace) -> Result<Entity, ParseError> {
        let mut vertices = Vec::new();
        for x in split {
            vertices.push(x.parse::<i64>().unwrap())
        }
        Ok(Entity::Line{vertices})
    }

    pub fn parse_line(&self, split: &mut std::str::SplitWhitespace, token: &str, line: &str) -> Result<Entity, ParseError> {
        match token.to_lowercase().as_str() {
            "#" => {
                Ok(Entity::Comment{content: line.trim_start_matches("# ").to_owned()})
            }
            "o" => {
                Ok(Entity::Object{name: line.trim_start_matches("o ").to_owned()})
            }
            "g" => {
                Ok(Entity::Group{name: line.trim_start_matches("g ").to_owned()})
            }
            "s" => {
                Ok(Entity::SmoothingGroup{name: line.trim_start_matches("s ").to_owned()})
            }
            "v" => {
                self.parse_v(split)
            }
            "vn" => {
                self.parse_vn(split)
            }
            "vt" => {
                self.parse_vt(split)
            }
            "f" => {
                self.parse_face(split)
            }
            "l" => {
                self.parse_polyline(split)
            }
            "mtllib" => {
                if let Some(x) = split.next() {
                    Ok(Entity::Mtllib{name: x.to_owned()})
                } else {
                    Err(ParseError::new(format!("could not parse line \"{}\"", token).as_ref()))
                }
            },
            "usemtl" => {
                if let Some(x) = split.next() {
                    Ok(Entity::Usemtl{name: x.to_owned()})
                } else {
                    Err(ParseError::new(format!("could not parse line \"{}\"", token).as_ref()))
                }
            },
            _ => {
                Err(ParseError::new(format!("unknown token \"{}\"", token).as_ref()))
            }
        }
    }
}