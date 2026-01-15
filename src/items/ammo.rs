use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*, state::commands};

use crate::{Interactable, InteractionEvent, UseEvent};

#[derive(Debug, Clone, Reflect, Default)]
pub enum AmmoType {
    #[default]
    None,
}

#[derive(Debug, Clone, Component,  Reflect, Default)]
#[reflect(Component)]
#[component(on_add = on_ammo_add)]
#[require(
    Interactable,
)]
pub struct Ammo;

#[derive(Debug, Clone, Component,  Reflect, Default)]
#[reflect(Component)]
pub struct AmmoPouch(pub i32);

pub struct AmmoPlugin;

impl Plugin for AmmoPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ammo>()
            .register_type::<AmmoPouch>();
    }
}

fn on_ammo_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    trace!("HOOK: on_ammo_add");
    world.commands()
        .entity(context.entity)
        .observe(ammo_interaction_observer)
        .observe(ammo_use_observer);
}

fn ammo_interaction_observer(
        trigger: On<InteractionEvent>
) {
    trace!("OBSERVER: ammo_interaction_observer");
    let _player = trigger.event().actor;
    let _ammo = trigger.entity;
}

fn ammo_use_observer(
    trigger: On<UseEvent>,
    mut commands: Commands,
) {
    trace!("OBSERVER: ammo_use_observer");
    commands.entity(trigger.entity).despawn();
}
