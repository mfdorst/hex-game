use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::cursor::HoveredTile;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_players)
            .add_event::<MoveMadeEvent>()
            .add_system(select_tile);
    }
}

pub struct MoveMadeEvent;

#[derive(Component, Default)]
pub struct PlayerColor(pub Color);
#[derive(Component, Default, Clone)]
pub struct PlayerId(pub u32);
#[derive(Component, Default)]
pub struct PlayerRemainingMoves(pub u32);
#[derive(Component)]
pub struct CurrentPlayer;

#[derive(Bundle, Default)]
struct PlayerBundle {
    order: PlayerId,
    color: PlayerColor,
    remaining_moves: PlayerRemainingMoves,
}
fn setup_players(mut commands: Commands) {
    commands
        .spawn(PlayerBundle {
            order: PlayerId(1),
            color: PlayerColor(Color::RED),
            remaining_moves: PlayerRemainingMoves(5),
        })
        .insert(CurrentPlayer);
}

fn select_tile(
    player_q: Query<(&PlayerId, &PlayerColor, &PlayerRemainingMoves), With<CurrentPlayer>>,
    hovered_tile: Res<HoveredTile>,
    buttons: Res<Input<MouseButton>>,
    mut move_made_ev: EventWriter<MoveMadeEvent>,
    mut tile_storage_q: Query<&mut TileStorage>,
    mut tile_q: Query<(Entity, &mut TileColor, Option<&PlayerId>), With<TilePos>>,
    mut commands: Commands,
) {
    if buttons.just_released(MouseButton::Left) {
        let (player_id, player_color, remaining_moves) = player_q.single();
        if remaining_moves.0 == 0 {
            return;
        }
        let tile_storage = tile_storage_q.single_mut();
        let Some(hovered_tile) = hovered_tile.0 else { return };
        let Some(tile_entity) = tile_storage.checked_get(&hovered_tile) else { return };
        let Ok((tile_entity, mut tile_color, tile_player_id)) = tile_q.get_mut(tile_entity) else { return };
        if let Some(tile_player_id) = tile_player_id {
            if tile_player_id.0 == player_id.0 {
                return;
            }
        }
        *tile_color = player_color.0.into();
        commands.entity(tile_entity).insert(player_id.clone());
        move_made_ev.send(MoveMadeEvent);
    }
}
