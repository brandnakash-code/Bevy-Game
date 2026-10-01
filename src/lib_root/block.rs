use bevy::prelude::*;

use crate::lib_root::textures::Texture;
use crate::Direction;

const fn rgb(red: u8, green: u8, blue: u8) -> Color {
    Color::srgb(red as f32, green as f32, blue as f32)
}

/// The set of voxel types that can exist inside a chunk.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Reflect, Default)]
pub enum Block {
    /// Empty space that does not render or cull geometry.
    Air,
    /// The default ground block used for terrain surfaces.
    #[default]
    Grass,
}

impl Block {
    /// Returns whether this block should suppress the generation of a face in the mesh.
    pub const fn culls(self) -> bool {
        match self {
            Block::Air => false,
            Block::Grass => true,
        }
    }

    /// Resolves the texture that should be used for a given face orientation.
    pub const fn texture(self, dir: Direction) -> Texture {
        match (self, dir) {
            (Block::Air, _) => Texture::Empty,
            (Block::Grass, _) => Texture::Tinted("grass_block_top.png", rgb(0, 125, 50)),
        }
    }
}
