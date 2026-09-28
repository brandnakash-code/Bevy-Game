use bevy::{ asset::RenderAssetUsages, mesh::{ Indices, PrimitiveTopology }, prelude::* };

use std::{ cmp::{ max, min }, collections::HashMap };

#[allow(unused)]
use thiserror::Error;

use crate::lib_root::{
    block::Block,
    textures::{ Texture, BlockDirMap, DirMap },
    consts::{ CHUNK_SIZE, CHUNK_HEIGHT },
};
use crate::Direction;

type Vertex = [f32; 3];
type RawMeshMap = BlockDirMap<Mesh>;

fn order(a: usize, b: usize) -> std::ops::RangeInclusive<usize> {
    min(a, b)..=max(a, b)
}

#[allow(unused)]
#[derive(Error, Debug, PartialEq)]
#[error("Attempted to access block at ({attempted:?}), but chunk size is ({range:?})")]
struct OutOfChunkError {
    attempted: (isize, isize, isize),
    range: (usize, usize, usize),
}

#[derive(Default)]
struct TriangleData {
    positions: Vec<Vertex>,
    normals: Vec<Vertex>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

impl TriangleData {
    fn new() -> Self {
        Self::default()
    }

    fn mesh(self) -> Mesh {
        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs)
            .with_inserted_indices(Indices::U32(self.indices))
    }
}

#[derive(Resource)]
pub struct Chunk([[[Block; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE]);

impl core::ops::Index<usize> for Chunk {
    type Output = [[Block; CHUNK_SIZE]; CHUNK_HEIGHT];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl core::ops::IndexMut<usize> for Chunk {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk([[[Block::default(); CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE])
    }
}

impl Chunk {
    /// Creates an empty chunk filled with [`Block::Air`].
    pub fn new() -> Self {
        Chunk::default()
    }

    /// Builds one mesh per visible block type in the chunk.
    pub fn mesh(&self) -> RawMeshMap {
        let mut triangle_map: BlockDirMap<TriangleData> = HashMap::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_SIZE {
                    let block = self[x][y][z];
                    if !block.culls() {
                        continue;
                    }

                    let is_empty_in_any_direction =
                        matches!(block.texture(Direction::NegY), Texture::Empty) ||
                        matches!(block.texture(Direction::PosY), Texture::Empty) ||
                        matches!(block.texture(Direction::NegX), Texture::Empty) ||
                        matches!(block.texture(Direction::PosX), Texture::Empty) ||
                        matches!(block.texture(Direction::NegZ), Texture::Empty) ||
                        matches!(block.texture(Direction::PosZ), Texture::Empty);
                    if is_empty_in_any_direction {
                        continue;
                    }

                    let dirmap: &mut DirMap<TriangleData> = triangle_map
                        .entry(block)
                        .or_insert_with(|| HashMap::new());

                    if !self.culls((x as isize) + 1, y as isize, z as isize) {
                        add_face(dirmap, x, y, z, Direction::PosX);
                    }

                    if !self.culls((x as isize) - 1, y as isize, z as isize) {
                        add_face(dirmap, x, y, z, Direction::NegX);
                    }

                    if !self.culls(x as isize, (y as isize) + 1, z as isize) {
                        add_face(dirmap, x, y, z, Direction::PosY);
                    }

                    if !self.culls(x as isize, (y as isize) - 1, z as isize) {
                        add_face(dirmap, x, y, z, Direction::NegY);
                    }

                    if !self.culls(x as isize, y as isize, (z as isize) + 1) {
                        add_face(dirmap, x, y, z, Direction::PosZ);
                    }

                    if !self.culls(x as isize, y as isize, (z as isize) - 1) {
                        add_face(dirmap, x, y, z, Direction::NegZ);
                    }
                }
            }
        }

        triangle_map
            .into_iter()
            .map(|(block, dirmap)| (
                block,
                dirmap
                    .into_iter()
                    .map(|(dir, premesh)| (dir, premesh.mesh()))
                    .collect(),
            ))
            .collect()
    }

    /// Sets the block at the given local chunk coordinates.
    ///
    /// # Panics
    ///
    /// Panics when any coordinate is outside the chunk.
    pub fn set(&mut self, x: usize, y: usize, z: usize, block: Block) {
        assert!(Chunk::is_in(x as isize, y as isize, z as isize));

        self[x][y][z] = block;
    }

    /// Fills the inclusive cuboid between two local chunk coordinates.
    ///
    /// # Panics
    ///
    /// Panics when either corner is outside the chunk.
    pub fn fill(&mut self, x1: usize, y1: usize, z1: usize, x2: usize, y2: usize, z2: usize) {
        assert!(Chunk::is_in(x1 as isize, y1 as isize, z1 as isize));
        assert!(Chunk::is_in(x2 as isize, y2 as isize, z2 as isize));

        for x in order(x1, x2) {
            for y in order(y1, y2) {
                for z in order(z1, z2) {
                    self[x][y][z] = Block::Grass;
                }
            }
        }
    }

    /// Returns whether local coordinates are inside the chunk bounds.
    fn is_in(x: isize, y: isize, z: isize) -> bool {
        x >= 0 &&
            y >= 0 &&
            z >= 0 &&
            x < (CHUNK_SIZE as isize) &&
            y < (CHUNK_HEIGHT as isize) &&
            z < (CHUNK_SIZE as isize)
    }

    /// Returns the block at valid local coordinates.
    fn get(&self, x: usize, y: usize, z: usize) -> Block {
        assert!(Chunk::is_in(x as isize, y as isize, z as isize));
        self[x][y][z]
    }

    /// Checks whether a block matches at local coordinates without panicking on out-of-bounds input.
    #[allow(unused)]
    fn try_is(&self, block: Block, x: isize, y: isize, z: isize) -> Result<bool, OutOfChunkError> {
        if !Chunk::is_in(x, y, z) {
            return Err(OutOfChunkError {
                attempted: (x, y, z),
                range: (CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE),
            });
        }

        let (x, y, z) = (x as usize, y as usize, z as usize);

        Ok(self.get(x, y, z) == block)
    }

    /// Returns whether the block at local coordinates blocks visibility.
    fn culls(&self, x: isize, y: isize, z: isize) -> bool {
        if !Chunk::is_in(x, y, z) {
            return false;
        }

        let (x, y, z) = (x as usize, y as usize, z as usize);

        self[x][y][z].culls()
    }
}

fn add_face(data: &mut DirMap<TriangleData>, x: usize, y: usize, z: usize, dir: Direction) {
    let dir_map = data.entry(dir).or_insert_with(|| TriangleData::new());
    let start = dir_map.positions.len() as u32;
    let (x, y, z) = (x as f32, y as f32, z as f32);

    let (vertices, normal) = match dir {
        //
        //   Y       Z
        //   ^      /
        //   |  D------E
        //     /|     /|
        //    / |    / |
        //   C--B---G--F
        //   | /    | /
        //   |/     |/
        //   A------H    -> X
        // A = (x, y, z)
        // B = (x, y, z + 1)
        // C = (x, y + 1, z)
        // D = (x, y + 1, z + 1)
        // E = (x + 1, y + 1, z + 1)
        // F = (x + 1, y, z + 1)
        // G = (x + 1, y + 1, z)
        // H = (x + 1, y, z)

        //      E
        //     /|
        //    / |
        //   G  F
        //   | /
        //   |/
        //   H
        Direction::PosX =>
            (
                [
                    [x + 1.0, y, z], // H
                    [x + 1.0, y + 1.0, z], // G
                    [x + 1.0, y + 1.0, z + 1.0], // E
                    [x + 1.0, y, z + 1.0], // F
                ],
                [1.0, 0.0, 0.0],
            ),

        //      D
        //     /|
        //    / |
        //   C--B
        //   | /
        //   |/
        //   A
        Direction::NegX =>
            (
                [
                    [x, y, z + 1.0], // B
                    [x, y + 1.0, z + 1.0], // D
                    [x, y + 1.0, z], // C
                    [x, y, z], // A
                ],
                [-1.0, 0.0, 0.0],
            ),
        //      D------E
        //     /      /
        //    C------G
        Direction::PosY =>
            (
                [
                    [x, y + 1.0, z], // C
                    [x, y + 1.0, z + 1.0], // D
                    [x + 1.0, y + 1.0, z + 1.0], // E
                    [x + 1.0, y + 1.0, z], // G
                ],
                [0.0, 1.0, 0.0],
            ),
        //      A------H
        //     /      /
        //    B------F
        Direction::NegY =>
            (
                [
                    [x, y, z + 1.0], // B
                    [x, y, z], // A
                    [x + 1.0, y, z], // H
                    [x + 1.0, y, z + 1.0], // F
                ],
                [0.0, -1.0, 0.0],
            ),
        //      D------E
        //      |      |
        //      B------F
        Direction::PosZ =>
            (
                [
                    [x + 1.0, y, z + 1.0], // F
                    [x + 1.0, y + 1.0, z + 1.0], // E
                    [x, y + 1.0, z + 1.0], // D
                    [x, y, z + 1.0], // B
                ],
                [0.0, 0.0, 1.0],
            ),
        //      C------G
        //      |      |
        //      A------H
        Direction::NegZ =>
            (
                [
                    [x, y, z], // A
                    [x, y + 1.0, z], // C
                    [x + 1.0, y + 1.0, z], // G
                    [x + 1.0, y, z], // H
                ],
                [0.0, 0.0, -1.0],
            ),
    };

    dir_map.positions.extend(vertices);
    dir_map.normals.extend([normal; 4]);
    dir_map.uvs.extend([
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 1.0],
        [1.0, 0.0],
    ]);

    // Two triangles
    dir_map.indices.extend([start, start + 1, start + 2, start, start + 2, start + 3]);
}
