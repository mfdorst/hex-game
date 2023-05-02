use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn spawn_color_hex(
    position: TilePos,
    color: Color,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    let tile_entity = commands
        .spawn(TileBundle {
            position,
            tilemap_id,
            texture_index: TileTextureIndex(0),
            color: TileColor(color),
            ..default()
        })
        .id();
    tile_storage.set(&position, tile_entity);
}
