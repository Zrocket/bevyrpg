use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::Player;

use super::GameState;

#[derive(Event)]
pub struct ShootEvent;

pub struct ShootPlugin;

impl Plugin for ShootPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShootEvent>()
            .add_systems(Update, shoot.run_if(in_state(GameState::Gameplay)));
    }
}

pub fn shoot(
    mut shoot_events: EventReader<ShootEvent>,
    rapier_context: Res<RapierContext>,
    player: Query<Entity, With<Player>>,
    query: Query<(&Camera, &GlobalTransform)>,
) {
    let player = player.get_single().unwrap();
    for _event in shoot_events.iter() {
        for (_camera, global_transform) in query.iter() {
            let camera_position = global_transform.translation();
            let direction = global_transform.forward();

            if let Some((entity, toi)) = rapier_context.cast_ray(
                camera_position, direction, Real::MAX, false, QueryFilter {exclude_collider: Some(player), ..default()}
                ) {
                let hit_point = camera_position + direction * toi;
                println!("SHOOT Entity {:?} hit at point {}", entity, hit_point);
            }
        }
    }
}

