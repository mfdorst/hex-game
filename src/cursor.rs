use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy_ecs_tilemap::{
    prelude::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage},
};

pub struct CursorTrackingPlugin;

impl Plugin for CursorTrackingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPos>()
            .init_resource::<HoveredTile>()
            .add_system(update_cursor_pos.in_base_set(CoreSet::First))
            .add_system(update_hovered_tile)
            .add_system(print_hovered_tile_coords);
    }
}

#[derive(Resource, Default)]
pub struct CursorPos(Vec2);
#[derive(Resource, Debug, Default)]
pub struct HoveredTile(TilePos);

fn update_cursor_pos(
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    for event in cursor_moved_events.iter() {
        for (xform, camera) in &camera_q {
            if let Some(pos) = camera.viewport_to_world_2d(xform, event.position) {
                *cursor_pos = CursorPos(pos);
            }
        }
    }
}

fn update_hovered_tile(
    mut hovered_tile: ResMut<HoveredTile>,
    cursor_pos: Res<CursorPos>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
) {
    for (map_size, grid_size, map_type, tile_storage, map_xform) in &tilemap_q {
        let cursor_pos = cursor_pos.0;
        let cursor_in_map_pos: Vec2 = {
            let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
            let cursor_in_map_pos = map_xform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };
        if let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
        {
            *hovered_tile = HoveredTile(tile_pos);
        }
    }
}

fn print_hovered_tile_coords(hovered_tile: Res<HoveredTile>) {
    eprintln!("{hovered_tile:?}");
}
