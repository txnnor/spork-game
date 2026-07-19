use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraController {
    pub yaw: f32,
    pub pitch: f32,
}
