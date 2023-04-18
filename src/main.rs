use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

// Side length of a colored quadrant (in number of tiles)
const QUADRANT_SIDE_LENGTH: u32 = 80;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 10.0,
            ..default()
        },
        ..default()
    });

    let texture_handle: Handle<Image> = asset_server.load("hex.png");

    let map_size = TilemapSize {
        x: QUADRANT_SIDE_LENGTH * 2,
        y: QUADRANT_SIDE_LENGTH * 2,
    };
    let quadrant_size = TilemapSize {
        x: QUADRANT_SIDE_LENGTH,
        y: QUADRANT_SIDE_LENGTH,
    };

    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    fill_tilemap_rect(
        TileTextureIndex(0),
        TilePos { x: 0, y: 0 },
        quadrant_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    fill_tilemap_rect(
        TileTextureIndex(1),
        TilePos {
            x: QUADRANT_SIDE_LENGTH,
            y: 0,
        },
        quadrant_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    fill_tilemap_rect(
        TileTextureIndex(2),
        TilePos {
            x: 0,
            y: QUADRANT_SIDE_LENGTH,
        },
        quadrant_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );
    fill_tilemap_rect(
        TileTextureIndex(3),
        TilePos {
            x: QUADRANT_SIDE_LENGTH,
            y: QUADRANT_SIDE_LENGTH,
        },
        quadrant_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    let tile_size = TilemapTileSize { x: 587.0, y: 512.0 };
    let grid_size = TilemapGridSize { x: 587.0, y: 512.0 };
    let map_type = TilemapType::Hexagon(HexCoordSystem::Column);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        map_type,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..default()
    });
}

fn swap_mesh_type(
    mut query: Query<(
        &mut Transform,
        &TilemapSize,
        &TilemapGridSize,
        &mut TilemapType,
    )>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (mut transform, map_size, grid_size, mut map_type) in query.iter_mut() {
            match *map_type {
                TilemapType::Hexagon(HexCoordSystem::Column) => {
                    *map_type = TilemapType::Hexagon(HexCoordSystem::ColumnEven);
                }
                TilemapType::Hexagon(HexCoordSystem::ColumnEven) => {
                    *map_type = TilemapType::Hexagon(HexCoordSystem::ColumnOdd);
                }
                TilemapType::Hexagon(HexCoordSystem::ColumnOdd) => {
                    *map_type = TilemapType::Hexagon(HexCoordSystem::Column);
                }
                _ => {}
            }

            *transform = get_tilemap_center_transform(map_size, grid_size, &map_type, 0.0);
        }
    }
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += 1.0;
        }
        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= 1.0; 
        }

        if ortho.scale < 10.0 {
            ortho.scale = 10.0;
        }

        // It is important not to change z, as it can mess with how the layers are shown.
        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 10000.0;
        transform.translation.z = z;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hexagon Column Example".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup)
        .add_system(camera_movement)
        .add_system(swap_mesh_type)
        .run();
}
