use bevy::{color::palettes::css::{DARK_GRAY, DARK_GREEN}, ui_widgets::observe};

use crate::widgets::floating_windows::floating_window_root;

use super::*;

#[derive(Component, Reflect)]
pub struct UiInventory;

#[derive(EntityEvent)]
pub struct DisplayInventoryEvent {
    pub entity: Entity,
}

#[derive(Component, Debug)]
pub struct Owner {
    item_owner: Entity,
    inv_owner: Entity,
}

#[derive(Component, Debug)]
pub struct InvRef(pub Entity);

pub struct InventoryUIPlugin;
impl Plugin for InventoryUIPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(OnEnter(GameState::Inventory), spawn_inventory_ui);
    }
}

pub fn spawn_inventory_ui(
    mut commands: Commands,
    inventory: Query<Entity, With<Player>>,
) {
    trace!("SYSTEM: spawn_inventory_ui");

    if let Ok(entity) = inventory.single() {
        commands.entity(entity).trigger(|entity| DisplayInventoryEvent { entity });
    }
}

pub fn display_inventory_event_observer(
    trigger: On<DisplayInventoryEvent>,
    mut commands: Commands,
    name_query: Query<&Name>,
    inventory: Query<&Inventory>,
) {
    trace!("OBSERVER: display_inventory_event_observer");
    let Ok(name) = name_query.get(trigger.entity) else {
        return;
    };

    let mut item_vec = vec![];

    if let Ok(inventory_handle) = inventory.get(trigger.entity) {
        for item in inventory_handle.iter() {
            if let Ok(item_name) = name_query.get(item) {
                trace!("Pushing item: {:?}, item_name: {:?}, to item_vec", item, item_name);
                item_vec.push((item_name.clone(), item.clone(), trigger.entity.clone()));
            }
        }
    }

    commands.spawn((
        floating_window_root(format!("{} Inventory", name),
        (
            Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                for (item, entity, inv) in item_vec {
                    parent.spawn((Text(item.to_string()), BackgroundColor::from(DARK_GREEN), Owner { item_owner: entity, inv_owner: inv }))
                        .observe(item_hover_test);
                }
            })),
            BackgroundColor::from(DARK_GRAY),
            InvRef(trigger.entity),
            observe(transfer_item_observer),
            observe(refresh_window_observer),
        ),
        ),
    ));
}

fn item_hover_test(
    trigger: On<Pointer<DragStart>>,
) {
    println!("MOVING OVER: {:?}", trigger.entity);
}

#[derive(EntityEvent)]
pub struct RefreshInventory {
    pub entity: Entity,
}

fn refresh_window_observer(
    trigger: On<RefreshInventory>,
    mut commands: Commands,
    mut children_query: Query<(&mut Children, &mut InvRef)>,
    name_query: Query<&Name>,
    inventory: Query<&Inventory>,
) {
    trace!("OBSERVER: refresh_window_observer");
    if let Ok((children, invref)) = children_query.get_mut(trigger.entity) {
        trace!("Got children: {:?}, invref: {:?}", children, invref);
        for child in children.iter() {
            trace!("Despawning child: {:?}", child);
            commands.entity(child).despawn();
        }

        let mut item_vec = vec![];

        if let Ok(inventory_handle) = inventory.get(invref.0) {
            for item in inventory_handle.iter() {
                if let Ok(item_name) = name_query.get(item) {
                    trace!("Pushing item: {:?}, item_name: {:?}, to item_vec", item, item_name);
                    item_vec.push((item_name.clone(), item.clone(), invref.0.clone()));
                }
            }
        }

        for (item, entity, inv) in item_vec {
            let child = commands.spawn((Text(item.to_string()), BackgroundColor::from(DARK_GREEN), Owner { item_owner: entity, inv_owner: inv })).id();
            commands.entity(trigger.entity).add_child(child).observe(item_hover_test);
        }

    }
}

fn transfer_item_observer(
    trigger: On<Pointer<DragDrop>>,
    mut commands: Commands,
    owner_query: Query<&Owner>,
    invref_query: Query<&InvRef>,
    childof_query: Query<&ChildOf>,
) {
    trace!("OBSERVER: transfer_item_observer");
    if let Ok(invref) = invref_query.get(trigger.entity)
    && let Ok(item) = owner_query.get(trigger.dropped)
    && let Ok(childof) = childof_query.get(trigger.dropped) {
        trace!("Removing item: {:?}, from inventory: {:?}", item.item_owner, item.inv_owner);
        commands.entity(item.inv_owner).trigger(|entity| RemoveFromInventoryEvent { entity, item: item.item_owner });
        trace!("Adding item: {:?}, to inventory: {:?}", item.item_owner, invref.0);
        commands.entity(invref.0).trigger(|entity| AddToInventoryEvent { entity, item: item.item_owner });
        trace!("Refreshing new inventory window: {:?}", trigger.entity);
        commands.entity(trigger.entity).trigger(|entity| RefreshInventory { entity });
        trace!("Refreshing old inventory window: {:?}", childof.0);
        commands.entity(childof.0).trigger(|entity| RefreshInventory { entity });
    }
}
