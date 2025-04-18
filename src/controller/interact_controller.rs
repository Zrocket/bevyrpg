use crate::interact::InteractEvent;
use crate::player::Player;
use crate::{CollisionLayer, HEIGHT, WIDTH};
use avian3d::prelude::*;
use avian_pickup::input::AvianPickupInput;
use avian_pickup::prop::HeldProp;
use bevy::prelude::*;

pub fn manage_interact(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    ray_caster: SpatialQuery,
    mut interact_event_writer: EventWriter<InteractEvent>,
    player: Query<Entity, With<Player>>,
    query: Query<(&Camera, &GlobalTransform), Without<HeldProp>>,
) {
    if key.just_pressed(KeyCode::KeyE) {
        if let Ok(player) = player.get_single() {
            for (camera, global_transform) in query.iter() {
                let center_window = camera.viewport_to_world(global_transform, Vec2 { y: HEIGHT / 2., x: WIDTH / 2. }).unwrap();
                let camera_position = global_transform.translation();
                let direction = global_transform.forward();
                if let Some(ray_data) = ray_caster.cast_ray(
                    center_window.origin,
                    //camera_position,
                    center_window.direction.into(),
                    500.0,
                    true,
                    &SpatialQueryFilter::default().with_excluded_entities([player]),
                ) {
                    let hit_point = camera_position + direction * ray_data.distance;
                    info!(
                        "INTERACT Entity {:?} hit at point {}, from {}",
                        ray_data.entity, hit_point, camera_position
                    );
                    commands.entity(ray_data.entity).trigger(InteractEvent {
                        actor: player,
                        target: ray_data.entity
                    });
                    //interact_event_writer.send(InteractEvent {
                    //    actor: player,
                    //    target: ray_data.entity,
                    //});
                }
            }
        }
    }
}
