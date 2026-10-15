use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use bevy_hanabi::{AccelModifier, Attribute, ColorOverLifetimeModifier, EffectAsset, EffectMaterial, ExprWriter, Gradient, OrientMode, OrientModifier, ParticleEffect, ScalarType, SetAttributeModifier, SetPositionSphereModifier, SetSizeModifier, SetVelocitySphereModifier, SpawnerSettings};

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_spark_particle_emitter_add)]
pub struct SparkParticleEmitter;

fn on_spark_particle_emitter_add(
   mut world: DeferredWorld,
   context: HookContext,
) {
    let asset_server = world.resource::<AssetServer>();
        let spark_resource = world.resource::<SparkEffectResource>().0.clone();
        let texture_handle: Handle<Image> = asset_server.load("particles/cloud.png");
        //let sample_player = SamplePlayer::new(asset_server.load("audio/loop_rain.ogg")).looping();

        let spark_emitter = world.commands().spawn((
                ParticleEffect::new(spark_resource),
                EffectMaterial {
                    images: vec![texture_handle.clone()],
                },
        )).id();

        world.commands()
            .entity(context.entity)
            //.insert((
            //        sample_player,
            //        sample_effects!(SpatialBasicNode::default())
            //))
            .add_child(spark_emitter);
}

#[derive(Resource)]
pub struct SparkEffectResource(pub Handle<EffectAsset>);

impl FromWorld for SparkEffectResource {
    fn from_world(world: &mut World) -> Self {
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();

        let writer = ExprWriter::new();

        let init_pos = SetPositionSphereModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            radius: writer.lit(0.05_f32).expr(),
            dimension: bevy_hanabi::ShapeDimension::Volume,
        };

        let init_vel = SetVelocitySphereModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            speed: (writer.lit(3.0_f32) + writer.rand(ScalarType::Float) * writer.lit(5.0_f32)).expr(),
        };

        let lifetime = (writer.lit(0.4_f32) + writer.rand(ScalarType::Float) * writer.lit(0.6_f32)).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        let accel = writer.lit(Vec3::new(0.0, -9.8, 0.0)).expr();
        let update_accel = AccelModifier::new(accel);

        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(1.0, 0.9, 0.5, 1.0));
        gradient.add_key(0.4, Vec4::new(1.0, 0.5, 0.1, 1.0));
        gradient.add_key(1.0, Vec4::new(0.8, 0.1, 0.0, 0.0));

        let module = writer.finish();

        let effect = EffectAsset::new(
            1024,
            SpawnerSettings::burst(100.0.into(), 2.0.into()),
            module,
        )
        .with_alpha_mode(bevy_hanabi::AlphaMode::Add)
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .update(update_accel)
        .render(ColorOverLifetimeModifier::new(gradient))
        .render(SetSizeModifier { size: Vec3::new(0.02, 0.15, 1.0).into() })
        .render(OrientModifier::new(OrientMode::AlongVelocity));

        let effect_asset = effects.add(effect);

        Self(effect_asset)
    }
}

pub struct SparkParticlePlugin;
impl Plugin for SparkParticlePlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<SparkParticleEmitter>()
           .init_resource::<SparkEffectResource>();
    }
}
