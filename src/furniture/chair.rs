use avian3d::prelude::RigidBodyDisabled;
use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{Interactable, InteractionEvent, Player, PlayerState};

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_chair_add)]
#[require(
    Interactable,
)]
pub struct Chair;

pub struct ChairPlugin;

impl Plugin for ChairPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Chair>();
    }
}

fn on_chair_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    trace!("HOOK: on_chair_add");
    world.commands()
        .entity(context.entity)
        .observe(chair_interaction_observer);
}

fn chair_interaction_observer(
    trigger: On<InteractionEvent>,
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut PlayerState, Entity), With<Player>>,
    transform_query: Query<&GlobalTransform, Without<Player>>,
) {
    trace!("OBSERVER: chair_interaction_observer");
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

