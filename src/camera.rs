use bevy::prelude::*;

use crate::{
    components::{Camera, Player},
    constants::CAMERA_DISTANCE,
};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera,
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player = player_query.single().unwrap();
    let mut camera = camera_query.single_mut().unwrap();

    camera.translation = player.translation + Vec3::new(0.0, CAMERA_DISTANCE, -5.0);
    camera.look_at(player.translation, Vec3::Y);
}
