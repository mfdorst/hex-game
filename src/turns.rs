use bevy::prelude::*;

use crate::player::{CurrentPlayer, MoveMadeEvent, PlayerRemainingMoves};

pub struct TurnsPlugin;

impl Plugin for TurnsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(count_moves);
    }
}

fn count_moves(
    mut move_made_ev: EventReader<MoveMadeEvent>,
    mut remaining_moves_q: Query<&mut PlayerRemainingMoves, With<CurrentPlayer>>,
) {
    for _ in &mut move_made_ev {
        let mut remaining_moves = remaining_moves_q.single_mut();
        remaining_moves.0 -= 1;
        eprintln!("{:?}", remaining_moves.0);
    }
}
