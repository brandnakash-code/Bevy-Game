use bevy::{ asset::RenderAssetUsages, mesh::{ Indices, PrimitiveTopology }, prelude::* };

use std::{ cmp::{ max, min }, collections::HashMap };
#[allow(unused)]
use thiserror::Error;

use crate::lib_root::block::{ Block };
use crate::lib_root::consts::{ CHUNK_SIZE, CHUNK_HEIGHT };

type Vertex = [f32; 3];

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

#[derive(Resource)]
pub struct Chunk {
    blocks: [[[Block; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE],
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { blocks: [[[Block::Air; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE] }
    }

    pub fn mesh(&self) -> HashMap<Block, Mesh> {
        let mut positions: Vec<Vertex> = Vec::new();
        let mut normals: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_SIZE {
                    if let None = self.blocks[x][y][z]. {
                        continue;
                    }

                    // Check every direction.
                    if !self.culls((x as isize) + 1, y as isize, z as isize) {
                        add_face(
                            &mut positions,
                            &mut normals,
                            &mut indices,
                            x as f32,
                            y as f32,
                            z as f32,
                            Direction::PosX
                        );
                    }

                    if !self.culls((x as isize) - 1, y as isize, z as isize) {
                        add_face(
                            &mut positions,
                            &mut normals,
                            &mut indices,
                            x as f32,
                            y as f32,
                            z as f32,
                            Direction::NegX
                        );
                    }

                    if !self.culls(x as isize, (y as isize) + 1, z as isize) {
                        add_face(
                            &mut positions,
                            &mut normals,
                            &mut indices,
                            x as f32,
                            y as f32,
                            z as f32,
                            Direction::PosY
                        );
                    }

                    if !self.culls(x as isize, (y as isize) - 1, z as isize) {
                        add_face(
                            &mut positions,
                            &mut normals,
                            &mut indices,
                            x as f32,
                            y as f32,
                            z as f32,
                            Direction::NegY
                        );
                    }

                    if !self.culls(x as isize, y as isize, (z as isize) + 1) {
                        add_face(
                            &mut positions,
                            &mut normals,
                            &mut indices,
                            x as f32,
                            y as f32,
                            z as f32,
                            Direction::PosZ
                        );
                    }

                    if !self.culls(x as isize, y as isize, (z as isize) - 1) {
                        add_face(
                            &mut positions,
                            &mut normals,
                            &mut indices,
                            x as f32,
                            y as f32,
                            z as f32,
                            Direction::NegZ
                        );
                    }
                }
            }
        }

        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_indices(Indices::U32(indices))
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, block: Block) {
        assert!(Chunk::is_in(x as isize, y as isize, z as isize));

        self.blocks[x][y][z] = block;
    }

    pub fn fill(&mut self, x1: usize, y1: usize, z1: usize, x2: usize, y2: usize, z2: usize) {
        assert!(Chunk::is_in(x1 as isize, y1 as isize, z1 as isize));
        assert!(Chunk::is_in(x2 as isize, y2 as isize, z2 as isize));

        for x in order(x1, x2) {
            for y in order(y1, y2) {
                for z in order(z1, z2) {
                    self.blocks[x][y][z] = Block::Block;
                }
            }
        }
    }

    fn is_in(x: isize, y: isize, z: isize) -> bool {
        x >= 0 &&
            y >= 0 &&
            z >= 0 &&
            x < (CHUNK_SIZE as isize) &&
            y < (CHUNK_HEIGHT as isize) &&
            z < (CHUNK_SIZE as isize)
    }

    fn get(&self, x: usize, y: usize, z: usize) -> Block {
        assert!(Chunk::is_in(x as isize, y as isize, z as isize));
        self.blocks[x][y][z]
    }

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

    fn culls(&self, x: isize, y: isize, z: isize) -> bool {
        if !Chunk::is_in(x, y, z) {
            return false;
        }

        let (x, y, z) = (x as usize, y as usize, z as usize);

        self.blocks[x][y][z].culls()
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk::new()
    }
}

#[derive(Clone, Copy)]
enum Direction {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

fn add_face(
    positions: &mut Vec<Vertex>,
    normals: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
    x: f32,
    y: f32,
    z: f32,
    direction: Direction
) {
    let start = positions.len() as u32;

    let (vertices, normal) = match direction {
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

    positions.extend(vertices);
    normals.extend([normal; 4]);

    // Two triangles
    indices.extend([start, start + 1, start + 2, start, start + 2, start + 3]);
}
