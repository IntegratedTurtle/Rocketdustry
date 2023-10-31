use crate::camera;
use bevy::{prelude::*, transform};

/// Defines how fast the Player is
const PLAYERSPEED: f32 = 500.0;
/// How fast the Camera will follow the player
const CAMERAFOLLOWSPEED: f32 = 2.0;

/// In this struct Information can be collected that will be used to spawn the player
#[derive(Resource)]
pub struct PlayerSpawnInfo {
    location: Vec3,
}
impl Default for PlayerSpawnInfo {
    fn default() -> Self {
        PlayerSpawnInfo {
            location: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_spawn_info: Res<PlayerSpawnInfo>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                player_spawn_info.location.x,
                player_spawn_info.location.y,
                player_spawn_info.location.z,
            ),
            texture: asset_server.load("sprites/Drone.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        transform.look_to(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction * time.delta_seconds(),
        );
        transform.translation += direction * PLAYERSPEED * time.delta_seconds();
    }
}

pub fn camera_follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut camera) = camera_query.get_single_mut() {
            let distance_x = player.translation.x - camera.translation.x;
            let distance_y = player.translation.y - camera.translation.y;
            camera.translation.x += distance_x * time.delta_seconds() * CAMERAFOLLOWSPEED;
            camera.translation.y += distance_y * time.delta_seconds() * CAMERAFOLLOWSPEED;
        }
    }
}
