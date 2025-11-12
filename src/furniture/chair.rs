use avian3d::prelude::RigidBodyDisabled;
use bevy::prelude::*;

use crate::{Interactable, InteractionEvent, Player, PlayerState};

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Chair;

pub struct ChairPlugin;

impl Plugin for ChairPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Chair>()
            .add_systems(Update, register_chair_interactions);
    }
}

fn register_chair_interactions(
    mut commands: Commands,
    mut chairs_query: Query<Entity, (With<Chair>, Without<Interactable>)>,
) {
    for chair in chairs_query.iter_mut() {
        commands.entity(chair).observe(chair_interaction_observer)
            .insert(Interactable);
    }
}

fn chair_interaction_observer(
    trigger: On<InteractionEvent>,
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut PlayerState, Entity), With<Player>>,
    transform_query: Query<&GlobalTransform, Without<Player>>,
) {
    if let Ok((mut player_transform, mut player_state, player_entity)) = player_query.single_mut()
        && let Ok(chair_transform) = transform_query.get(trigger.entity) {
            *player_transform = Transform {
                translation: Vec3 { x: chair_transform.translation().x, y: chair_transform.translation().y + 1.0, z: chair_transform.translation().z },
                rotation: chair_transform.rotation(),
                ..default()
            };
            *player_state = PlayerState::Sitting;
            commands.entity(player_entity).insert(RigidBodyDisabled);
    }
}

