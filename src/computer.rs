use bevy::prelude::*;

use crate::Player;

#[derive(Event)]
pub struct UseComputerEvent {
    pub target: Entity,
}

pub struct ComputerPlugin;

impl Plugin for ComputerPlugin {
    fn build(&self, app: &mut App) {}
}

fn use_computer_event_handler(player_query: Query<Entity, With<Player>>) {}
