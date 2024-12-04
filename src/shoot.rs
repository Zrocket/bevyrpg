use bevy::prelude::*;
use avian3d::prelude::*;
use crate::{Player, DamageEvent};

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
    mut damage_event: EventWriter<DamageEvent>,
    ray_caster: SpatialQuery,
    player: Query<Entity, With<Player>>,
    query: Query<(&Camera, &GlobalTransform)>,
) {
    trace!("Event Handler: shoot");
    let player = player.get_single().unwrap();
    for _event in shoot_events.read() {
        for (_camera, global_transform) in query.iter() {
            let camera_position = global_transform.translation();
            let direction = global_transform.forward();

            if let Some(ray_data) = ray_caster.cast_ray(
                camera_position, direction.into(), 100.0, false, SpatialQueryFilter::default().with_excluded_entities([player]) 
                ) {
                let hit_point = camera_position + direction * ray_data.time_of_impact;
                info!("SHOOT Entity {:?} hit at point {}", ray_data.entity, hit_point);
                damage_event.send(DamageEvent { target: ray_data.entity, ammount: 10 });
            }
        }
    }
}

