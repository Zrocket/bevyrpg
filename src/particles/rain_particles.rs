use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use bevy_hanabi::{Attribute, EffectAsset, EffectMaterial, ExprWriter, OrientModifier, ParticleEffect, SetAttributeModifier, SetSizeModifier, SpawnerSettings, VectorType};
use bevy_seedling::{nodes::core::SpatialBasicNode, sample::SamplePlayer, sample_effects};

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_rain_particle_emitter_add)]
pub struct RainParticleEmitter;

fn on_rain_particle_emitter_add(
   mut world: DeferredWorld,
   context: HookContext,
) {
    let asset_server = world.resource::<AssetServer>();
        let rain_resource = world.resource::<RainEffectResource>().0.clone();
        let texture_handle: Handle<Image> = asset_server.load("particles/cloud.png");
        let sample_player = SamplePlayer::new(asset_server.load("audio/loop_rain.ogg")).looping();

        let rain_emitter = world.commands().spawn((
                ParticleEffect::new(rain_resource),
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
            .add_child(rain_emitter);
}

#[derive(Resource)]
pub struct RainEffectResource(pub Handle<EffectAsset>);

impl FromWorld for RainEffectResource {
    fn from_world(world: &mut World) -> Self {
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();

        let writer = ExprWriter::new();

        let age = writer.lit(0.).expr();
        let init_age = SetAttributeModifier::new(Attribute::AGE, age);

        let color = writer.lit(Vec4::new(0., 0., 1., 1.))
            .pack4x8unorm()
            .expr();
        let init_color = SetAttributeModifier::new(Attribute::COLOR, color);

        let half_extents = Vec3::new(10.0, 0.1, 10.0);
        let center = Vec3::new(0.0, 15.0, 0.0);

        let pos = (writer.rand(VectorType::VEC3F) * writer.lit(half_extents * 2.0)
            + writer.lit(center - half_extents))
            .expr();
        let init_pos = SetAttributeModifier::new(Attribute::POSITION, pos);

        let vel = writer.lit(Vec3::new(0.0, -25.0, 0.0)).expr();
        let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, vel);

        let lifetime = writer.lit(1.5_f32).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        let mut module = writer.finish();

        let effect = EffectAsset::new(1024, SpawnerSettings::rate(50.0.into()), module)
        .init(init_pos)
        .init(init_age)
        .init(init_color)
        .init(init_vel)
        .init(init_lifetime)
        .render(SetSizeModifier {
            size: Vec3::new(0.02, 0.4, 1.0).into()
        })
        .render(OrientModifier::new(bevy_hanabi::OrientMode::FaceCameraPosition));

        let effect_asset = effects.add(effect);

        Self(effect_asset)
    }
}

pub struct RainParticlePlugin;
impl Plugin for RainParticlePlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<RainParticleEmitter>()
           .init_resource::<RainEffectResource>(); 
    }
}
