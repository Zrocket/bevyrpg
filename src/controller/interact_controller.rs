use crate::interact::InteractEvent;
use crate::player::Player;
use avian3d::prelude::*;
use bevy::prelude::*;

pub fn manage_interact(
    key: Res<ButtonInput<KeyCode>>,
    ray_caster: SpatialQuery,
    mut interact_event_writer: EventWriter<InteractEvent>,
    player: Query<Entity, With<Player>>,
    query: Query<(&Camera, &GlobalTransform)>,
) {
    if key.just_pressed(KeyCode::KeyE) {
        if let Ok(player) = player.get_single() {
            for (_camera, global_transform) in query.iter() {
                let camera_position = global_transform.translation();
                let direction = global_transform.forward();
                if let Some(ray_data) = ray_caster.cast_ray(
                    camera_position,
                    direction.into(),
                    100.0,
                    false,
                    &SpatialQueryFilter::default().with_excluded_entities([player]),
                ) {
                    let hit_point = camera_position + direction * ray_data.distance;
                    info!(
                        "INTERACT Entity {:?} hit at point {}",
                        ray_data.entity, hit_point
                    );
                    interact_event_writer.send(InteractEvent {
                        actor: player,
                        target: ray_data.entity,
                    });
                }
            }
        }
    }
}
