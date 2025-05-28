use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

use crate::{interact::Interaction, Player};

#[derive(Event)]
pub struct SitEvent {
    pub actor: Entity,
    pub target: Entity,
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Chair;

impl Interaction for Chair {
    fn interact(
        &self,
        commands: &mut Commands,
        entity: Entity,
        prop: Entity,
        ) {
        commands.trigger_targets(SitEvent {actor: entity, target: prop}, prop);
    }
}

pub struct ChairPlugin;

impl Plugin for ChairPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Chair>()
            .register_component_as::<dyn Interaction, Chair>()
            .add_event::<SitEvent>()
            .add_observer(sit_event_handler);
    }
}

fn sit_event_handler(
    trigger: Trigger<SitEvent>,
    mut commands: Commands,
    mut player_query: Query<&mut Transform, With<Player>>,
    transform_query: Query<&Transform, Without<Player>>,
) {
    if let Ok(mut player_transform) = player_query.single_mut() &&
        let Ok(chair_transform) = transform_query.get(trigger.target) {
            *player_transform = Transform {
                translation: Vec3 { x: chair_transform.translation.x, y: chair_transform.translation.y + 1.0, z: chair_transform.translation.z },
                rotation: chair_transform.rotation,
                ..default()
            };
    }
}
