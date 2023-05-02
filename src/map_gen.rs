use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct MapGenPlugin;

impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileHandle>()
            .add_startup_systems((generate_map, apply_system_buffers).chain());
    }
}

pub const MAP_SIDE_LENGTH: u32 = 32;
pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 58.0, y: 50.0 };
pub const GRID_SIZE: TilemapGridSize = TilemapGridSize { x: 58.0, y: 50.0 };
pub const COORD_SYSTEM: HexCoordSystem = HexCoordSystem::ColumnEven;
pub const MAP_TYPE: TilemapType = TilemapType::Hexagon(COORD_SYSTEM);
pub const MAP_SIZE: TilemapSize = TilemapSize {
    x: MAP_SIDE_LENGTH,
    y: MAP_SIDE_LENGTH,
};

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

    let mut tile_storage = TileStorage::empty(MAP_SIZE);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    fill_tilemap_hexagon(
        TileTextureIndex(0),
        TilePos {
            x: MAP_SIDE_LENGTH / 2,
            y: MAP_SIDE_LENGTH / 2,
        },
        MAP_SIDE_LENGTH / 2,
        COORD_SYSTEM,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: GRID_SIZE,
        size: MAP_SIZE,
        storage: tile_storage,
        texture: TilemapTexture::Single(tile_handle_hex_col.clone()),
        tile_size: TILE_SIZE,
        map_type: MAP_TYPE,
        transform: get_tilemap_center_transform(&MAP_SIZE, &GRID_SIZE, &MAP_TYPE, 0.0),
        ..Default::default()
    });
}
