use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct AddToInventoryEvent {
    /// The entity who's inventory the item should be added to
    pub entity: Entity,
    /// The item to be added to the inventory
    pub item: Entity,
}

#[derive(EntityEvent)]
pub struct RemoveFromInventoryEvent {
    /// The entity who's inventory the item should be removed from
    pub entity: Entity,
    /// The item to be removed from the inventory
    pub item: Entity,
}

/*#[derive(Message)]
pub struct RemoveMessage {
    pub actor: Entity,
    pub target: Entity,
}*/

#[derive(Component)]
#[relationship_target(relationship = InInventory, linked_spawn)]
pub struct Inventory(Vec<Entity>);

#[derive(Component)]
#[relationship(relationship_target = Inventory)]
pub struct InInventory(pub Entity);

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app;
            //.add_message::<RemoveMessage>();
            //.add_systems(
            //    Update,
            //    remove_from_inventory.run_if(in_state(GameState::Gameplay)),
           // )
    }
}


/// Allows an entity to accept items into their inventory
pub fn add_to_inventory_observer<T: Component>(
    trigger: On<AddToInventoryEvent>,
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    trace!("OBSERVER: add_to_inventory_observer");
    if let Ok(entity) = query.get(trigger.entity) {
        println!("ADDING: {:?}", trigger.item);
        commands.entity(trigger.item).insert(InInventory(entity));
    }
}

/// Allows an entity to remove items from their inventory
pub fn remove_from_inventory_observer<T: Component>(
    trigger: On<RemoveFromInventoryEvent>,
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    trace!("OBSERVER: remove_from_inventory_observer");
    if query.get(trigger.entity).is_ok() {
        println!("REMOVING: {:?}", trigger.item);
        commands.entity(trigger.item).remove::<InInventory>();
    }
}
