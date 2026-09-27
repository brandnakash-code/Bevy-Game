use bevy::prelude::*;

use crate::lib_root::textures::Texture;
use crate::Direction;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum Block {
    Air,
    Grass,
}

impl Block {
    pub const fn culls(self) -> bool {
        match self {
            Block::Air => false,
            Block::Grass => true,
        }
    }

    pub const fn texture(self, dir: Direction) -> Texture {
        match (self, dir) {
            (Block::Air, _) => Texture::Empty,
            (Block::Grass, _) => Texture::Path("grass_block_top.png"),
        }
    }
}
