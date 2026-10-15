use bevy::prelude::*;
use bevy_hanabi::{Attribute, EffectAsset, ExprWriter, ScalarType, SetAttributeModifier};

#[derive(Resource)]
pub struct SteamEffectResource(pub Handle<EffectAsset>);

/*impl FromWorld for SteamEffectResource {
    fn from_world(world: &mut World) -> Self {
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();

        let writer = ExprWriter.new();

        let age = writer.lit(0.).expr();
        let init_age = SetAttributeModifier::new(Attribute::AGE, age);

        let rotation = (writer.rand(ScalarType::Float) * writer.lit(std::f32::consts::TAU)).expr();
    }
}*/

pub struct SteamParticlePlugin;
impl Plugin for SteamParticlePlugin {
    fn build(&self, app: &mut App) {
       app; 
    }
}
