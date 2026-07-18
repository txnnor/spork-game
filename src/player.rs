use bevy::prelude::*;

use crate::{components::Player, constants::PLAYER_SPEED};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player,
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.9))),
        Transform::from_xyz(0.0, 1.0, 3.0),
    ));
}

pub fn player_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut player in &mut query {
        if keyboard.pressed(KeyCode::KeyW) {
            player.translation.z += PLAYER_SPEED * time.delta_secs()
        }
        if keyboard.pressed(KeyCode::KeyS) {
            player.translation.z -= PLAYER_SPEED * time.delta_secs()
        }
        if keyboard.pressed(KeyCode::KeyA) {
            player.translation.x += PLAYER_SPEED * time.delta_secs()
        }
        if keyboard.pressed(KeyCode::KeyD) {
            player.translation.x -= PLAYER_SPEED * time.delta_secs()
        }
    }
}
