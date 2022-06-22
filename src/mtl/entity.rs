//! Contains the entity types that are used when working with the [`wavefront obj`] format.
//!
//! [`wavefront obj`]: https://en.wikipedia.org/wiki/Wavefront_.obj_file
//!

use crate::mtl::parser::*;
use crate::mtl::writer::*;
use std::io::{BufReader, BufWriter, Cursor};

pub type Format = String;

#[derive(Debug, Clone, PartialEq)]
pub enum IllumMode {
    /// Color on and Ambient off
    ColorNoAmbient,
    /// Color on and Ambient on
    ColorAndAmbient,
    /// Highlight on
    Highlight,
    /// Reflection on and Ray trace on
    ReflecRaytrace,
    /// Transparency: Glass on, Reflection: Ray trace on
    TranspGlassAndReflecRaytrace,
    /// Reflection: Fresnel on and Ray trace on
    ReflecFresnelRaytrace,
    /// Transparency: Refraction on, Reflection: Fresnel off and Ray trace on
    TranspRefracRaytraceNoFresnel,
    /// Transparency: Refraction on, Reflection: Fresnel on and Ray trace on
    TranspRefracRaytraceFresnel,
    /// Reflection on and Ray trace off
    ReflecNoRaytrace,
    /// Transparency: Glass on, Reflection: Ray trace off
    TranspGlassNoRaytrace,
    /// Casts shadows onto invisible surfaces
    ShadowsToVisSurfaces,
}

/// Contains all possible entities that can exist in an MTL format.
#[derive(Debug, Clone, PartialEq)]
pub enum Entity {
    Comment {
        content: String,
    },
    MaterialName {
        name: String,
    },
    AmbientColor {
        r: f64,
        g: f64,
        b: f64,
    },
    DiffuseColor {
        r: f64,
        g: f64,
        b: f64,
    },
    SpecularColor {
        r: f64,
        g: f64,
        b: f64,
    },
    SpecularHighlights {
        value: f64,
    },
    OpticalDensity {
        value: f64,
    },
    Dissolve {
        value: f64,
    },
    InvertedDissolve {
        value: f64,
    },
    Illum {
        mode: IllumMode,
    },
    TextureMapAmbient {
        file: String,
    },
    TextureMapDiffuse {
        file: String,
    },
    TransmissionFilterColorRGB {
        r: f64,
        g: f64,
        b: f64,
    },
    TextureMapSpecular {
        file: String,
    },
    TextureMapHighlight {
        file: String,
    },
    TextureMapAlpha {
        file: String,
    },
    BumpMap {
        file: String,
    },
    DisplacementMap {
        file: String,
    },
    StencilDecalTextureMap {
        file: String,
    },
    SphericalReflectionMap {
        file: String,
    }
}

impl Entity {
    pub fn token(&self) -> &str {
        match self {
            Self::Comment { .. } => "#",
            Self::MaterialName { .. } => "newmtl",
            Self::AmbientColor { .. } => "ka",
            Self::DiffuseColor { .. } => "kd",
            Self::SpecularColor { .. } => "ks",
            Self::SpecularHighlights { .. } => "ns",
            Self::OpticalDensity { .. } => "ni",
            Self::Dissolve { .. } => "d",
            Self::InvertedDissolve { .. } => "tr",
            Self::Illum { .. } => "illum",
            Self::TransmissionFilterColorRGB { .. } => "tf",
            Self::TextureMapAmbient { .. } => "map_ka",
            Self::TextureMapDiffuse { .. } => "map_kd",
            Self::TextureMapSpecular { .. } => "map_ks",
            Self::TextureMapHighlight { .. } => "map_ns",
            Self::TextureMapAlpha { .. } => "map_d",
            Self::BumpMap { .. } => "map_bump",
            Self::DisplacementMap { .. } => "disp",
            Self::StencilDecalTextureMap { .. } => "decal",
            Self::SphericalReflectionMap { .. } => "refl",
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

impl ToString for IllumMode {
    fn to_string(&self) -> String {
        match self {
            Self::ColorNoAmbient => 0.to_string(),
            Self::ColorAndAmbient => 1.to_string(),
            Self::Highlight => 2.to_string(),
            Self::ReflecRaytrace => 3.to_string(),
            Self::TranspGlassAndReflecRaytrace => 4.to_string(),
            Self::ReflecFresnelRaytrace => 5.to_string(),
            Self::TranspRefracRaytraceNoFresnel => 6.to_string(),
            Self::TranspRefracRaytraceFresnel => 7.to_string(),
            Self::ReflecNoRaytrace => 8.to_string(),
            Self::TranspGlassNoRaytrace => 9.to_string(),
            Self::ShadowsToVisSurfaces => 10.to_string(),
        }
    }
}

impl From<i16> for IllumMode {
    fn from(input: i16) -> Self {
        match input {
            0 => Self::ColorNoAmbient,
            1 => Self::ColorAndAmbient,
            2 => Self::Highlight,
            3 => Self::ReflecRaytrace,
            4 => Self::TranspGlassAndReflecRaytrace,
            5 => Self::ReflecFresnelRaytrace,
            6 => Self::TranspRefracRaytraceNoFresnel,
            7 => Self::TranspRefracRaytraceFresnel,
            8 => Self::ReflecNoRaytrace,
            9 => Self::TranspGlassNoRaytrace,
            10 => Self::ShadowsToVisSurfaces,
            _ => Err(crate::error::ParserError::new("could not parse illum mode")).unwrap()
        }
    }
}

impl From<IllumMode> for String {
    fn from(e: IllumMode) -> Self {
        e.to_string()
    }
}
