use bevy::{color::palettes::css::CRIMSON, prelude::*};

use crate::{AddToInventoryEvent, DisplayInventoryEvent, InspectEvent, Interactable, InteractionEvent, RemoveFromInventoryEvent, UiInspect, display_inventory_event_observer, inventory::Inventory, widgets};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
#[require(Inventory)]
pub struct Container;

pub struct ContainerPlugin;
impl Plugin for ContainerPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<Container>()
           .add_systems(Update, register_container_items); 
    }
}

fn register_container_items(
    mut commands: Commands,
    mut unregistered_items_query: Query<Entity, (With<Container>, Without<Interactable>)>,
) {
    for unregistered_item in unregistered_items_query.iter_mut() {
        commands.entity(unregistered_item)
            .observe(container_interaction_observer)
            .observe(container_inspection_observer)
            .observe(display_inventory_event_observer)
            .observe(container_add_to_inventory_observer)
            .observe(container_remove_from_inventory_observer)
            .insert(Interactable);
    }
}

fn container_interaction_observer(
    trigger: On<InteractionEvent>,
    mut commands: Commands,
    inventory_query: Query<&Inventory>,
) {
    commands.entity(trigger.entity).trigger(|entity| DisplayInventoryEvent { entity });
}

fn container_inspection_observer(
    trigger: On<InspectEvent>,
    name_query: Query<&Name>,
    mut commands: Commands,
) {
    if let Ok(name) = name_query.get(trigger.entity) {
        commands.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(5.),
                    height: Val::Percent(5.),
                    left: Val::Percent(55.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_self: AlignSelf::Center,
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                BackgroundColor(CRIMSON.into()),
                UiInspect,
                children![
                    widgets::label(name),
                ]
        ));
    }
}

fn container_add_to_inventory_observer(
    trigger: On<AddToInventoryEvent>,
    //mut commands: Commands,
    mut container_inventory_query: Query<(Entity, &mut Inventory), With<Container>>,
) {
    trace!("OBSERVER: container_add_to_inventory_observer");
    if let Ok((containter_entity, mut container_inventory)) = container_inventory_query.get_mut(trigger.entity) {
        container_inventory.items.push(trigger.item);
        //commands.entity(containter_entity).add_child(trigger.item);
    }
}

fn container_remove_from_inventory_observer(
    trigger: On<RemoveFromInventoryEvent>,
    //mut commands: Commands,
    mut container_query: Query<(Entity, &mut Inventory), With<Container>>,
) {
    trace!("OBSERVER: container_remove_from_inventory_observer");
    if let Ok((container_entity, mut container_inventory)) = container_query.get_mut(trigger.entity) {
        let index = container_inventory.items.iter().position(|x| *x == trigger.item).unwrap();
        container_inventory.items.remove(index);
        //commands.entity(player_entity).add_child(trigger.item);
    }
}
