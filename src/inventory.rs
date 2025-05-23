use super::GameState;
use bevy::prelude::*;
use std::ops::Deref;

use crate::items::*;

#[derive(Event)]
pub struct PickUpEvent {
    pub actor: Entity,
    pub target: Entity,
}

#[derive(Event)]
pub struct RemoveEvent {
    pub actor: Entity,
    pub target: Entity,
}

#[derive(Component, Default)]
pub struct Inventory {
    pub items: Vec<Entity>,
    pub ui_index: usize,
    pub ui_active: bool,
}

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct InInventory(pub Entity);

impl From<InInventory> for Entity {
    fn from(in_inventory: InInventory) -> Entity {
        in_inventory.0
    }
}

impl From<&InInventory> for Entity {
    fn from(in_inventory: &InInventory) -> Entity {
        in_inventory.0
    }
}

impl From<Entity> for InInventory {
    fn from(entity: Entity) -> InInventory {
        InInventory(entity)
    }
}

impl AsRef<Entity> for InInventory {
    fn as_ref(&self) -> &Entity {
        &self.0
    }
}

impl Deref for InInventory {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PickUpEvent>()
            .add_event::<RemoveEvent>()
            .add_systems(
                Update,
                add_to_inventory.run_if(in_state(GameState::Gameplay)),
            )
            .add_systems(
                Update,
                remove_from_inventory.run_if(in_state(GameState::Gameplay)),
            )
            .register_type::<InInventory>();
    }
}

fn add_to_inventory(
    mut commands: Commands,
    mut pick_up_events: EventReader<PickUpEvent>,
    mut item: Query<Entity>, //With<ItemType>>,
    mut actor: Query<(Entity, &mut Inventory)>,
) {
    trace!("SYSTEM: add_to_inventory");

    for event in pick_up_events.read() {
        info!("Event Handler: add_to_inventory");
        if item.get_mut(event.target).is_ok() {
            //commands.entity(item_entity).despawn_recursive();
            commands
                .entity(event.target)
                .insert(InInventory(event.actor));
            //.remove::<PbrBundle>();
            //.remove::<Collider>();
            if let Ok((_, mut inventory)) = actor.get_mut(event.actor) {
                inventory.items.push(event.target);
            }
        }
    }
}

fn remove_from_inventory(
    mut commands: Commands,
    mut remove_events: EventReader<RemoveEvent>,
    item_query: Query<Entity, With<ItemType>>,
    mut actor: Query<(Entity, &mut Inventory)>,
) {
    trace!("SYSTEM: remove_from_inventory");

    for event in remove_events.read() {
        if let Ok((_, mut inventory)) = actor.get_mut(event.actor) {
            inventory.items.retain(|item| *item != event.target);
            if let Ok(item) = item_query.get(event.target) {
                commands.entity(item).remove::<InInventory>();
            }
        }
    }
}
