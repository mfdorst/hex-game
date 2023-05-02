use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::cursor::HoveredTile;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentPlayer(Player { color: Color::RED }))
            .add_system(select_tile);
    }
}

struct Player {
    color: Color,
}
#[derive(Resource)]
struct CurrentPlayer(Player);

fn select_tile(
    current_player: Res<CurrentPlayer>,
    hovered_tile: Res<HoveredTile>,
    buttons: Res<Input<MouseButton>>,
    mut tile_storage_q: Query<&mut TileStorage>,
    mut tile_color_q: Query<&mut TileColor, With<TilePos>>,
) {
    if buttons.just_released(MouseButton::Left) {
        let tile_storage = tile_storage_q.single_mut();
        let Some(hovered_tile) = hovered_tile.0 else { return };
        let Some(tile_entity) = tile_storage.checked_get(&hovered_tile) else { return };
        let Ok(mut tile_color) = tile_color_q.get_mut(tile_entity) else { return };
        *tile_color = current_player.0.color.into();
    }
}
