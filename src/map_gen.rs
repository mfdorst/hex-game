use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct MapGen;

impl Plugin for MapGen {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileHandle>()
            .add_startup_systems((generate_map, apply_system_buffers).chain());
    }
}

const MAP_SIDE_LENGTH: u32 = 32;

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 58.0, y: 50.0 };
const GRID_SIZE: TilemapGridSize = TilemapGridSize { x: 58.0, y: 50.0 };

#[derive(Deref, Resource)]
pub struct TileHandle(Handle<Image>);

impl FromWorld for TileHandle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("bw-tile-hex-col.png"))
    }
}

fn generate_map(mut commands: Commands, tile_handle_hex_col: Res<TileHandle>) {
    commands.spawn(Camera2dBundle::default());

    let map_size = TilemapSize {
        x: MAP_SIDE_LENGTH,
        y: MAP_SIDE_LENGTH,
    };

    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    let hex_coord_system = HexCoordSystem::Column;

    fill_tilemap_hexagon(
        TileTextureIndex(0),
        TilePos {
            x: MAP_SIDE_LENGTH / 2,
            y: MAP_SIDE_LENGTH / 2,
        },
        MAP_SIDE_LENGTH / 2,
        hex_coord_system,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    let tile_size = TILE_SIZE;
    let grid_size = GRID_SIZE;
    let map_type = TilemapType::Hexagon(hex_coord_system);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(tile_handle_hex_col.clone()),
        tile_size,
        map_type,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}
