use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, input::common_conditions::input_just_pressed, light::light_consts::lumens::VERY_LARGE_CINEMA_LIGHT, prelude::*};

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
    mut light_query: Query<(Entity, Option<&mut PointLight>), With<LightFlicker>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if let Ok((light_entity, Some(mut point_light))) = light_query.single_mut() {
    //&& key.just_pressed(KeyCode::KeyB) {
        if point_light.intensity == 0. {
        println!("VVVVVVVVVVV");
            point_light.intensity = VERY_LARGE_CINEMA_LIGHT;
        } else {
        println!("XXXXXXXXXX");
            point_light.intensity = 0.
        }
    } else {
        error!("toggle_light: Failed to query Entity and PointLight for LightFlicker");
    }
}
