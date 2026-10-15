use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use bevy_hanabi::{AccelModifier, Attribute, EffectAsset, EffectMaterial, ExprWriter, OrientMode, OrientModifier, ParticleEffect, ScalarType, SetAttributeModifier, SpawnerSettings, VectorType};

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_trash_particle_emitter_add)]
pub struct TrashParticleEmitter;

fn on_trash_particle_emitter_add(
   mut world: DeferredWorld,
   context: HookContext,
) {
    let asset_server = world.resource::<AssetServer>();
        let trash_resource = world.resource::<TrashEffectResource>().0.clone();
        let texture_handle: Handle<Image> = asset_server.load("particles/cloud.png");
        //let sample_player = SamplePlayer::new(asset_server.load("audio/loop_rain.ogg")).looping();

        let trash_emitter = world.commands().spawn((
                ParticleEffect::new(trash_resource),
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
            .add_child(trash_emitter);
}

#[derive(Resource)]
pub struct TrashEffectResource(pub Handle<EffectAsset>);

impl FromWorld for TrashEffectResource {
    fn from_world(world: &mut World) -> Self {
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();

        let writer = ExprWriter::new();

        let half_extents = Vec3::new(1.0, 1.5, 6.0);
        let center = Vec3::new(-12.0, 1.5, 0.0);

        let pos = (writer.rand(VectorType::VEC3F) * writer.lit(half_extents * 2.0)
            + writer.lit(center - half_extents))
            .expr();
        let init_pos = SetAttributeModifier::new(Attribute::POSITION, pos);

        let wind = writer.lit(Vec3::new(4.0, 0.3, 0.0));
        let jitter = (writer.rand(VectorType::VEC3F) * writer.lit(2.0_f32) - writer.lit(1.0_f32))
            * writer.lit(Vec3::new(1.5, 0.8, 1.0));

        let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, (wind + jitter).expr());

        let lifetime = (writer.lit(6.0_f32) + writer.rand(ScalarType::Float) * writer.lit(3.0_f32)).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        let size = (writer.lit(0.15_f32) + writer.rand(ScalarType::Float) * writer.lit(0.2_f32)).expr();
        let init_size = SetAttributeModifier::new(Attribute::SIZE, size);

        let color = (writer.lit(Vec4::new(0.45, 0.42, 0.35, 1.0))
            + writer.rand(VectorType::VEC4F) * writer.lit(Vec4::new(0.5, 0.5, 0.5, 0.0)))
            .pack4x8unorm()
            .expr();
        let init_color = SetAttributeModifier::new(Attribute::COLOR, color);

        let phase = writer.attr(Attribute::POSITION).x() * writer.lit(1.2_f32)
            + writer.time() * writer.lit(3.0_f32);
        let flutter = phase.sin() * writer.lit(Vec3::new(0.5, 2.5, 1.0));
        let update_flutter = AccelModifier::new(flutter.expr());

        let gravity = writer.lit(Vec3::new(0.0, -0.8, 0.0)).expr();
        let update_gravity = AccelModifier::new(gravity);

        let spin = (writer.attr(Attribute::AGE) * writer.lit(4.0_f32)).expr();

        let module = writer.finish();

        let effect = EffectAsset::new(256, SpawnerSettings::rate(8.0.into()), module)
            .with_alpha_mode(bevy_hanabi::AlphaMode::Opaque)
            .init(init_pos)
            .init(init_vel)
            .init(init_lifetime)
            .init(init_size)
            .init(init_color)
            .update(update_flutter)
            .update(update_gravity)
            .render(OrientModifier {
                mode: OrientMode::FaceCameraPosition,
                rotation: Some(spin),
            });

        let effect_asset = effects.add(effect);

        Self(effect_asset)
    }
}


pub struct TrashParticlePlugin;
impl Plugin for TrashParticlePlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<TrashParticleEmitter>()
           .init_resource::<TrashEffectResource>();
    }
}
