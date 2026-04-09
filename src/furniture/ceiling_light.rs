use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, input::common_conditions::input_just_pressed, prelude::*};

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
            .register_type::<LightFlicker>()
            .add_systems(Update, toggle_light.run_if(input_just_pressed(KeyCode::KeyX)));
    }
}

fn toggle_light(
    mut commands: Commands,
    light_query: Query<(Entity, Option<&PointLight>), With<LightFlicker>>,
) {
    if let Ok((light_entity, point_light)) = light_query.single() {
        if point_light.is_some() {
            commands.entity(light_entity).remove::<PointLight>();
        } else {
            commands.entity(light_entity).insert(PointLight::default());
        }
    }
}
