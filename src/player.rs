use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub const PLAYER_1: Player = Player { color: Color::RED };
pub const PLAYER_2: Player = Player {
    color: Color::GREEN,
};
pub const PLAYER_3: Player = Player { color: Color::BLUE };

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {}
}

pub struct Player {
    color: Color,
}
