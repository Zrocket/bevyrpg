use bevy::{color::palettes::css::CRIMSON, prelude::*};

use crate::{DisplayInventoryEvent, InspectEvent, Interactable, InteractionEvent, UiInspect, add_to_inventory_observer, display_inventory_event_observer, remove_from_inventory_observer, widgets};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
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
            .observe(add_to_inventory_observer::<Container>)
            .observe(remove_from_inventory_observer::<Container>)
            .insert(Interactable);
    }
}

fn container_interaction_observer(
    trigger: On<InteractionEvent>,
    mut commands: Commands,
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
