use bevy::{prelude::*, reflect};
use super::GameState;

use crate::character::*;
use crate::items::*;

#[derive(Event)]
pub struct PickUpEvent {
    pub actor: Entity,
    pub item: Entity,
}

#[derive(Event)]
pub struct RemoveEvent {
    pub actor: Entity,
    pub index: i32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Inventory {
    pub items: Vec<Item>,
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PickUpEvent>()
            .add_event::<RemoveEvent>()
            .add_systems(Update, add_to_inventory.run_if(in_state(GameState::Gameplay)))
            .add_systems(Update, remove_from_inventory.run_if(in_state(GameState::Gameplay)));
    }
}

fn add_to_inventory(
    mut commands: Commands,
    mut pick_up_events: EventReader<PickUpEvent>,
    mut actor: Query<(Entity, &mut Inventory)>,
    mut item: Query<(Entity, &mut Item)>
) {
    for event in pick_up_events.iter() {
        let (_, mut inventory) = actor.get_mut(event.actor).unwrap();
        let (item_entity, item) = item.get_mut(event.item).unwrap();
        inventory.items.push(item.clone());
        commands.entity(item_entity).despawn_recursive();
    }
}

fn remove_from_inventory(
    mut remove_events: EventReader<RemoveEvent>,
    mut actor: Query<(Entity, &mut Inventory)>,
){
    for event in remove_events.iter() {
        let (_, mut inventory) = actor.get_mut(event.actor).unwrap();
        inventory.items.remove(event.index as usize);
    }
}
