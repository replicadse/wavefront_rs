//! Contains the logic to transform entities to MTL formatted strings.

use std::io::Write;

use crate::mtl::entity::Entity;

/// Will write entities to a `Write` trait.
pub struct Writer {
    pub auto_newline: bool,
}

impl Writer {
    /// Writes the given entity to the given `Write` trait as MTL format
    /// representation of that `Entity`.
    pub fn write<W: Write>(&self, writer: &mut W, e: &Entity) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let safecall = move |writer: &mut W, e: &Entity| -> std::result::Result<(), Box<dyn std::error::Error>> {
            match e {
                | Entity::Comment { content } => {
                    writer.write_all(format!("{} {}", e.token(), content).as_ref())?;
                },
                | Entity::MaterialName { name } => {
                    writer.write_all(format!("{} {}", e.token(), name).as_ref())?;
                },
                | Entity::AmbientColor { r, g, b } => {
                    writer.write_all(format!("{} {} {} {}", e.token(), r, g, b).as_ref())?;
                },
                | Entity::DiffuseColor { r, g, b } => {
                    writer.write_all(format!("{} {} {} {}", e.token(), r, g, b).as_ref())?;
                },
                | Entity::SpecularColor { r, g, b } => {
                    writer.write_all(format!("{} {} {} {}", e.token(), r, g, b).as_ref())?;
                },
                | Entity::SpecularHighlights { value } => {
                    writer.write_all(format!("{} {}", e.token(), value).as_ref())?;
                },
                | Entity::OpticalDensity { value } => {
                    writer.write_all(format!("{} {}", e.token(), value).as_ref())?;
                },
                | Entity::Dissolve { value } => {
                    writer.write_all(format!("{} {}", e.token(), value).as_ref())?;
                },
                | Entity::InvertedDissolve { value } => {
                    writer.write_all(format!("{} {}", e.token(), value).as_ref())?;
                },
                | Entity::Illum { mode } => {
                    writer.write_all(format!("{} {}", e.token(), mode.to_string()).as_ref())?;
                },
                | Entity::TextureMapAmbient { file } => {
                    writer.write_all(format!("{} {}", e.token(), file).as_ref())?;
                },
                | Entity::TextureMapDiffuse { file } => {
                    writer.write_all(format!("{} {}", e.token(), file).as_ref())?;
                },
                | Entity::TransmissionFilterColorRGB { r, g, b } => {
                    writer.write_all(format!("{} {} {} {}", e.token(), r, g, b).as_ref())?;
                },
                | Entity::TextureMapSpecular { file } => {
                    writer.write_all(format!("{} {}", e.token(), file).as_ref())?;
                },
                | Entity::TextureMapHighlight { file } => {
                    writer.write_all(format!("{} {}", e.token(), file).as_ref())?;
                },
                | Entity::TextureMapAlpha { file } => {
                    writer.write_all(format!("{} {}", e.token(), file).as_ref())?;
                },
                | Entity::BumpMap { file } => {
                    writer.write_all(format!("{} {}", e.token(), file).as_ref())?;
                },
                | Entity::DisplacementMap { file } => {
                    writer.write_all(format!("{} {}", e.token(), file).as_ref())?;
                },
                | Entity::StencilDecalTextureMap { file } => {
                    writer.write_all(format!("{} {}", e.token(), file).as_ref())?;
                },
                | Entity::SphericalReflectionMap { file } => {
                    writer.write_all(format!("{} {}", e.token(), file).as_ref())?;
                },
            }
            if self.auto_newline {
                writer.write_all("\n".as_bytes())?;
            }
            Ok(())
        };
        match safecall(writer, e) {
            | Ok(..) => Ok(()),
            | Err(err) => Err(Box::new(crate::error::WriterError::new(err.to_string().as_ref()))),
        }
    }
}
