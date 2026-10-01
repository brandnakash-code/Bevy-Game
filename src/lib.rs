pub mod lib_root;

use bevy::prelude::*;

use std::collections::HashMap;
use std::collections::hash_map;

use crate::lib_root::block::Block;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

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

#[derive(Deref, DerefMut, Default, Clone)]
pub struct MaterialHandle(Handle<StandardMaterial>);
