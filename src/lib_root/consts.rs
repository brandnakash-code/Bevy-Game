pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = u8::MAX as usize;

pub const RENDER_DISTANCE: i32 = 16;

pub const SEED: u32 = 123456789;
pub const NOISE_INTERPOLATION: usize = 3;
pub const WORLD_SCALE: f64 = 50.0;
pub const BASE_HEIGHT: f64 = 64.0;

// frequency goes +/- AMPLITUDE in either direction
pub const LOW_AMPLITUDE: f64 = 30.0;
pub const MED_AMPLITUDE: f64 = 25.0;
pub const HIGH_AMPLITUDE: f64 = 25.0;

// bigger -> wider terrain, gentler slopes
pub const LOW_FREQUENCY: f64 = 10.0;
pub const MED_FREQUENCY: f64 = 5.0;
pub const HIGH_FREQUENCY: f64 = 2.0;
