use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_light_flicker_add)]
pub struct LightFlicker {
    freq: f32,
}

fn on_light_flicker_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity);
}

pub struct CeilingLightPlugin;
impl Plugin for CeilingLightPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<LightFlicker>();
    }
}
