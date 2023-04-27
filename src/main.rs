use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod camera_movement;
mod map_gen;

use camera_movement::CameraMovement;
use map_gen::MapGen;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Hex Game"),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(TilemapPlugin)
        .add_plugin(CameraMovement)
        .add_plugin(MapGen)
        .run();
}
