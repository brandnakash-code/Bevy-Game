use bevy::prelude::*;

use std::collections::HashSet;

use noise::{ NoiseFn, Perlin };

use crate::lib_root::{
    consts::{
        BASE_HEIGHT,
        CHUNK_HEIGHT,
        CHUNK_SIZE,
        HIGH_AMPLITUDE,
        HIGH_FREQUENCY,
        LOW_AMPLITUDE,
        LOW_FREQUENCY,
        MED_AMPLITUDE,
        MED_FREQUENCY,
        NOISE_INTERPOLATION,
        SEED,
        WORLD_SCALE,
    },
    chunk::Chunk,
};

type InterpolatedNoiseMap = [[f64; SPACES]; SPACES];
type NoiseMap = [[f64; CHUNK_SIZE]; CHUNK_SIZE];

const SPACES: usize = CHUNK_SIZE.div_ceil(NOISE_INTERPOLATION);

/// Converts a local chunk X coordinate and chunk coordinates into a world X coordinate.
pub fn local_to_world_x(a: usize, chunk_coords: IVec2) -> i32 {
    chunk_coords.x * (CHUNK_SIZE as i32) + (a as i32)
}

/// Converts a local chunk Z coordinate and chunk coordinates into a world Z coordinate.
pub fn local_to_world_z(a: usize, chunk_coords: IVec2) -> i32 {
    chunk_coords.y * (CHUNK_SIZE as i32) + (a as i32)
}

/// Generates terrain blocks for the chunk at the given chunk coordinates.
pub fn generate(chunk_pos: IVec2) -> Chunk {
    let perlin = Perlin::new(SEED);

    let low_freq_noises: NoiseMap = get_noises(&perlin, chunk_pos, LOW_FREQUENCY);
    let med_freq_noises: NoiseMap = get_noises(&perlin, chunk_pos, MED_FREQUENCY);
    let high_freq_noises: NoiseMap = get_noises(&perlin, chunk_pos, HIGH_FREQUENCY);

    let mut chunk = Chunk::new();

    #[allow(clippy::needless_range_loop)]
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let low_noise_value = low_freq_noises[x][z];
            let med_noise_value = med_freq_noises[x][z];
            let high_noise_value = high_freq_noises[x][z];

            let low_freq_height = BASE_HEIGHT + low_noise_value * LOW_AMPLITUDE;
            let med_freq_height = med_noise_value * MED_AMPLITUDE;
            let high_freq_height = high_noise_value * HIGH_AMPLITUDE;

            let height = low_freq_height + med_freq_height + high_freq_height;

            let height = height.min((CHUNK_HEIGHT as f64) - 1.0).round() as usize;
            chunk.fill(x, 0, z, x, height, z);
        }
    }

    chunk
}

/// Returns all chunk coordinates in a square centered on `chunk_pos`.
///
/// The square extends `radius` chunks in each horizontal direction, inclusive.
pub fn get_chunk_positions(chunk_pos: IVec2, radius: i32) -> HashSet<IVec2> {
    let mut chunks = HashSet::with_capacity((radius * radius * 4) as usize);

    for x in -radius..=radius {
        for z in -radius..=radius {
            chunks.insert(IVec2::new(chunk_pos.x + x, chunk_pos.y + z));
        }
    }

    chunks
}

/// Converts a world-space X / Z position into its containing chunk coordinates.
pub fn world_to_chunk_coords(player_position: Vec3) -> IVec2 {
    IVec2::new(
        (player_position.x.floor() as i32).div_euclid(CHUNK_SIZE as i32),
        (player_position.z.floor() as i32).div_euclid(CHUNK_SIZE as i32)
    )
}

/// Samples Perlin noise on a sparse grid and bilinearly interpolates it per block.
fn get_noises(perlin: &Perlin, chunk_pos: IVec2, freq: f64) -> NoiseMap {
    let mut samples: InterpolatedNoiseMap = [[0.0; SPACES]; SPACES];

    #[allow(clippy::needless_range_loop)]
    for x in 0..SPACES {
        for z in 0..SPACES {
            let world_x = local_to_world_x(x * NOISE_INTERPOLATION, chunk_pos);
            let world_z = local_to_world_z(z * NOISE_INTERPOLATION, chunk_pos);

            let noise_value = perlin.get([
                (world_x as f64) / WORLD_SCALE / freq,
                (world_z as f64) / WORLD_SCALE / freq,
            ]);
            samples[x][z] = noise_value;
        }
    }

    let mut noises: NoiseMap = [[0.0; CHUNK_SIZE]; CHUNK_SIZE];

    #[allow(clippy::needless_range_loop)]
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let sample_x_min = x / NOISE_INTERPOLATION;
            let sample_z_min = z / NOISE_INTERPOLATION;
            let sample_x_max = (x + NOISE_INTERPOLATION - 1)
                .div_ceil(NOISE_INTERPOLATION)
                .min(SPACES - 1);
            let sample_z_max = (z + NOISE_INTERPOLATION - 1)
                .div_ceil(NOISE_INTERPOLATION)
                .min(SPACES - 1);

            let point_min = IVec2::new(
                (sample_x_min * NOISE_INTERPOLATION) as i32,
                (sample_z_min * NOISE_INTERPOLATION) as i32
            );
            let point_max = IVec2::new(
                (sample_x_max * NOISE_INTERPOLATION) as i32,
                (sample_z_max * NOISE_INTERPOLATION) as i32
            );

            let noise_value = bilinearly_interpolate(
                point_min,
                point_max,
                samples[sample_x_min][sample_z_min],
                samples[sample_x_max][sample_z_min],
                samples[sample_x_min][sample_z_max],
                samples[sample_x_max][sample_z_max],
                IVec2::new(x as i32, z as i32)
            );

            noises[x][z] = noise_value;
        }
    }

    noises
}

/// Interpolates a value at `target` from the four values surrounding it.
fn bilinearly_interpolate(
    point_min: IVec2,
    point_max: IVec2,
    y_00: f64,
    y_10: f64,
    y_01: f64,
    y_11: f64,
    target: IVec2
) -> f64 {
    let width = point_max.x - point_min.x;
    let depth = point_max.y - point_min.y;

    let u = if width == 0 {
        0.0
    } else {
        (((target.x - point_min.x) as f64) / (width as f64)).clamp(0.0, 1.0)
    };
    let v = if depth == 0 {
        0.0
    } else {
        (((target.y - point_min.y) as f64) / (depth as f64)).clamp(0.0, 1.0)
    };

    let y_z_min = y_00 * (1.0 - u) + y_10 * u;
    let y_z_max = y_01 * (1.0 - u) + y_11 * u;

    y_z_min * (1.0 - v) + y_z_max * v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_chunk_does_not_panic() {
        let _ = generate(IVec2::ZERO);
    }

    #[test]
    fn local_to_world_z_uses_chunk_z_coordinate() {
        assert_eq!(local_to_world_z(7, IVec2::new(2, 3)), 3 * (CHUNK_SIZE as i32) + 7);
    }

    #[test]
    fn bilinear_interpolation_uses_depth_from_y_axis() {
        let value = bilinearly_interpolate(
            IVec2::new(1, 2),
            IVec2::new(3, 6),
            0.0,
            10.0,
            20.0,
            30.0,
            IVec2::new(2, 5)
        );

        assert!((value - 20.0).abs() < 0.0001);
    }
}
