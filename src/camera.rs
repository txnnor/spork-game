use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{
    components::{CameraController, MainCamera, Player},
    constants::CAMERA_DISTANCE,
};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        CameraController {
            yaw: 0.0,
            pitch: -0.5,
        },
        Camera3d::default(),
        Transform::default(),
    ));
}

pub fn mouse_look(
    mut mouse_events: MessageReader<MouseMotion>,
    mut camera_query: Query<&mut CameraController, With<MainCamera>>,
) {
    let mut controller = camera_query.single_mut().unwrap();

    for event in mouse_events.read() {
        controller.yaw -= event.delta.x * 0.003;
        controller.pitch -= event.delta.y * 0.003;

        controller.pitch = controller.pitch.clamp(-1.2, 1.2);
    }
}

pub fn follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<
        (&mut Transform, &CameraController),
        (With<MainCamera>, Without<Player>),
    >,
) {
    let player = player_query.single().unwrap();

    let (mut camera, controller) = camera_query.single_mut().unwrap();

    let direction = Vec3::new(
        controller.yaw.sin() * controller.pitch.cos(),
        controller.pitch.sin(),
        controller.yaw.cos() * controller.pitch.cos(),
    );

    camera.translation = player.translation - direction * CAMERA_DISTANCE;
    camera.look_at(player.translation, Vec3::Y);
}
