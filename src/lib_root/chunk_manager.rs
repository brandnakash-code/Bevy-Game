use bevy::{ prelude::*, tasks::{ AsyncComputeTaskPool, Task } };
use futures_lite::future;
use std::collections::{ HashMap, HashSet };

use crate::lib_root::{
    chunk::Chunk,
    chunk_gen::{ generate, get_chunk_positions },
    consts::{ CHUNK_SIZE, RENDER_DISTANCE },
    block::Block,
};

type ChunkMeshMap = HashMap<Block, Handle<Mesh>>;

#[derive(Resource, Default)]
pub struct ChunkManager {
    entities: HashMap<IVec2, Entity>,
    chunks: HashMap<IVec2, Chunk>,
    meshes: HashMap<IVec2, HashMap<Block, Handle<Mesh>>>,
    pending: HashMap<IVec2, Task<Chunk>>,
    center: Option<IVec2>,
}

impl ChunkManager {
    pub fn new() -> Self {
        ChunkManager {
            entities: HashMap::new(),
            chunks: HashMap::new(),
            meshes: HashMap::new(),
            pending: HashMap::new(),
            center: None,
        }
    }

    /// Removes a chunk from Entities, but keeps it's mesh and block data.
    pub fn unload_chunk(&mut self, commands: &mut Commands, chunk_position: IVec2) {
        if let Some(entity) = self.entities.remove(&chunk_position) {
            commands.entity(entity).despawn();
        }
    }

    /// Takes a chunk, meshes it, and returns an entity.
    fn load_chunk(
        &mut self,
        chunk_position: IVec2,
        chunk: Chunk,
        meshes: &mut Assets<Mesh>,
        commands: &mut Commands,
        materials: &mut Assets<StandardMaterial>
    ) -> Entity {
        commands
            .spawn((
                Transform::from_xyz(
                    (chunk_position.x as f32) * (CHUNK_SIZE as f32),
                    0.0,
                    (chunk_position.y as f32) * (CHUNK_SIZE as f32)
                ),
            ))
            .with_children(|parent| {
                for (block, mesh) in self.mesh_and_add_chunk {
                    todo!();
                }
            })
            .id()
    }

    /// Loads a chunk that has been generated, meshed or unmeshed, and returns an entity.
    fn load_cached_chunk(
        &mut self,
        chunk_position: IVec2,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>
    ) -> Entity {
        // cached chunk has two states: meshed and unmeshed, and returns an Entity.
        let mesh = if let Some(mesh) = self.meshes.get(&chunk_position) {
            mesh.clone()
        } else {
            let mesh = meshes.add(self.chunks[&chunk_position].mesh());
            self.meshes.insert(chunk_position, mesh.clone());
            mesh
        };

        commands
            .spawn((
                Mesh3d(mesh),
                MeshMaterial3d(
                    materials.add(StandardMaterial {
                        base_color: Color::srgb(0.3, 0.7, 0.3),
                        ..default()
                    })
                ),
                Transform::from_xyz(
                    (chunk_position.x as f32) * (CHUNK_SIZE as f32),
                    0.0,
                    (chunk_position.y as f32) * (CHUNK_SIZE as f32)
                ),
            ))
            .id()
    }

    pub fn center(&self) -> Option<IVec2> {
        self.center
    }

    pub fn set_center(
        &mut self,
        commands: &mut Commands,
        center: IVec2,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>
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

        for new_chunk_position in new_chunk_positions {
            if
                !self.entities.contains_key(&new_chunk_position) && // not already loaded
                !self.pending.contains_key(&new_chunk_position) && // not scheduled
                !self.chunks.contains_key(&new_chunk_position) // not generated
            {
                let task_pool = AsyncComputeTaskPool::get();
                let task = task_pool.spawn(async move { generate(new_chunk_position) });
                self.pending.insert(new_chunk_position, task);
            } else if self.chunks.contains_key(&new_chunk_position) {
                // generated already
                let entity = self.load_cached_chunk(
                    new_chunk_position,
                    commands,
                    meshes,
                    materials
                );
                self.entities.insert(new_chunk_position, entity);
            }
        }

        self.center = Some(center);
    }

    pub fn poll_generation_tasks(
        &mut self,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>
    ) {
        let mut completed = Vec::new();

        for (&chunk_position, task) in &mut self.pending {
            if let Some(chunk) = future::block_on(future::poll_once(task)) {
                completed.push((chunk_position, chunk));
            }
        }

        for (chunk_position, chunk) in completed {
            self.pending.remove(&chunk_position);

            let Some(center) = self.center else {
                continue;
            };

            if !get_chunk_positions(center, RENDER_DISTANCE).contains(&chunk_position) {
                self.add_chunk(chunk_position, chunk);
                continue;
            }

            let entity = self.load_chunk(chunk_position, chunk, meshes, commands, materials);
            self.entities.insert(chunk_position, entity);
        }
    }

    /// Returns positions of loaded & rendered chunks.
    pub fn get_loaded_chunk_positions(&self) -> HashSet<IVec2> {
        self.entities.keys().cloned().collect()
    }

    /// Adds a chunk to the chunk manager
    fn add_chunk(&mut self, chunk_position: IVec2, chunk: Chunk) {
        self.chunks.insert(chunk_position, chunk);
    }

    fn update_mesh(
        &mut self,
        chunk_pos: IVec2,
        meshes: &mut Assets<Mesh>
    ) -> HashMap<Block, Handle<Mesh>> {}

    /// Adds a mesh and the given chunk to the chunk manager, and returns a HashMap<Block, Handle<Mesh>>
    fn mesh_and_add_chunk(
        &mut self,
        chunk_position: IVec2,
        chunk: Chunk,
        meshes: &mut Assets<Mesh>
    ) -> ChunkMeshMap {
        let mut chunk_mesh_map: ChunkMeshMap = HashMap::new();
        for (block, mesh) in chunk.mesh() {
            chunk_mesh_map.insert(block, meshes.add(mesh));
        }

        self.chunks.insert(chunk_position, chunk);
        self.meshes.insert(chunk_position, chunk_mesh_map.clone());

        chunk_mesh_map
    }
}
