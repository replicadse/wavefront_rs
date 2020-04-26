use std::fmt::Write;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct FaceVertex {
    pub vertex: i64,
    pub normal: Option<i64>,
    pub texture: Option<i64>,
}

impl FaceVertex {
    pub fn new(vertex: i64) -> FaceVertex {
        Self {
            vertex,
            normal: None,
            texture: None,
        }
    }

    pub fn new2(vertex: i64, normal: Option<i64>, texture: Option<i64>) -> FaceVertex {
        Self {
            vertex,
            normal,
            texture,
        }
    }
}


impl ToString for Entity {
    fn to_string(&self) -> String {
        let mut result = "".to_owned();
        match self {
            Entity::Comment{content} => {
                result.write_str(format!("# {}\n", content).as_ref()).unwrap();
            }
            Entity::Face{vertices} => {
                result.write_str("f".as_ref()).unwrap();
                for v in vertices {
                    result.write_str(" ".as_ref()).unwrap();
                    result.write_str(format!("{}", v.vertex).as_ref()).unwrap();
                    if let Some(x) = v.normal {
                        result.write_str(format!("/{}", x).as_ref()).unwrap();
                    }
                    if let Some(x) = v.texture {
                        result.write_str(format!("/{}", x).as_ref()).unwrap();
                    }
                }
                result.write_str("\n").as_ref().unwrap();
            }
            Entity::Line{vertices} => {
                result.write_str("l").as_ref().unwrap();
                for v in vertices {
                    result.write_str(format!(" {}", v).as_ref()).unwrap();
                }
                result.write_str("\n").as_ref().unwrap();
            }
            Entity::Group{name} => {
                result.write_str(format!("g {}\n", name).as_ref()).unwrap();
            }
            Entity::Mtllib{name} => {
                result.write_str(format!("mtllib {}\n", name).as_ref()).unwrap();
            }
            Entity::Object{name} => {
                result.write_str(format!("o {}\n", name).as_ref()).unwrap();
            }
            Entity::SmoothingGroup{name} => {
                result.write_str(format!("s {}\n", name).as_ref()).unwrap();
            }
            Entity::Usemtl{name} => {
                result.write_str(format!("usemtl {}\n", name).as_ref()).unwrap();
            }
            Entity::Vertex{x, y, z, w} => {
                result.write_str(format!("v {} {} {}", x, y, z).as_ref()).unwrap();
                if let Some(v) = w {
                    result.write_str(format!("{}", v).as_ref()).unwrap();
                }
                result.write_str("\n").as_ref().unwrap();
            }
            Entity::VertexNormal{x, y, z} => {
                result.write_str(format!("vn {} {} {}\n", x, y, z).as_ref()).unwrap();
            }
            Entity::VertexTexture{x, y, z} => {
                result.write_str(format!("vt {} {}", x, y).as_ref()).unwrap();
                if let Some(v) = z {
                    result.write_str(format!("{}", v).as_ref()).unwrap();
                }
                result.write_str("\n").as_ref().unwrap();
            }
        }
        result
    }
}
