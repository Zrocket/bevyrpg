use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use bevy_hanabi::{EffectAsset, EffectMaterial, ParticleEffect};

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_frost_particle_emitter_add)]
pub struct FrostParticleEmitter;

fn on_frost_particle_emitter_add(
   mut world: DeferredWorld,
   context: HookContext,
) {
    let asset_server = world.resource::<AssetServer>();
        let frost_resource = world.resource::<FrostEffectResource>().0.clone();
        let texture_handle: Handle<Image> = asset_server.load("particles/cloud.png");
        //let sample_player = SamplePlayer::new(asset_server.load("audio/loop_rain.ogg")).looping();

        let frost_emitter = world.commands().spawn((
                ParticleEffect::new(frost_resource),
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
            .add_child(frost_emitter);
}

#[derive(Resource)]
pub struct FrostEffectResource(pub Handle<EffectAsset>);

/*impl FromWorld for FrostEffectResource {
    fn from_world(world: &mut World) -> Self {
    }
}*/

pub struct FrostParticlePlugin;
impl Plugin for FrostParticlePlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<FrostParticleEmitter>();
    }
}
