use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{Interactable, InteractionEvent};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub enum ArmorType {
    #[default]
    None,
    Chest,
    Leg,
    Foot,
    Head,
    Arm,
    Hand,
    Face,
}

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
#[component(on_add = on_armor_add)]
#[require(
    Interactable,
)]
pub struct Armor {
    armor_type: ArmorType,
    defense: i32,
}

pub struct ArmorPlugin;
impl Plugin for ArmorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Armor>();
    }
}

fn on_armor_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    trace!("HOOK: on_armor_add");
    world.commands()
        .entity(context.entity);
}

fn armor_interaction_observer(
        trigger: On<InteractionEvent, Armor>
) {
    trace!("OBSERVER: armor_interaction_observer");
    let _player = trigger.event().actor;
    let _armor = trigger.entity;
}
