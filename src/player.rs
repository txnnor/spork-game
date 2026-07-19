use bevy::prelude::*;

use crate::{
    components::{MainCamera, Player},
    constants::PLAYER_SPEED,
};

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
    mut player_query: Query<&mut Transform, With<Player>>,
    camera_query: Query<&Transform, (With<MainCamera>, Without<Player>)>,
) {
    let mut player = player_query.single_mut().unwrap();
    let camera = camera_query.single().unwrap();

    let mut forward = *camera.forward();

    forward.y = 0.0;
    forward = forward.normalize();

    let mut right = *camera.right();

    right.y = 0.0;
    right = right.normalize();

    let mut movement = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        movement += forward;
    }

    if keyboard.pressed(KeyCode::KeyS) {
        movement -= forward;
    }

    if keyboard.pressed(KeyCode::KeyA) {
        movement -= right;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        movement += right;
    }

    movement = movement.normalize_or_zero();

    player.translation += movement * PLAYER_SPEED * time.delta_secs();
}
