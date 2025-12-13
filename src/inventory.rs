use super::GameState;
use bevy::prelude::*;
use std::ops::Deref;

use crate::items::*;

#[derive(EntityEvent)]
pub struct AddToInventoryEvent {
    pub entity: Entity,
    pub item: Entity,
}

#[derive(EntityEvent)]
pub struct RemoveFromInventoryEvent {
    pub entity: Entity,
    pub item: Entity,
}

#[derive(Message)]
pub struct RemoveMessage {
    pub actor: Entity,
    pub target: Entity,
}

#[derive(Component, Default, Debug)]
pub struct Inventory {
    pub items: Vec<Entity>,
}

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<RemoveMessage>();
            //.add_systems(
            //    Update,
            //    remove_from_inventory.run_if(in_state(GameState::Gameplay)),
           // )
    }
}

fn remove_from_inventory(
    mut commands: Commands,
    mut remove_events: MessageReader<RemoveMessage>,
    item_query: Query<Entity, With<Item>>,
    mut actor: Query<(Entity, &mut Inventory)>,
) {
    trace!("SYSTEM: remove_from_inventory");

    for event in remove_events.read() {
        if let Ok((_, mut inventory)) = actor.get_mut(event.actor) {
            inventory.items.retain(|item| *item != event.target);
            if let Ok(item) = item_query.get(event.target) {
            }
        }
    }
}
