//! Core world-generation and rendering modules.
//!
//! These modules are responsible for the chunk representation, procedural terrain generation,
//! streaming nearby chunks, and material management for the voxel meshes.

pub mod chunk;
pub mod block;
pub mod chunk_gen;
pub mod chunk_manager;
pub mod consts;
pub mod textures;
