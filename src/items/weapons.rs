use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{Interactable, InteractionEvent};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub enum WeaponType {
    #[default]
    None,
}

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
#[component(on_add = on_weapon_add)]
#[require(
    Interactable,
)]
pub struct Weapon {
    weapon_type: WeaponType,
}

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Weapon>();
    }
}

fn on_weapon_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    trace!("HOOK: on_weapon_add");
    world.commands()
        .entity(context.entity)
        .observe(weapon_interaction_observer);
}

fn weapon_interaction_observer(
    trigger: On<InteractionEvent, Weapon>
) {
    trace!("OBSERVER: weapon_interaction_observer");
    let _actor = trigger.event().actor;
    let _weapon = trigger.entity;
}
