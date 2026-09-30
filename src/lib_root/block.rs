use bevy::prelude::*;

use crate::lib_root::textures::Texture;
use crate::Direction;

const fn rgb(red: u8, green: u8, blue: u8) -> Color {
    Color::srgb(red as f32, green as f32, blue as f32)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Reflect, Default)]
pub enum Block {
    Air,
    #[default]
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
            (Block::Grass, _) => Texture::Tinted("grass_block_top.png", rgb(0, 125, 50)),
        }
    }
}
