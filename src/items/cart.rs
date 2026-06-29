use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::misc_pickup_observer;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    crate::Interactable,
)]
#[component(on_add = on_cartridge_add)]
pub struct Cartridge {
    pub title: String,
    pub icon: Image,
}

fn on_cartridge_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    trace!("HOOK: on_cartridge_add");
    world.commands()
        .entity(context.entity)
        .observe(misc_pickup_observer);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub enum CartDatatype {
    Image,
    Audio,
    Game,
    Video,
    Text,
}

pub struct CartItemPlugin;
impl Plugin for CartItemPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<Cartridge>()
           .register_type::<CartDatatype>();
    }
}
