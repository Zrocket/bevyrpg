use crate::{DamageEvent, Player, PlayerCamera};
use avian3d::prelude::*;
use bevy::prelude::*;

use super::GameState;

#[derive(Event)]
pub struct ShootEvent;

#[derive(Debug, Component)]
pub struct Grenade; /*{
    rigid_body: RigidBody,
    transform: Transform,
    restitution: Restitution,
    damage: i32,
    splash_radius: i32,
    timer: Timer,
    collider: Collider,
}*/

pub struct ShootPlugin;

impl Plugin for ShootPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShootEvent>()
            //.add_systems(Update, shoot.run_if(in_state(GameState::Gameplay)));
            .add_systems(Update, shoot_grenade.run_if(in_state(GameState::Gameplay)));
    }
}

pub fn shoot(
    mut shoot_events: EventReader<ShootEvent>,
    mut damage_event: EventWriter<DamageEvent>,
    ray_caster: SpatialQuery,
    player: Query<Entity, With<Player>>,
        query: Query<&GlobalTransform, With<Camera>>,
) {
    trace!("Event Handler: shoot");
    let player = player.get_single().unwrap();
    for _event in shoot_events.read() {
        for global_transform in query.iter() {
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
                    "SHOOT Entity {:?} hit at point {}",
                    ray_data.entity, hit_point
                );
                damage_event.send(DamageEvent {
                    target: ray_data.entity,
                    ammount: 10,
                });
            }
        }
    }
}

const GRENADE_SIZE: f32 = 0.1;

pub fn shoot_grenade(
    mut commands: Commands,
    mut shoot_events: EventReader<ShootEvent>,
    mut damage_event: EventWriter<DamageEvent>,
    player_entity_query: Query<Entity, With<Player>>,
    camera_transform_query: Query<&GlobalTransform, With<PlayerCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    trace!("Event Handler: shoot_grenade");
    for _event in shoot_events.read() {
        for global_transform in camera_transform_query.iter() {
            let camera_position = global_transform.translation();
            println!("Camera Position: {}", camera_position);
            let grenade_position = camera_position + (global_transform.forward() * 3.);
            println!("Grenade Position: {}", grenade_position);
            let direction = global_transform.forward();
            let linear_velocity = direction * 23.;
            commands.spawn((
                    Grenade,
                    RigidBody::Dynamic,
                    Collider::capsule(GRENADE_SIZE, GRENADE_SIZE),
                    Mesh3d(meshes.add(Capsule3d::new(GRENADE_SIZE, GRENADE_SIZE))),
                    MeshMaterial3d(materials.add(Color::WHITE)),
                    Transform {
                        translation: grenade_position,
                        ..default()
                    },
                    LinearVelocity(linear_velocity),
                    Restitution::new(1.),
            ));
        }
    }
}
