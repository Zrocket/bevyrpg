use bevy::prelude::*;
use bevy_hanabi::{Attribute, EffectAsset, ExprWriter, SetAttributeModifier, SpawnerSettings, VectorType};

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

        let center = Vec3::ZERO;
        let half_extents = Vec3::new(2.0, 2.0, 2.0);

        let pos = (writer.rand(VectorType::VEC3F) * writer.lit(half_extents * 2.0)
            + writer.lit(center - half_extents))
            .expr();
        let init_pos = SetAttributeModifier::new(Attribute::POSITION, pos);

        let mut module = writer.finish();

        let lifetime = module.lit(0.5);
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        let effect = EffectAsset::new(1024, SpawnerSettings::rate(50.0.into()), module)
        .init(init_pos)
        .init(init_age)
        .init(init_color);

        let effect_asset = effects.add(effect);

        Self(effect_asset)
    }
}

pub struct RainParticlePlugin;
impl Plugin for RainParticlePlugin {
    fn build(&self, app: &mut App) {
       app; 
    }
}
