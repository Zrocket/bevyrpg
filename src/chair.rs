use bevy::{prelude::*, transform};

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

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Chair;

pub struct ChairPlugin;

impl Plugin for ChairPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Chair>()
            .add_event::<SitEvent>();
    }
}

fn sit_event_handler(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    transform_query: Query<&Transform>,
    mut events: EventReader<SitEvent>,
    mut actors: Query<Entity, With<Player>>,
) {
    for event in events.read() {
        if let Ok(mut player_transform) = player_query.get_single_mut() {
            if let Ok(chair_transform) = transform_query.get(event.target) {
                player_transform = chair_transform;
            }
        }
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
    for event in events.read() {}
}
