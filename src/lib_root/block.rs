use bevy::prelude::*;

use std::collections::HashMap;

pub enum Texture {
    Empty,
    Color(Color),
    Path(&'static str),
}

impl Texture {
    fn material(self, asset_server: &AssetServer) -> Option<StandardMaterial> {
        match self {
            Texture::Empty => None,
            Texture::Color(color) => Some(StandardMaterial { base_color: color, ..default() }),
            Texture::Path(path) =>
                Some(StandardMaterial {
                    base_color_texture: Some(asset_server.load(path)),
                    ..default()
                }),
        }
    }
}

#[derive(Resource, Default)]
pub struct Textures(HashMap<Block, Option<Handle<StandardMaterial>>>);

impl Textures {
    pub fn get(
        &mut self,
        block: Block,
        asset_server: &AssetServer,
        materials: &mut Assets<StandardMaterial>
    ) -> Option<Handle<StandardMaterial>> {
        if let Some(texture) = self.0.get(&block) {
            texture.clone()
        } else {
            self.0.insert(
                block,
                block
                    .texture()
                    .material(asset_server)
                    .map(|texture| materials.add(texture))
            );
            self.0.get(&block).expect("just created, should exist").clone()
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum Block {
    Air,
    Block,
}

impl Block {
    pub const fn culls(self) -> bool {
        match self {
            Block::Air => false,
            Block::Block => true,
        }
    }

    pub const fn texture(self) -> Texture {
        match self {
            Block::Air => Texture::Empty,
            Block::Block => Texture::Path("grass.png"),
        }
    }
}
