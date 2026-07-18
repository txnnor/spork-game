mod camera;
mod components;
mod constants;
mod player;
mod world;

use bevy::prelude::*;

use crate::{
    camera::{follow_player, spawn_camera},
    player::{player_movement, spawn_player},
    world::setup_world,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_world, spawn_camera, spawn_player))
        .add_systems(Update, (player_movement, follow_player).chain())
        .run();
}
