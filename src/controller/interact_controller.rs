use bevy::prelude::*;
use bevy_rapier3d::prelude::{*, Real};
use crate::player::Player;
use crate::interact::InteractEvent;

pub fn manage_interact(
    key: Res<ButtonInput<KeyCode>>,
    mut interact_event_writer: EventWriter<InteractEvent>,
    player: Query<Entity, With<Player>>,
    query: Query<(&Camera, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
) {
    if key.just_pressed(KeyCode::KeyE) {
        if let Ok(player) = player.get_single() {
            for (_camera, global_transform) in query.iter() {
                let camera_position = global_transform.translation();
                let direction = global_transform.forward();
                if let Some((entity, toi)) = rapier_context.cast_ray(
                    camera_position, direction.into(), Real::MAX, false, QueryFilter {exclude_collider: Some(player), ..default()}
                    ) {
                    let hit_point = camera_position + direction * toi;
                    info!("INTERACT Entity {:?} hit at point {}", entity, hit_point);
                    interact_event_writer.send(InteractEvent {
                        actor: player,
                        target: entity,
                    });
                }
            }
        }
    }
}
