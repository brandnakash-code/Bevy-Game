# Bevy-Game

A lightweight voxel terrain prototype built with Bevy. The project generates a chunked world using layered Perlin noise, renders visible faces as meshes, and streams nearby chunks around the camera as the player moves through the world.

## Features

- Procedural terrain generation with layered noise
- Chunk-based world generation and loading
- Mesh generation for visible voxel faces only
- Texture/material caching for voxel faces
- Basic first-person camera controls with mouse look and WASD movement

## Project layout

- `src/main.rs` sets up the Bevy app, camera, lighting, and runtime systems.
- `src/lib.rs` declares and re-exports the library modules, and defines shared coordinate and map types.
- `src/block.rs` defines voxel block types and their texture lookup rules.
- `src/chunk/` contains the in-memory chunk representation, mesh generation, terrain generation, and chunk streaming.
- `src/texture/` describes block textures and caches Bevy materials for each block face.
- `src/consts.rs` stores chunk size, world-generation tuning, and render-distance values.

## Rust source organization

Keep each Rust source file's header in this order, with implementation code after the imports:

1. `pub mod` declarations
2. `pub use` re-exports
3. `use bevy::...` imports
4. `use std::...` imports
5. Imports from other crates
6. `use crate::...` imports

## Running the game

```bash
cargo run
```

## Controls

- `W`, `A`, `S`, `D`: move
- `Space`: ascend
- `Shift`: descend
- Mouse: look around
- Left click: lock cursor
- `Esc`: release cursor

## Notes

The terrain is intentionally simple and designed as a learning project. Chunk generation runs asynchronously, while the visible chunk set is updated around the player's current chunk position to keep the world streaming-friendly.
