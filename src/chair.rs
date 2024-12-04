use bevy::prelude::*;

use crate::Player;

#[derive(Event)]
pub struct SitEvent {
    pub actor: Entity,
    pub target: Entity,
}

#[derive(Event)]
pub struct StandEvent {
    pub actor: Entity,
    pub target: Entity,
}

#[derive(Component)]
pub struct Chair;

pub struct ChairPlugin;

impl Plugin for ChairPlugin {
    fn build(&self, app: &mut App) {
    }
}

fn sit_event_handler(
    mut commands: Commands,
    mut events: EventReader<SitEvent>,
    mut actors: Query<Entity, With<Player>>,
) {
    for event in events.read() {
    }
    if let Ok(player) = actors.get_single_mut() {
        commands.entity(player);
           // .remove::<>()
    }
}

fn stand_event_handler(
    mut commands: Commands,
    mut events: EventReader<StandEvent>,
    mut actors: Query<Entity, With<Player>>,
) {
    for event in events.read() {
    }
}
