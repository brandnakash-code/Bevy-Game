/// The number of blocks along a chunk's X and Z axes.
pub const CHUNK_SIZE: usize = 16;

/// The number of block positions along a chunk's Y axis.
pub const CHUNK_HEIGHT: usize = u8::MAX as usize;

/// The chunk radius around the player's current chunk that is loaded and rendered.
/// This produces a square region with `(2 * RENDER_DISTANCE + 1)^2` chunks.
pub const RENDER_DISTANCE: i32 = 16;

/// The seed passed to the Perlin noise generator, determining the generated terrain pattern.
pub const SEED: u32 = 123456789;

/// The spacing, in blocks, between sampled noise values before bilinear interpolation.
/// A value of `3` samples a 16-by-16 chunk on a 6-by-6 grid.
pub const NOISE_INTERPOLATION: usize = 3;

/// The base divisor applied to world X / Z coordinates before sampling noise.
/// Larger values produce larger, more widely spaced terrain features.
pub const WORLD_SCALE: f64 = 50.0;

/// The baseline terrain height in blocks before noise-layer contributions are added.
pub const BASE_HEIGHT: f64 = 64.0;

/// The maximum height contribution, in blocks, of the low-frequency noise layer.
pub const LOW_AMPLITUDE: f64 = 30.0;

/// The maximum height contribution, in blocks, of the medium-frequency noise layer.
pub const MED_AMPLITUDE: f64 = 25.0;

/// The maximum height contribution, in blocks, of the high-frequency noise layer.
pub const HIGH_AMPLITUDE: f64 = 25.0;

/// The divisor for the low-frequency noise layer. Larger values create broader features.
pub const LOW_FREQUENCY: f64 = 10.0;

/// The divisor for the medium-frequency noise layer. Larger values create broader features.
pub const MED_FREQUENCY: f64 = 5.0;

/// The divisor for the high-frequency noise layer. Larger values create broader features.
pub const HIGH_FREQUENCY: f64 = 2.0;
