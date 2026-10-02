use bevy::prelude::*;

use crate::Block;
use crate::BlockDirMap;
use crate::Direction;
use crate::MaterialHandle;

/// Material cache for every block-direction combination that has been requested.
#[derive(Resource, Default)]
pub struct TextureCache(BlockDirMap<Option<MaterialHandle>>);

impl TextureCache {
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
