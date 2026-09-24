use bevy::{ prelude::*, tasks::{ AsyncComputeTaskPool, Task } };
use futures_lite::future;
use std::collections::{ HashMap, HashSet };

use crate::lib_root::{
    chunk::Chunk,
    chunk_gen::{ generate, get_chunk_positions },
    consts::{ CHUNK_SIZE, RENDER_DISTANCE },
    block::{ Block, Textures },
};

type MultiMeshMap = HashMap<Block, Handle<Mesh>>;

#[derive(Resource, Default)]
pub struct ChunkManager {
    entities: HashMap<IVec2, Entity>,
    chunks: HashMap<IVec2, Chunk>,
    meshes: HashMap<IVec2, MultiMeshMap>,
    pending: HashMap<IVec2, Task<Chunk>>,
    center: Option<IVec2>,
}

impl ChunkManager {
    pub fn new() -> Self {
        ChunkManager::default()
    }

    /// Removes a chunk from Entities, but keeps it's mesh and block data.
    pub fn unload_chunk(&mut self, commands: &mut Commands, chunk_position: IVec2) {
        if let Some(entity) = self.entities.remove(&chunk_position) {
            commands.entity(entity).despawn();
        }
    }

    /// Takes a chunk, meshes it, and returns an entity.
    #[allow(clippy::too_many_arguments)] // SHUT THE FUCK UP CLIPPY I NEED ALL THESE OKAY
    fn load_new_chunk(
        &mut self,
        chunk_pos: IVec2,
        chunk: Chunk,
        meshes: &mut Assets<Mesh>,
        commands: &mut Commands,
        materials: &mut Assets<StandardMaterial>,
        textures: &mut Textures,
        asset_server: &AssetServer
    ) -> Entity {
        let mesh_map: MultiMeshMap = self.add_chunk_and_mesh(chunk_pos, chunk, meshes);

        self.load_chunk_from_meshmap(
            chunk_pos,
            mesh_map,
            commands,
            materials,
            textures,
            asset_server
        )
    }

    /// Loads an already generated chunk, meshes it if needed, and returns an entity.
    fn load_existing_chunk(
        &mut self,
        chunk_pos: IVec2,
        meshes: &mut Assets<Mesh>,
        commands: &mut Commands,
        materials: &mut Assets<StandardMaterial>,
        textures: &mut Textures,
        asset_server: &AssetServer
    ) -> Entity {
        let mesh_map: MultiMeshMap = if let Some(mesh_map) = self.meshes.get(&chunk_pos) {
            mesh_map.clone()
        } else {
            self.update_mesh(chunk_pos, meshes).expect("chunk should already exist")
        };

        self.load_chunk_from_meshmap(
            chunk_pos,
            mesh_map,
            commands,
            materials,
            textures,
            asset_server
        )
    }

    /// read the fucking function name
    fn load_chunk_from_meshmap(
        &mut self,
        chunk_pos: IVec2,
        mesh_map: MultiMeshMap,
        commands: &mut Commands,
        materials: &mut Assets<StandardMaterial>,
        textures: &mut Textures,
        asset_server: &AssetServer
    ) -> Entity {
        commands
            .spawn((
                Transform::from_xyz(
                    (chunk_pos.x as f32) * (CHUNK_SIZE as f32),
                    0.0,
                    (chunk_pos.y as f32) * (CHUNK_SIZE as f32)
                ),
                Visibility::default(),
            ))
            .with_children(|parent| {
                for (block, mesh) in mesh_map {
                    let Some(material) = textures.get(block, asset_server, materials) else {
                        continue;
                    };

                    parent.spawn((Mesh3d(mesh), MeshMaterial3d(material)));
                }
            })
            .id()
    }

    /// returns the center.
    pub fn center(&self) -> Option<IVec2> {
        self.center
    }

    /// Loads chunks in a square around the center, schedules non existing chunks to be generated.
    pub fn set_center(
        &mut self,
        center: IVec2,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        textures: &mut Textures,
        asset_server: &AssetServer
    ) {
        if let Some(current_center) = self.center && current_center == center {
            return;
        }

        let new_chunk_positions: HashSet<IVec2> = get_chunk_positions(center, RENDER_DISTANCE);

        for old_chunk_position in self.get_loaded_chunk_positions() {
            if !new_chunk_positions.contains(&old_chunk_position) {
                self.unload_chunk(commands, old_chunk_position);
            }
        }

        for new_chunk_pos in new_chunk_positions {
            if
                !self.entities.contains_key(&new_chunk_pos) && // not already loaded
                !self.pending.contains_key(&new_chunk_pos) && // not scheduled
                !self.chunks.contains_key(&new_chunk_pos) // not generated
            {
                self.schedule_chunk_gen(new_chunk_pos);
            } else if
                self.chunks.contains_key(&new_chunk_pos) &&
                !self.entities.contains_key(&new_chunk_pos)
            {
                // generated already
                let entity = self.load_existing_chunk(
                    new_chunk_pos,
                    meshes,
                    commands,
                    materials,
                    textures,
                    asset_server
                );
                self.entities.insert(new_chunk_pos, entity);
            }
        }

        self.center = Some(center);
    }

    /// Generates scheduled chunks.
    pub fn poll_generation_tasks(
        &mut self,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        textures: &mut Textures,
        asset_server: &AssetServer
    ) {
        let Some(center) = self.center else {
            return;
        };

        let mut completed = HashMap::new();

        for (&chunk_pos, task) in &mut self.pending {
            if let Some(chunk) = future::block_on(future::poll_once(task)) {
                completed.insert(chunk_pos, chunk);
            }
        }

        let render = get_chunk_positions(center, RENDER_DISTANCE);

        for (chunk_pos, chunk) in completed {
            self.pending.remove(&chunk_pos);

            // Don't mesh and move on if out of range when already generated
            if !render.contains(&chunk_pos) {
                self.add_chunk(chunk_pos, chunk);
                continue;
            }

            let entity = self.load_new_chunk(
                chunk_pos,
                chunk,
                meshes,
                commands,
                materials,
                textures,
                asset_server
            );
            self.entities.insert(chunk_pos, entity);
        }
    }

    /// Returns positions of loaded & rendered chunks.
    pub fn get_loaded_chunk_positions(&self) -> HashSet<IVec2> {
        self.entities.keys().cloned().collect()
    }

    /// Schedules a chunk to be generated.
    fn schedule_chunk_gen(&mut self, chunk_pos: IVec2) {
        let task_pool = AsyncComputeTaskPool::get();
        let task = task_pool.spawn(async move { generate(chunk_pos) });
        self.pending.insert(chunk_pos, task);
    }

    /// Adds a chunk.
    fn add_chunk(&mut self, chunk_pos: IVec2, chunk: Chunk) {
        self.chunks.insert(chunk_pos, chunk);
    }

    /// Updates a mesh for a chunk, returns None if the chunk doesn't exist.
    fn update_mesh(&mut self, chunk_pos: IVec2, meshes: &mut Assets<Mesh>) -> Option<MultiMeshMap> {
        let Some(chunk) = self.chunks.get(&chunk_pos) else {
            self.meshes.remove(&chunk_pos);
            return None;
        };

        let mut chunk_mesh_map: MultiMeshMap = HashMap::new();
        for (block, mesh) in chunk.mesh() {
            chunk_mesh_map.insert(block, meshes.add(mesh));
        }

        self.meshes.insert(chunk_pos, chunk_mesh_map.clone());
        Some(chunk_mesh_map)
    }

    /// Adds a mesh and a chunk.
    fn add_chunk_and_mesh(
        &mut self,
        chunk_pos: IVec2,
        chunk: Chunk,
        meshes: &mut Assets<Mesh>
    ) -> MultiMeshMap {
        self.chunks.insert(chunk_pos, chunk);
        self.update_mesh(chunk_pos, meshes).expect("just created chunk, should exist")
    }
}
