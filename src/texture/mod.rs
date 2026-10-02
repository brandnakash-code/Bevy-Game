pub mod cache;

use bevy::prelude::*;

/// Describes how a block face should be rendered when it is visible.
pub enum Texture {
    /// No visible face should be created.
    Empty,
    /// A flat solid color.
    Color(Color),
    /// A texture loaded from an asset path.
    Path(&'static str),
    /// A texture tinted with a solid overlay color.
    Tinted(&'static str, Color),
}

impl Texture {
    /// Converts this texture description into a material, if the block should be rendered.
    fn material(self, asset_server: &AssetServer) -> Option<StandardMaterial> {
        match self {
            Texture::Empty => None,
            Texture::Color(color) => Some(StandardMaterial { base_color: color, ..default() }),
            Texture::Path(path) =>
                Some(StandardMaterial {
                    base_color_texture: Some(asset_server.load(path)),
                    ..default()
                }),
            Texture::Tinted(path, color) =>
                Some(StandardMaterial {
                    base_color: color,
                    base_color_texture: Some(asset_server.load(path)),
                    ..default()
                }),
        }
    }
}
