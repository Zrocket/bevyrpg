use bevy::prelude::*;

use crate::Player;

#[derive(Event)]
pub struct UseComputerEvent {
    pub target: Entity,
}

pub struct ComputerPlugin;

impl Plugin for ComputerPlugin {
    fn build(&self, _app: &mut App) {}
}

fn _use_computer_event_handler(_player_query: Query<Entity, With<Player>>) {}
