use crate::player::Player;
use crate::{InspectEvent, InteractionEvent, RESOLUTION_HEIGHT, RESOLUTION_WIDTH};
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
    camera_query: Query<(&Camera, &GlobalTransform), Without<HeldProp>>,
    ray_caster: SpatialQuery,
    mut player: Query<(Entity, &mut RayHit), With<Player>>,
) {
    trace!("SYSTEM: player_raycast");
    if let Ok((player, mut ray_hit)) = player.single_mut() {
        for (camera, global_transform) in camera_query.iter() {
            let center_window = camera.viewport_to_world(global_transform, Vec2 { y: (RESOLUTION_HEIGHT / 2) as f32, x: (RESOLUTION_WIDTH / 2) as f32 }).unwrap();
            let camera_position = global_transform.translation();
            let camera_direction = global_transform.forward();
            if let Some(ray_data) = ray_caster.cast_ray(
                center_window.origin,
                center_window.direction,
                500.0,
                true,
                &SpatialQueryFilter::default().with_excluded_entities([player]),
            ) {
                let _ray_hit_point = camera_position + camera_direction * ray_data.distance;
                ray_hit.0 = ray_data.entity;
            }
        }
    }
}

pub fn manage_interact(
    mut commands: Commands,
    player: Query<(Entity, &RayHit), With<Player>>,
    mut avian_pickup_input_writer: MessageWriter<AvianPickupInput>,
    held_prop_query: Query<&HeldProp>,
) {
    trace!("SYSTEM: manage_interact");
    if let Ok((player, ray_hit)) = player.single() {
        if let Ok(_held_prop) = held_prop_query.single() {
            avian_pickup_input_writer.write(
                AvianPickupInput { actor: player, action: avian_pickup::input::AvianPickupAction::Drop }
            );
            return
        }
        commands.entity(ray_hit.0).trigger(|entity| InteractionEvent { entity, actor: player });
    }
}

pub fn manage_inspect(
    mut commands: Commands,
    player: Query<(Entity, &RayHit), With<Player>>,
    held_prop_query: Query<&HeldProp>,
) {
    trace!("SYSTEM: manage_inspect");
    if let Ok(_held_prop) = held_prop_query.single() {
        return;
    }
    if let Ok((player, ray_hit)) = player.single() {
        commands.entity(ray_hit.0).trigger(|entity| InspectEvent { entity, actor: player });
    }
}
