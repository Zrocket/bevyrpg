use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use bevy_hanabi::{Attribute, ColorOverLifetimeModifier, EffectAsset, EffectMaterial, ExprWriter, LinearDragModifier, OrientModifier, ParticleEffect, ScalarType, SetAttributeModifier, SpawnerSettings, VectorType};

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_dust_particle_emitter_add)]
pub struct DustParticleEmitter;

fn on_dust_particle_emitter_add(
   mut world: DeferredWorld,
   context: HookContext,
) {
    let asset_server = world.resource::<AssetServer>();
        let dust_resource = world.resource::<DustEffectResource>().0.clone();
        let texture_handle: Handle<Image> = asset_server.load("particles/cloud.png");
        //let sample_player = SamplePlayer::new(asset_server.load("audio/fire-1.ogg")).looping();

        let dust_emitter = world.commands().spawn((
                ParticleEffect::new(dust_resource),
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
            .add_child(dust_emitter);
}

#[derive(Resource)]
pub struct DustEffectResource(pub Handle<EffectAsset>);

impl FromWorld for DustEffectResource {
    fn from_world(world: &mut World) -> Self {
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();

        let writer = ExprWriter::new();

        let half_extents = Vec3::new(5.0, 2.0, 5.0);
        let center = Vec3::new(0.0, 2.0, 0.0);
        let pos = (writer.rand(VectorType::VEC3F) * writer.lit(half_extents * 2.0)
            + writer.lit(center - half_extents))
            .expr();
        let init_pos = SetAttributeModifier::new(Attribute::POSITION, pos);

        let vel = ((writer.rand(VectorType::VEC3F) *writer.lit(2.0_f32) - writer.lit(1.0_f32))
            * writer.lit(0.15_f32))
            .expr();
        let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, vel);

        let lifetime = (writer.lit(6.0_f32) + writer.rand(ScalarType::Float) * writer.lit(4.0_f32)).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        let size = (writer.lit(0.01_f32) + writer.rand(ScalarType::Float) * writer.lit(0.02_f32)).expr();
        let init_size = SetAttributeModifier::new(Attribute::SIZE, size);

        let drag = writer.lit(0.5_f32).expr();
        let update_drag = LinearDragModifier::new(drag);

        let mut gradient = bevy_hanabi::Gradient::new();
        gradient.add_key(0.0, Vec4::new(1.0, 0.95, 0.85, 0.0));
        gradient.add_key(0.2, Vec4::new(1.0, 0.95, 0.85, 0.5));
        gradient.add_key(0.8, Vec4::new(1.0, 0.95, 0.85, 0.5));
        gradient.add_key(1.0, Vec4::new(1.0, 0.95, 0.85, 0.0));

        let module = writer.finish();

        let effect = EffectAsset::new(512, SpawnerSettings::rate(50.0.into()), module)
            .with_alpha_mode(bevy_hanabi::AlphaMode::Blend)
            .init(init_pos)
            .init(init_vel)
            .init(init_lifetime)
            .init(init_size)
            .update(update_drag)
            .render(ColorOverLifetimeModifier::new(gradient))
            .render(OrientModifier::new(bevy_hanabi::OrientMode::FaceCameraPosition));

        let effect_asset = effects.add(effect);

        Self(effect_asset)
    }
}

pub struct DustParticlePlugin;
impl Plugin for DustParticlePlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<DustParticleEmitter>()
           .init_resource::<DustEffectResource>();
    }
}
