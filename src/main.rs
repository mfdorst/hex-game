use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod camera_movement;
mod cursor;
mod map_gen;
mod player;

use camera_movement::CameraMovementPlugin;
use cursor::CursorTrackingPlugin;
use map_gen::MapGenPlugin;
use player::PlayerPlugin;

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
        .add_plugin(CameraMovementPlugin)
        .add_plugin(MapGenPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CursorTrackingPlugin)
        .run();
}
