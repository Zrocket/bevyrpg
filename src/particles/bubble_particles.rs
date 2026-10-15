use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use bevy_hanabi::{EffectAsset, EffectMaterial, ParticleEffect};

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_bubble_particle_emitter_add)]
pub struct BubbleParticleEmitter;

fn on_bubble_particle_emitter_add(
   mut world: DeferredWorld,
   context: HookContext,
) {
    let asset_server = world.resource::<AssetServer>();
        let bubble_resource = world.resource::<BubbleEffectResource>().0.clone();
        let texture_handle: Handle<Image> = asset_server.load("particles/cloud.png");

        let bubble_emitter = world.commands().spawn((
                ParticleEffect::new(bubble_resource),
                EffectMaterial {
                    images: vec![texture_handle.clone()],
                },
        )).id();

        world.commands()
            .entity(context.entity)
            .add_child(bubble_emitter);
}

#[derive(Resource)]
pub struct BubbleEffectResource(pub Handle<EffectAsset>);

/*impl FromWorld for BubbleEffectResource {
    fn from_world(world: &mut World) -> Self {
    }
}*/

pub struct BubbleParticlePlugin;
impl Plugin for BubbleParticlePlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<BubbleParticleEmitter>();
    }
}
