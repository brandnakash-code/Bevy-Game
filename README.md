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
- `src/lib.rs` defines shared helpers and directional map types used across the voxel system.
- `src/lib_root/` contains the gameplay and world-generation components:
  - `block.rs` defines the voxel block types and texture lookup rules.
  - `chunk.rs` contains the in-memory chunk representation and mesh-building logic.
  - `chunk_gen.rs` handles terrain generation, chunk coordinate conversion, and noise sampling.
  - `chunk_manager.rs` schedules async chunk generation and manages loaded chunk entities.
  - `textures.rs` caches Bevy materials for each block face.
  - `consts.rs` stores chunk size, world-generation tuning, and render distance values.

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
