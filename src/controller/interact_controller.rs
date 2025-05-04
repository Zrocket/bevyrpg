use crate::interact::Interaction;
use crate::player::Player;
use crate::{HEIGHT, WIDTH};
use avian3d::prelude::*;
use avian_pickup::input::AvianPickupInput;
use avian_pickup::prop::HeldProp;
use bevy::prelude::*;

pub fn manage_interact(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    ray_caster: SpatialQuery,
    player: Query<Entity, With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform), Without<HeldProp>>,
    interact_query: Query<&dyn Interaction>,
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
    held_prop_query: Query<&HeldProp>,
) {
    if key.just_pressed(KeyCode::KeyE) {
        if let Ok(player) = player.get_single() {
            if let Ok(_held_prop) = held_prop_query.get_single() {
                avian_pickup_input_writer.send(
                    AvianPickupInput { actor: player, action: avian_pickup::input::AvianPickupAction::Drop }
                );
                return
            }
            for (camera, global_transform) in camera_query.iter() {
                let center_window = camera.viewport_to_world(global_transform, Vec2 { y: HEIGHT / 2., x: WIDTH / 2. }).unwrap();
                let camera_position = global_transform.translation();
                let camera_direction = global_transform.forward();
                if let Some(ray_data) = ray_caster.cast_ray(
                    center_window.origin,
                    //camera_position,
                    center_window.direction.into(),
                    500.0,
                    true,
                    &SpatialQueryFilter::default().with_excluded_entities([player]),
                ) {
                    let hit_point = camera_position + camera_direction * ray_data.distance;
                    info!(
                        "INTERACT Entity {:?} hit at point {}, from {}",
                        ray_data.entity, hit_point, camera_position
                    );
                    if let Ok(interaction) = interact_query.get(ray_data.entity) {
                        for act in interaction.iter() {
                            if let Some(temp) = act.interact(&mut commands, &player, &ray_data.entity) {
                                avian_pickup_input_writer.send(temp);
                            }
                        }
                    }
                }
            }
        }
    }
}
