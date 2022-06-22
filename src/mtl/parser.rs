//! Contains logic to read entities from a `BufRead` that returns MTL formatted strings.
//!

use crate::mtl::entity::IllumMode;
use crate::mtl::entity::Entity;
use std::io::BufRead;
use std::result::Result;

/// Will read from a given `BufRead` and parse entities.
pub struct Parser {}

impl Parser {
    /// Will read from the given `BufRead`as long as it is not EOF.\
    /// When an entity is parsed, the given callback is invoked and the entity is inserted into it as parameter.\
    /// Will return `Ok(())` if successful or an `Error` (if parsing failed).
    pub fn read_to_end<R: BufRead>(
        reader: &mut R,
        mut callback: impl FnMut(Entity),
    ) -> Result<(), Box<dyn std::error::Error>> {
        for l in reader.lines() {
            let s: String = l?;
            let mut split = s.split_whitespace();
            if let Some(x) = split.next() {
                match Self::parse_split(&mut split, x, s.as_ref()) {
                    Ok(x) => callback(x),
                    Err(x) => return Err(x),
                }
            }
        }
        Ok(())
    }

    /// Will read from the given `BufRead` until the first encountered linebreak.\
    /// Will return `Ok(Entity)` if successful or an `Error` (if parsing failed).
    pub fn parse_line<R: BufRead>(reader: &mut R) -> Result<Entity, Box<dyn std::error::Error>> {
        let value = &mut String::new();
        match reader.read_line(value) {
            Ok(x) => {
                if x > 0 {
                    let mut split = value.split_whitespace();
                    match split.next() {
                        Some(x) => Self::parse_split(&mut split, x, value.as_ref()),
                        None => Err(Box::new(crate::error::ParserError::new("invalid line"))),
                    }
                } else {
                    Err(Box::new(crate::error::ParserError::new("reached EOF")))
                }
            }
            Err(..) => Err(Box::new(crate::error::ParserError::new(
                "error reading line",
            ))),
        }
    }

    fn parse_split(
        split: &mut std::str::SplitWhitespace,
        token: &str,
        line: &str,
    ) -> std::result::Result<Entity, Box<dyn std::error::Error>> {
        match token.to_lowercase().as_str() {
            "#" => Ok(Entity::Comment {
                content: line.trim_start_matches("# ").to_owned(),
            }),
            "newmtl" => Ok(Entity::MaterialName {
                name: line.trim_start_matches("newmtl ").to_owned(),
            }),
            "ka" => {
                let vals = Self::parse_f64_triplet(split)?;
                Ok(Entity::AmbientColor {
                    r: vals.0,
                    g: vals.1,
                    b: vals.2,
                })
            },
            "kd" => {
                let vals = Self::parse_f64_triplet(split)?;
                Ok(Entity::DiffuseColor {
                    r: vals.0,
                    g: vals.1,
                    b: vals.2,
                })
            },
            "ks" => {
                let vals = Self::parse_f64_triplet(split)?;
                Ok(Entity::SpecularColor {
                    r: vals.0,
                    g: vals.1,
                    b: vals.2,
                })
            },
            "ns" => Ok(Entity::SpecularHighlights {
                value: Self::parse_value::<f64>(split)?
            }),
            "ni" => Ok(Entity::OpticalDensity {
                value: Self::parse_value::<f64>(split)?
            }),
            "d" => Ok(Entity::Dissolve {
                value: Self::parse_value::<f64>(split)?
            }),
            "tr" => Ok(Entity::InvertedDissolve {
                value: Self::parse_value::<f64>(split)?
            }),
            "illum" => Ok(Entity::Illum {
                mode: IllumMode::from(Self::parse_value::<i16>(split)?)
            }),
            "tf" => {
                let vals = Self::parse_f64_triplet(split)?;
                Ok(Entity::TransmissionFilterColorRGB {
                    r: vals.0,
                    g: vals.1,
                    b: vals.2,
                })
            },
            "map_ka" => Ok(Entity::TextureMapAmbient {
                file: line.trim_start_matches("map_ka ").to_owned(),
            }),
            "map_kd" => Ok(Entity::TextureMapDiffuse {
                file: line.trim_start_matches("map_kd ").to_owned(),
            }),
            "map_ks" => Ok(Entity::TextureMapSpecular {
                file: line.trim_start_matches("map_ks ").to_owned(),
            }),
            "map_ns" => Ok(Entity::TextureMapHighlight {
                file: line.trim_start_matches("map_ns ").to_owned(),
            }),
            "map_d" => Ok(Entity::TextureMapAlpha {
                file: line.trim_start_matches("map_d ").to_owned(),
            }),
            "map_bump" => Ok(Entity::BumpMap {
                file: line.trim_start_matches("map_bump ").to_owned(),
            }),
            "bump" => Ok(Entity::BumpMap {
                file: line.trim_start_matches("bump ").to_owned(),
            }),
            "disp" => Ok(Entity::DisplacementMap {
                file: line.trim_start_matches("disp ").to_owned(),
            }),
            "decal" => Ok(Entity::StencilDecalTextureMap {
                file: line.trim_start_matches("decal ").to_owned(),
            }),
            "refl" => Ok(Entity::SphericalReflectionMap {
                file: line.trim_start_matches("refl ").to_owned(),
            }),
            _ => Err(Box::new(crate::error::ParserError::new(
                format!("unknown token \"{}\"", token).as_ref(),
            ))),
        }
    }

    fn parse_value<T: std::str::FromStr>(
        split: &mut std::str::SplitWhitespace,
    ) -> std::result::Result<T, Box<dyn std::error::Error>> {
        let xs = split.next();
        if xs == None {
            return Err(Box::new(crate::error::ParserError::new(
                "invalid data",
            )));
        }
        let x = xs.unwrap().parse::<T>();
        if x.is_err() {
            return Err(Box::new(crate::error::ParserError::new(
                "invalid data",
            )));
        }
        match x {
            Ok(v) => Ok(v),
            Err(..) => Err(Box::new(crate::error::ParserError::new("parsing failure")))
        }
    }

    fn parse_f64_triplet(
        split: &mut std::str::SplitWhitespace,
    ) -> std::result::Result<(f64, f64, f64), Box<dyn std::error::Error>> {
        let xs = split.next();
        let ys = split.next();
        let zs = split.next();
        if xs == None || ys == None || zs == None {
            return Err(Box::new(crate::error::ParserError::new(
                "invalid data",
            )));
        }
        let x = xs.unwrap().parse::<f64>();
        let y = ys.unwrap().parse::<f64>();
        let z = zs.unwrap().parse::<f64>();
        if x.is_err() || y.is_err() || z.is_err() {
            return Err(Box::new(crate::error::ParserError::new(
                "invalid data",
            )));
        }
        Ok((x?, y?, z?))
    }
}
