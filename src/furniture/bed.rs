use std::time::Duration;

use avian3d::prelude::RigidBodyDisabled;
use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{CameraInterpolation, InteractionEvent, Player, PlayerCamera, PlayerState};

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require()]
#[component(on_add = on_bed_add)]
pub struct Bed;

fn on_bed_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(bed_interaction_observer);
}

fn bed_interaction_observer(
    trigger: On<InteractionEvent>,
    mut commands: Commands,
    bed_query: Query<&GlobalTransform, With<Bed>>,
    mut player_query: Query<(Entity, &mut Transform, &mut PlayerState), With<Player>>,
    camera_query: Query<Entity, With<PlayerCamera>>,
    time: Res<Time>,
) {
    trace!("OBSERVER: bed_interaction_observer");
    if let Ok(bed_global_transform) = bed_query.get(trigger.entity)
    && let Ok((player_entity, mut player_transform, mut player_state)) = player_query.single_mut()
    && let Ok(camera_entity) = camera_query.single() {
        *player_transform = Transform {
            translation: Vec3 {
                x: bed_global_transform.translation().x,
                y: bed_global_transform.translation().y,
                z: bed_global_transform.translation().z 
            },
            rotation: bed_global_transform.rotation(),
            ..default()
        };
        *player_state = PlayerState::Sleeping;
        commands.entity(player_entity).insert(RigidBodyDisabled);
        commands.entity(camera_entity)
            .insert(CameraInterpolation {
                duration: time.elapsed() + Duration::new(1, 0),
                start_time: time.elapsed(),
            });
    }
}

pub struct BedPlugin;
impl Plugin for BedPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<Bed>(); 
    }
}
