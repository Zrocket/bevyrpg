use std::time::Duration;

use avian3d::prelude::RigidBodyDisabled;
use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{CameraInterpolation, Interactable, InteractionEvent, Player, PlayerCamera, PlayerState};

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_chair_add)]
#[require(
    Interactable,
)]
#[type_path("api")]
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
    camera_query: Query<Entity, With<PlayerCamera>>,
    time: Res<Time>,
) {
    trace!("OBSERVER: chair_interaction_observer");
    if let Ok((mut player_transform, mut player_state, player_entity)) = player_query.single_mut()
        && let Ok(chair_transform) = transform_query.get(trigger.entity)
        && let Ok(camera_entity) = camera_query.single() {
            *player_transform = Transform {
                translation: Vec3 { x: chair_transform.translation().x, y: chair_transform.translation().y + 1.0, z: chair_transform.translation().z },
                rotation: chair_transform.rotation(),
                ..default()
            };
            *player_state = PlayerState::Sitting;
            commands.entity(player_entity).insert(RigidBodyDisabled);
            commands.entity(camera_entity)
                .insert(CameraInterpolation {
                    duration: time.elapsed() + Duration::new(1, 0),
                    start_time: time.elapsed(),
                });
    }
}

