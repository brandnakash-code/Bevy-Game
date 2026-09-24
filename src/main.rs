use bevy::{ input::mouse::MouseMotion, prelude::*, window::{ CursorGrabMode, CursorOptions } };

use bevy_game::lib_root::chunk_gen::world_to_chunk_coords;
use bevy_game::lib_root::chunk_manager::ChunkManager;
use bevy_game::lib_root::block::Textures;

#[derive(Component)]
struct PlayerCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .init_resource::<ChunkManager>()
        .init_resource::<Textures>()
        .add_systems(Update, (
            camera_movement,
            update_visible_chunks,
            finish_chunk_generation,
            grab_mouse,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    let player_position = Vec3::new(10.0, 8.0, 10.0);

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(player_position).looking_at(Vec3::new(8.0, 0.0, 8.0), Vec3::Y),
        PlayerCamera,
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn update_visible_chunks(
    mut commands: Commands,
    player_query: Query<&Transform, With<PlayerCamera>>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut textures: ResMut<Textures>,
    asset_server: Res<AssetServer>
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_chunk = world_to_chunk_coords(player_transform.translation);

    if let Some(center) = chunk_manager.center() && center == player_chunk {
        return;
    }

    chunk_manager.set_center(
        player_chunk,
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut textures,
        &asset_server
    );
}

fn finish_chunk_generation(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut textures: ResMut<Textures>,
    asset_server: Res<AssetServer>
) {
    chunk_manager.poll_generation_tasks(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut textures,
        &asset_server
    );
}

fn camera_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut query: Query<&mut Transform, With<PlayerCamera>>
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction += *transform.forward();
    }

    if keyboard.pressed(KeyCode::KeyS) {
        direction += *transform.back();
    }

    if keyboard.pressed(KeyCode::KeyA) {
        direction += *transform.left();
    }

    if keyboard.pressed(KeyCode::KeyD) {
        direction += *transform.right();
    }

    if keyboard.pressed(KeyCode::Space) {
        direction += Vec3::Y;
    }

    if keyboard.pressed(KeyCode::ShiftLeft) {
        direction -= Vec3::Y;
    }

    if direction != Vec3::ZERO {
        transform.translation += direction.normalize() * 0.15;
    }

    // Very basic mouse look
    for motion in mouse_motion.read() {
        let sensitivity = 0.003;

        transform.rotate_y(-motion.delta.x * sensitivity);
        transform.rotate_local_x(-motion.delta.y * sensitivity);
    }
}

fn grab_mouse(
    mut cursor_options: Single<&mut CursorOptions>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>
) {
    // Grab the mouse on left click
    if mouse.just_pressed(MouseButton::Left) {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    // Release the mouse on Escape
    if key.just_pressed(KeyCode::Escape) {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}
