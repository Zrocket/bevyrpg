use avian3d::spatial_query::{SpatialQuery, SpatialQueryFilter};
use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{ApplicatorSubstance, UseRoverAttachmentEvent, rover::attachment::applicator};

#[derive(Component)]
#[require(
    Name::new("Applicator"),
    crate::RoverAttachment,
)]
#[component(on_add = on_applicator_add)]
pub struct ApplicatorAttachment(pub Option<Entity>);

fn on_applicator_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(use_appliicator_observer)
        .observe(switch_applicator_substance_observer);
}

#[derive(EntityEvent)]
pub struct ApplicatorEvent{ pub entity: Entity }

#[derive(EntityEvent)]
pub struct SwitchApplicatorSubstanceEvent{
    pub entity: Entity,
    pub substance: Entity,
}

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
    let Ok(rover_entity) = rover_query.single() else {
        error!("use_appliicator_observer: Failed to query Entity for Rover");
        return;
    };
    let Ok(camera_transform) = transform_query.single() else {
        error!("use_appliicator_observer: Failed to query GlobalTransform for RoverCamera");
        return;
    };

    //if let Ok(rover_entity) = rover_query.single()
    //&& let Ok(camera_transform) = transform_query.single() {
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
    //}
}

fn switch_applicator_substance_observer(
    trigger: On<SwitchApplicatorSubstanceEvent>,
    mut applicator_query: Query<&mut ApplicatorAttachment>,
    substance_query: Query<Entity, With<ApplicatorSubstance>>,
) {
    let Ok(mut applicator) = applicator_query.single_mut() else {
        error!("switch_applicator_substance_observer: Failed to query ApplicatorAttachment");
        return;
    };
    let Ok(substance) = substance_query.get(trigger.substance) else {
        error!("switch_applicator_substance_observer: Failed to query Entity for {}", trigger.substance);
        return;
    };

    //if let Ok(mut applicator) = applicator_query.single_mut()
    //&& let Ok(substance) = substance_query.get(trigger.substance) {
    applicator.0 = Some(substance);
    //}
}
