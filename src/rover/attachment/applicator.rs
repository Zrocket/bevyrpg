use avian3d::spatial_query::{SpatialQuery, SpatialQueryFilter};
use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::UseRoverAttachmentEvent;

#[derive(Component)]
#[require(
    Name::new("Applicator"),
)]
#[component(on_add = on_applicator_add)]
pub struct ApplicatorAttachment(pub Option<Entity>);

fn on_applicator_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(use_appliicator_observer);
}

#[derive(EntityEvent)]
pub struct ApplicatorEvent{ pub entity: Entity }

pub struct ApplicatorAttachmentPlugin;
impl Plugin for ApplicatorAttachmentPlugin {
    fn build(&self, app: &mut App) {
       app;
    }
}

fn use_appliicator_observer(
    _trigger: On<UseRoverAttachmentEvent>,
    mut commands: Commands,
    rover_query: Query<Entity, With<crate::Rover>>,
    transform_query: Query<&GlobalTransform, With<crate::RoverCamera>>,
    ray_caster: SpatialQuery,
) {
    if let Ok(rover_entity) = rover_query.single()
    && let Ok(camera_transform) = transform_query.single() {
        let camera_position = camera_transform.translation();
        let direction = camera_transform.forward().normalize();
        if let Some(ray_data) = ray_caster.cast_ray(
            camera_position,
            Dir3::new_unchecked(direction),
            5.0,
            true,
            &SpatialQueryFilter::default().with_excluded_entities([rover_entity])
        ) {
            commands.entity(ray_data.entity).trigger(|entity| ApplicatorEvent { entity });
        }
    }
}
