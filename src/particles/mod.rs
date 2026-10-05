use avian3d::prelude::*;
use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use bevy_hanabi::{AccelModifier, Attribute, ColorBlendMask, ColorOverLifetimeModifier, EffectAsset, EffectMaterial, ExprWriter, OrientModifier, ParticleEffect, ParticleTextureModifier, ScalarType, SetAttributeModifier, SetPositionSphereModifier, SetVelocitySphereModifier, SizeOverLifetimeModifier, SpawnerSettings};
use bevy_seedling::{prelude::SpatialBasicNode, sample::SamplePlayer, sample_effects};

use crate::{DamageEvent, Health};

mod fire_particles;

use fire_particles::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    CollisionEventsEnabled,
    PointLight,
)]
#[component(on_add = on_particle_tester_add)]
#[type_path("api")]
pub struct ParticleTester;

fn on_particle_tester_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let asset_server = world.resource::<AssetServer>();
    let fire_resource = world.resource::<FireEffectResource>().0.clone();
    let smoke_resource = world.resource::<SmokeEffectResource>().0.clone();
    let texture_handle: Handle<Image> = asset_server.load("particles/cloud.png");
    let sample_player = SamplePlayer::new(asset_server.load("audio/fire-1.ogg")).looping();

    let fire_emitter = world.commands().spawn((
            ParticleEffect::new(fire_resource),
            EffectMaterial {
                images: vec![texture_handle.clone()],
            },
    )).id();
    let smoke_emitter = world.commands().spawn((
            ParticleEffect::new(smoke_resource),
            EffectMaterial {
                images: vec![texture_handle.clone()],
            },
    )).id();

    world.commands()
        .entity(context.entity)
        .insert((
            sample_player,
            sample_effects!(SpatialBasicNode::default())
        ))
        .add_child(fire_emitter)
        .add_child(smoke_emitter)
        .observe(fire_collision_observer);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    CollisionEventsEnabled,
)]
#[type_path("api")]
pub struct Flamable;

pub struct ParticlePlugin;
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<ParticleTester>()
           .register_type::<Flamable>()
           .init_resource::<FireEffectResource>()
           .init_resource::<SmokeEffectResource>();
           //.add_systems(Startup, smoke_effect)
           //.add_systems(Startup, fire_effect);
    }
}

fn fire_collision_observer(
    trigger: On<CollisionStart>,
    mut commands: Commands,
    flamable_query: Query<Entity, (With<Flamable>, Without<ParticleTester>)>,
    character_query: Query<Entity, With<Health>>,
) {
    if let Ok(flamable_entity) = flamable_query.get(trigger.event().collider2) {
        commands.entity(flamable_entity)
            .insert(ParticleTester);
        } else if let Ok(character_entity) = character_query.get(trigger.event().collider2) {
            commands.entity(character_entity).trigger(|entity| DamageEvent { entity, ammount: 10 });
        }
}
