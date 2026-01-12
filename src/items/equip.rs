use bevy::{ecs::{component::Component, entity::Entity, lifecycle::HookContext, observer::On, relationship, system::Commands, world::DeferredWorld}, prelude::Plugin, prelude::trace};

use crate::UseEvent;

pub enum EquipSlot {
    Arm,
    Leg,
    Hand,
    Feet,
    Body,
    Head,
    Finger,
    Toe,
}

#[derive(Component)]
#[component(on_add = on_equiptable_add)]
pub struct Equiptable {
    pub slot: EquipSlot,
    pub defense: i32,
}

#[derive(Component)]
#[relationship_target(relationship = InEquiptment)]
pub struct Equiptment(Vec<Entity>);

#[derive(Component)]
#[relationship(relationship_target = Equiptment)]
pub struct InEquiptment(pub Entity);

pub struct EquipItemPlugin;
impl Plugin for EquipItemPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app; 
    }
}

fn on_equiptable_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    trace!("HOOK: on_equiptable_add");
    world.commands()
        .entity(context.entity)
        .observe(equip_use_event_observer);
}

fn equip_use_event_observer(
    trigger: On<UseEvent>,
    mut commands: Commands,
) {
    trace!("OBSERVER: equip_use_event_observer");
    commands.entity(trigger.entity)
        .insert(InEquiptment(trigger.actor));
}
