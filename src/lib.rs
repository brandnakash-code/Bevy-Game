pub mod lib_root;

use bevy::prelude::*;

use derive_more::FromIterator;
use std::collections::HashMap;

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

#[derive(Deref, DerefMut, Default, FromIterator)]
pub struct BlockMap<T>(HashMap<Block, T>);

#[derive(Deref, DerefMut, Default)]
pub struct DirMap<T>(HashMap<Direction, T>);

#[derive(Deref, DerefMut, Default, FromIterator)]
pub struct BlockDirMap<T>(BlockMap<DirMap<T>>);

#[derive(Deref, DerefMut, Default, Clone)]
pub struct MaterialHandle(Handle<StandardMaterial>);
