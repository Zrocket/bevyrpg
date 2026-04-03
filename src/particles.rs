use avian3d::prelude::*;
use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use bevy_hanabi::{AccelModifier, Attribute, ColorBlendMask, ColorOverLifetimeModifier, EffectAsset, EffectMaterial, ExprWriter, OrientModifier, ParticleEffect, ParticleTextureModifier, ScalarType, SetAttributeModifier, SetPositionSphereModifier, SetVelocitySphereModifier, SizeOverLifetimeModifier, SpawnerSettings};
use bevy_seedling::{prelude::SpatialBasicNode, sample::SamplePlayer, sample_effects};

#[derive(Resource)]
pub struct FireEffectResource(Handle<EffectAsset>);

impl FromWorld for FireEffectResource {
    fn from_world(world: &mut World) -> Self {
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();

        // Define a color gradient from red to transparent black
        let mut color_gradient = bevy_hanabi::Gradient::new();
        //color_gradient.add_key(0.0, Vec4::new(1., 1., 1., 1.));
        color_gradient.add_key(0.0, Vec4::new(1., 0., 0., 1.));
        color_gradient.add_key(1.0, Vec4::new(1., 0.5, 0., 0.));

        let mut size_gradient = bevy_hanabi::Gradient::new();
        size_gradient.add_key(0.0, Vec3::new(0., 0., 0.));
        size_gradient.add_key(0.2, Vec3::new(1., 1., 1.));
        size_gradient.add_key(1.0, Vec3::new(0., 0., 0.));

        let writer = ExprWriter::new();

        let age = writer.lit(0.).expr();
        let init_age = SetAttributeModifier::new(Attribute::AGE, age);

        let rotation = (writer.rand(ScalarType::Float) * writer.lit(std::f32::consts::TAU)).expr();
        let init_rotation = SetAttributeModifier::new(Attribute::F32_0, rotation);
        let rotation_attr = writer.attr(Attribute::F32_0).expr();

        let age_rot = (writer.attr(Attribute::F32_0) + (writer.lit(40.) * writer.attr(Attribute::AGE))).expr();

        let texture_slot = writer.lit(0u32).expr();

        // Create a new expression module
        //let mut module = Module::default();
        let mut module = writer.finish();

        // On spawn, randomly initialize the position of the particle
        // to be over the surface of a sphere of radius 2 units.
        let init_pos = SetPositionSphereModifier {
            center: module.lit(Vec3::ZERO),
            radius: module.lit(0.4),
            dimension: bevy_hanabi::ShapeDimension::Volume,
        };

        // Also initialize a radial initial velocity to 6 units/sec
        // away from the (same) sphere center.
        let init_vel = SetVelocitySphereModifier {
            center: module.lit(Vec3::ZERO),
            //speed: module.lit(6.),
            speed: module.lit(Vec3::new(0., 5., 0.)),
        };

        // Initialize the total lifetime of the particle, that is
        // the time for which it's simulated and rendered. This modifier
        // is almost always required, otherwise the particles will stay
        // alive forever, and new particles can't be spawned instead.
        let lifetime = module.lit(0.5); // literal value "10.0"
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        // Every frame, add a gravity-like acceleration downward
        let accel = module.lit(Vec3::new(0., 3., 0.));
        let update_accel = AccelModifier::new(accel);

        module.add_texture_slot("color");

        // Create the effect asset
        let effect = EffectAsset::new(
            // Maximum number of particles alive at a time
            5000,
            // Spawn at a rate of 5 particles per second
            SpawnerSettings::rate(500.0.into()),
            // Move the expression module into the asset
            module,
        )
        .with_name("FireEffect")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .init(init_age)
        .init(init_rotation)
        .update(update_accel)
        // Render the particles with a color gradient over their
        // lifetime. This maps the gradient key 0 to the particle spawn
        // time, and the gradient key 1 to the particle death (10s).
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient,
            blend: bevy_hanabi::ColorBlendMode::Overwrite,
            mask: ColorBlendMask::RGBA,
        })
        .render(OrientModifier {
            mode: bevy_hanabi::OrientMode::FaceCameraPosition,
            rotation: Some(rotation_attr),
            //rotation: None,
        }.with_rotation(age_rot))
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        })
        .render(ParticleTextureModifier {
            texture_slot,
            sample_mapping: bevy_hanabi::ImageSampleMapping::ModulateOpacityFromR,
        });

        // Insert into the asset system
        let effect_asset = effects.add(effect);

        Self(effect_asset)
    }
}

#[derive(Resource)]
pub struct SmokeEffectResource(Handle<EffectAsset>);

impl FromWorld for SmokeEffectResource {
    fn from_world(world: &mut World) -> Self {
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();
        let mut color_gradient = bevy_hanabi::Gradient::new();
        color_gradient.add_key(0.0, Vec4::new(0., 0., 0., 0.));
        color_gradient.add_key(0.5, Vec4::new(1., 1., 1., 0.5));
        color_gradient.add_key(1.0, Vec4::new(0., 0., 0., 0.));

        let mut size_gradient = bevy_hanabi::Gradient::new();
        size_gradient.add_key(0.0, Vec3::new(0., 0., 0.));
        size_gradient.add_key(0.2, Vec3::new(1., 1., 1.));
        size_gradient.add_key(1.0, Vec3::new(0., 0., 0.));

        let writer = ExprWriter::new();

        let age = writer.lit(0.).expr();
        let init_age = SetAttributeModifier::new(Attribute::AGE, age);

        let rotation = (writer.rand(ScalarType::Float) * writer.lit(std::f32::consts::TAU)).expr();
        let init_rotation = SetAttributeModifier::new(Attribute::F32_0, rotation);
        let rotation_attr = writer.attr(Attribute::F32_0).expr();

        let age_rot = (writer.attr(Attribute::F32_0) + (writer.lit(40.) * writer.attr(Attribute::AGE))).expr();

        let texture_slot = writer.lit(0u32).expr();

        let mut module = writer.finish();

        let init_pos = SetPositionSphereModifier {
            center: module.lit(Vec3::ZERO),
            radius: module.lit(0.4),
            dimension: bevy_hanabi::ShapeDimension::Volume,
        };

        let init_vel = SetVelocitySphereModifier {
            center: module.lit(Vec3::ZERO),
            speed: module.lit(Vec3::new(0., 4., 0.)),
        };

        let lifetime = module.lit(1.0); // literal value "10.0"
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        let accel = module.lit(Vec3::new(0., 3., 0.));
        let update_accel = AccelModifier::new(accel);

        module.add_texture_slot("color");

        let effect = EffectAsset::new(
            5000,
            SpawnerSettings::rate(50.0.into()),
            module
        )
        .with_name("SmokeEffect")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .init(init_age)
        .init(init_rotation)
        .update(update_accel)
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient,
            blend: bevy_hanabi::ColorBlendMode::Overwrite,
            mask: ColorBlendMask::RGBA,
        })
        .render(OrientModifier {
            mode: bevy_hanabi::OrientMode::FaceCameraPosition,
            rotation: Some(rotation_attr),
            //rotation: None,
        }.with_rotation(age_rot))
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        })
        .render(ParticleTextureModifier {
            texture_slot,
            sample_mapping: bevy_hanabi::ImageSampleMapping::ModulateOpacityFromR,
        });

        let effect_asset = effects.add(effect);

        Self(effect_asset)
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    CollisionEventsEnabled,
    PointLight,
)]
#[component(on_add = on_particle_tester_add)]
#[type_path("api")]
pub struct ParticleTester;

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

fn fire_collision_observer(
    trigger: On<CollisionStart>,
    mut commands: Commands,
    flamable_query: Query<Entity, (With<Flamable>, Without<ParticleTester>)>,
) {
    if let Ok(flamable_entity) = flamable_query.get(trigger.event().collider2) {
        commands.entity(flamable_entity)
            .insert(ParticleTester);
    }
}
