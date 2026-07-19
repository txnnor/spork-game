mod camera;
mod components;
mod constants;
mod player;
mod world;

use bevy::prelude::*;

use crate::{
    camera::{follow_player, mouse_look, spawn_camera},
    player::{player_movement, spawn_player},
    world::setup_world,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (lock_cursor, setup_world, spawn_camera, spawn_player),
        )
        .add_systems(Update, (player_movement, follow_player, mouse_look).chain())
        .run();
}

use bevy::window::{CursorGrabMode, CursorOptions};

pub fn lock_cursor(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
}
