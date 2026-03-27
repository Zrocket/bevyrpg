use crate::{AmmoPouch, DamageEvent, Player, PlayerCamera};
use avian3d::prelude::*;
use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use bevy_seedling::sample::SamplePlayer;
use chill_bevy_console::console_closed;

use super::GameState;

const GRENADE_SIZE: f32 = 0.1;
const ROCKET_SIZE: f32 = 0.1;

#[derive(Event, Message)]
pub struct ShootEvent;

#[derive(Debug, Component, Reflect)]
pub struct Cooldown(Timer);

#[derive(Debug, Component, Reflect)]
#[require(
    RigidBody::Dynamic,
    Collider::capsule(GRENADE_SIZE, GRENADE_SIZE),
    CollisionEventsEnabled,
)]
#[component(on_add = on_grenade_add)]
pub struct Grenade(Timer); /*{
    damage: i32,
    splash_radius: i32,
}*/

#[derive(Debug, Component, Reflect)]
#[require(
    RigidBody::Kinematic,
    Collider::sphere(ROCKET_SIZE),
    CollisionEventsEnabled,
)]
#[component(on_add = on_rocket_add)]
pub struct Rocket;

fn on_grenade_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let mut meshes = world.resource_mut::<Assets<Mesh>>();
    let mesh = meshes.add(Capsule3d::new(GRENADE_SIZE, GRENADE_SIZE));
    let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
    let material = materials.add(Color::WHITE);

    world.commands()
        .entity(context.entity)
        .insert(Mesh3d(mesh))
        .insert(MeshMaterial3d(material));
}

fn on_rocket_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let mut meshes = world.resource_mut::<Assets<Mesh>>();
    let mesh = meshes.add(Sphere::new(ROCKET_SIZE));
    let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
    let material = materials.add(Color::WHITE);
    world.commands()
        .entity(context.entity)
        .insert(Mesh3d(mesh))
        .insert(MeshMaterial3d(material))
        .observe(explode_rocket);
}

pub struct ShootPlugin;
impl Plugin for ShootPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<ShootEvent>()
            .register_type::<Grenade>()
            .register_type::<Rocket>()
            .register_type::<Cooldown>()
            .add_systems(Update, (
                shoot_rocket.run_if(in_state(GameState::Gameplay)).run_if(console_closed),
                grenade_fuse.run_if(in_state(GameState::Gameplay)),
                ),
            );
    }
}

pub fn shoot(
    mut commands: Commands,
    mut shoot_messages: MessageReader<ShootEvent>,
    ray_caster: SpatialQuery,
    player: Query<Entity, With<Player>>,
        query: Query<&GlobalTransform, With<Camera>>,
) {
    trace!("Message Handler: shoot");
    let player = player.single().unwrap();
    for _message in shoot_messages.read() {
        for global_transform in query.iter() {
            let camera_position = global_transform.translation();
            let direction = global_transform.forward();

            if let Some(ray_data) = ray_caster.cast_ray(
                camera_position,
                direction,
                100.0,
                false,
                &SpatialQueryFilter::default().with_excluded_entities([player]),
            ) {
                let hit_point = camera_position + direction * ray_data.distance;
                info!(
                    "SHOOT Entity {:?} hit at point {}",
                    ray_data.entity, hit_point
                );
                commands.entity(ray_data.entity).trigger(|entity| DamageEvent { entity, ammount: 10 });
            }
        }
    }
}

pub fn grenade_fuse(
    mut commands: Commands,
    mut grenade_query: Query<(Entity, &mut Grenade)>,
    time: Res<Time>,
) {
    for (entity, mut grenade) in grenade_query.iter_mut() {
        grenade.0.tick(time.delta());

        if grenade.0.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn shoot_grenade(
    mut commands: Commands,
    mut shoot_messages: MessageReader<ShootEvent>,
    camera_transform_query: Query<&GlobalTransform, With<PlayerCamera>>,
) {
    trace!("Message Handler: shoot_grenade");
    for _message in shoot_messages.read() {
        for global_transform in camera_transform_query.iter() {
            let camera_position = global_transform.translation();
            let grenade_position = camera_position + (global_transform.forward() * 3.);
            let direction = global_transform.forward();
            let linear_velocity = direction * 23.;
            commands.spawn((
                    Grenade(Timer::from_seconds(3., TimerMode::Once)),
                    Transform {
                        translation: grenade_position,
                        ..default()
                    },
                    LinearVelocity(linear_velocity),
            ));
        }
    }
}

pub fn shoot_rocket(
    mut commands: Commands,
    mut shoot_messages: MessageReader<ShootEvent>,
    mut player_query: Query<&mut AmmoPouch, With<Player>>,
    camera_transform_query: Query<&GlobalTransform, With<PlayerCamera>>,
    asset_server: Res<AssetServer>,
) {
    trace!("Message Handler: shoot_rocket");
    for _message in shoot_messages.read() {
        if let Ok(mut player_ammo) = player_query.single_mut() && player_ammo.0 > 0 {
            player_ammo.0 -= 1;
            println!("player_ammo: {:?}", player_ammo);

            for global_transform in camera_transform_query.iter() {
                let camera_position = global_transform.translation();
                let rocket_position = camera_position + (global_transform.forward() * 1.);
                let direction = global_transform.forward().normalize();
                let linear_velocity = direction * 21.1111;
                commands.spawn((
                        Rocket,
                        Transform {
                            translation: rocket_position,
                            ..default()
                        },
                        LinearVelocity(linear_velocity),
                ));
                commands.spawn(SamplePlayer::new(asset_server.load("audio/rocket/rocket_launch_1.wav")));
            }
        }
    }
}

pub fn explode_rocket(
    trigger: On<CollisionStart>,
    mut commands: Commands,
) {
    commands.entity(trigger.event().collider2).trigger(|entity| DamageEvent { entity, ammount: 10 });
    commands.entity(trigger.event().collider1).despawn();
}
