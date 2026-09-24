use bevy::prelude::*;

use crate::lib_root::textures::Texture;

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
