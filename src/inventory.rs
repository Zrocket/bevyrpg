use bevy::prelude::*;

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
