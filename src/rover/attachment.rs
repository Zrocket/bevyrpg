use avian3d::prelude::{Collider, CollisionEventsEnabled, CollisionStart, LinearVelocity, RigidBody};
use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{InteractionEvent, Rover, RoverCamera};

#[derive(Debug, Component, Reflect)]
#[require(
    RigidBody::Dynamic,
    Collider::capsule(0.1, 0.1),
    CollisionEventsEnabled,
)]
#[component(on_add = on_foam_dart_add)]
pub struct FoamDart(pub Timer);

fn on_foam_dart_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let mut meshes = world.resource_mut::<Assets<Mesh>>();
    let mesh = meshes.add(Capsule3d::new(0.1, 0.1));
    let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
    let material = materials.add(Color::WHITE);

    world.commands()
        .entity(context.entity)
        .insert(Mesh3d(mesh))
        .insert(MeshMaterial3d(material))
        .observe(dart_interaction);
}

#[derive(Component, Default)]
#[relationship_target(relationship = AttachedToRover)]
pub struct RoverAttachments(Vec<Entity>);

#[derive(Component)]
#[relationship(relationship_target = RoverAttachments)]
pub struct AttachedToRover(pub Entity);

#[derive(EntityEvent)]
pub struct UseRoverAttachmentEvent {
    pub entity: Entity,
}

#[derive(Component)]
#[require(
    Name::new("Foam Gun"),
)]
#[component(on_add = on_foam_gun_add)]
pub struct FoamGunAttachment;

fn on_foam_gun_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(use_foam_gun_observer);
}

fn use_foam_gun_observer(
    _trigger: On<UseRoverAttachmentEvent>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
    transform_query: Query<&GlobalTransform, With<RoverCamera>>,
) {
    if let Ok(rover_entity) = rover_query.single()
    && let Ok(camera_transform) = transform_query.single() {
        let camera_position = camera_transform.translation();
        let dart_position = camera_position + (camera_transform.forward() * 1.);
        let direction = camera_transform.forward().normalize();
        let linear_velocity = direction * 23.;
        commands.spawn((
                //Grenade(Timer::from_seconds(3., TimerMode::Once)),
                FoamDart(Timer::from_seconds(3., TimerMode::Once)),
                Transform {
                    translation: dart_position,
                    ..default()
                },
                LinearVelocity(linear_velocity),
        ));
    }
}

pub(crate) fn dart_timer(
    mut commands: Commands,
    mut dart_query: Query<(Entity, &mut FoamDart)>,
    time: Res<Time>,
) {
    for (entity, mut dart) in dart_query.iter_mut() {
        dart.0.tick(time.delta());

        if dart.0.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub(crate) fn dart_interaction(
    trigger: On<CollisionStart>,
    mut commands: Commands,
) {
    println!("AAAAAAAAAAAAAAAAAA");
    commands.entity(trigger.event().collider2).trigger(|entity| InteractionEvent { entity, actor: trigger.collider1 });
}
