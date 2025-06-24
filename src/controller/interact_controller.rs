use crate::interact::Interaction;
use crate::player::Player;
use crate::{Inspectable, RESOLUTION_HEIGHT, RESOLUTION_WIDTH};
use avian3d::prelude::*;
use avian_pickup::{
    input::AvianPickupInput,
    prop::HeldProp,
};
use bevy::prelude::*;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct RayHit(pub Entity);

pub fn player_raycast(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform), Without<HeldProp>>,
    ray_caster: SpatialQuery,
    mut player: Query<(Entity, &mut RayHit), With<Player>>,
) {
    trace!("SYSTEM: player_raycast");
    if let Ok((player, mut ray_hit)) = player.single_mut() {
        for (camera, global_transform) in camera_query.iter() {
            let center_window = camera.viewport_to_world(global_transform, Vec2 { y: RESOLUTION_HEIGHT / 2., x: RESOLUTION_WIDTH / 2. }).unwrap();
            let camera_position = global_transform.translation();
            let camera_direction = global_transform.forward();
            if let Some(ray_data) = ray_caster.cast_ray(
                center_window.origin,
                center_window.direction,
                500.0,
                true,
                &SpatialQueryFilter::default().with_excluded_entities([player]),
            ) {
                //info!("interact ray casted");
                let ray_hit_point = camera_position + camera_direction * ray_data.distance;
                //info!(
                //    "INTERACT Entity {:?} hit at point {}, from {}",
                //    ray_data.entity, ray_hit_point, camera_position
                //);
                ray_hit.0 = ray_data.entity;
            }
        }
    }
}

pub fn manage_interact(
    mut commands: Commands,
    ray_caster: SpatialQuery,
    player: Query<(Entity, &RayHit), With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform), Without<HeldProp>>,
    interact_query: Query<&dyn Interaction>,
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
    held_prop_query: Query<&HeldProp>,
) {
    trace!("SYSTEM: manage_interact");
    if let Ok((player, ray_hit)) = player.single() {
        //info!("got plalyer");
        if let Ok(_held_prop) = held_prop_query.single() {
            avian_pickup_input_writer.write(
                AvianPickupInput { actor: player, action: avian_pickup::input::AvianPickupAction::Drop }
            );
            return
        }
        if let Ok(interaction) = interact_query.get(ray_hit.0) {
            //info!("ray interaction");
            for act in interaction.iter() {
                act.interact(&mut commands, player, ray_hit.0);
            }
        }
    }
}

pub fn manage_inspect(
    mut commands: Commands,
    ray_caster: SpatialQuery,
    player: Query<(Entity, &RayHit), With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform), Without<HeldProp>>,
    inspection_query: Query<&dyn Inspectable>,
    held_prop_query: Query<&HeldProp>,
) {
    trace!("SYSTEM: manage_inspect");
    if let Ok((player, ray_hit)) = player.single() {
        //info!("got player");

        if let Ok(_held_prop) = held_prop_query.single() {
            return;
        }
        if let Ok(inspection) = inspection_query.get(ray_hit.0) {
            //info!("ray inspection");
            for act in inspection.iter() {
                act.inspect(&mut commands, player, ray_hit.0);
            }
        }
    }
}
