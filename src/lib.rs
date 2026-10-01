//! Voxel world utilities and shared data structures used by the Bevy terrain prototype.
//!
//! The crate is organized around chunk generation, chunk streaming, and face-based mesh
//! rendering. Most gameplay logic lives in the `lib_root` module, while the items exported
//! here provide the coordinate and map helpers that those systems depend on.

pub mod lib_root;

use bevy::prelude::*;

use std::collections::HashMap;
use std::collections::hash_map;

use crate::lib_root::block::Block;

/// Standard axis-aligned directions used to identify voxel faces.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

/// A map keyed by voxel block type.
#[derive(Deref, DerefMut, Default)]
pub struct BlockMap<T>(HashMap<Block, T>);

impl<T> FromIterator<(Block, T)> for BlockMap<T> {
    fn from_iter<I: IntoIterator<Item = (Block, T)>>(iter: I) -> Self {
        BlockMap(iter.into_iter().collect())
    }
}

impl<T> IntoIterator for BlockMap<T> {
    type Item = (Block, T);

    type IntoIter = hash_map::IntoIter<Block, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// A map keyed by a cardinal direction.
#[derive(Deref, DerefMut, Default)]
pub struct DirMap<T>(HashMap<Direction, T>);

impl<T> FromIterator<(Direction, T)> for DirMap<T> {
    fn from_iter<I: IntoIterator<Item = (Direction, T)>>(iter: I) -> Self {
        DirMap(iter.into_iter().collect())
    }
}

impl<T> IntoIterator for DirMap<T> {
    type Item = (Direction, T);

    type IntoIter = hash_map::IntoIter<Direction, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// A nested map where each block stores a directional map.
#[derive(Deref, DerefMut, Default)]
pub struct BlockDirMap<T>(BlockMap<DirMap<T>>);

impl<T> FromIterator<(Block, DirMap<T>)> for BlockDirMap<T> {
    fn from_iter<I: IntoIterator<Item = (Block, DirMap<T>)>>(iter: I) -> Self {
        BlockDirMap(iter.into_iter().collect())
    }
}

impl<T> IntoIterator for BlockDirMap<T> {
    type Item = (Block, DirMap<T>);

    type IntoIter = hash_map::IntoIter<Block, DirMap<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// A thin wrapper around a Bevy material handle for cached voxel textures.
#[derive(Deref, DerefMut, Default, Clone)]
pub struct MaterialHandle(Handle<StandardMaterial>);
