use bevy::prelude::*;

use crate::lib_root::block::Block;

use crate::Direction;
use crate::BlockDirMap;
use crate::MaterialHandle;

pub enum Texture {
    Empty,
    Color(Color),
    Path(&'static str),
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

#[derive(Resource, Default)]
pub struct Textures(BlockDirMap<Option<MaterialHandle>>);

impl Textures {
    /// Returns the cached material handle for a block, creating it on first use.
    ///
    /// The returned handle is `None` for blocks whose texture is [`Texture::Empty`].
    pub fn get(
        &mut self,
        block: Block,
        dir: Direction,
        asset_server: &AssetServer,
        materials: &mut Assets<StandardMaterial>
    ) -> Option<MaterialHandle> {
        self.0
            .entry(block)
            .or_default()
            .entry(dir)
            .or_insert_with(||
                block
                    .texture(dir)
                    .material(asset_server)
                    .map(|inner| MaterialHandle(materials.add(inner)))
            )
            .clone()
    }
}
