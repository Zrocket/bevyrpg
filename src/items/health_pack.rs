use avian_pickup::{input::{AvianPickupAction, AvianPickupInput}, prop::HeldProp};
use avian3d::prelude::RigidBodyDisabled;
use bevy::{color::palettes::css::CRIMSON, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{AddToInventoryEvent, HealEvent, InspectEvent, Interactable, InteractionEvent, ItemDetails, PickupEvent, Shelf, UiInspect, UseEvent, widgets};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[component(on_add = on_health_item_add)]
#[require(
    Interactable,
)]
#[type_path("api")]
pub struct HealthItem;

pub struct HealthItemPlugin;

impl Plugin for HealthItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HealthItem>();
    }
}

fn on_health_item_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    trace!("HOOK: on_health_item_add");
    world.commands()
        .entity(context.entity)
        .observe(health_item_interaction_observer)
        .observe(health_item_pickup_observer)
        .observe(health_item_inspection_observer)
        .observe(health_item_use_observer);
}

fn health_item_interaction_observer(
    trigger: On<InteractionEvent>,
    mut commands: Commands,
    parent_query: Query<&ChildOf>,
    transform_query: Query<&Transform>,
    _held_prop_query: Query<&HeldProp>,
) {
    trace!("OBSERVER: misc_interaction_observer");
    let actor = trigger.event().actor;
    if let Ok(parent) = parent_query.get(trigger.event().entity)
    && let Ok(parent_transform) = transform_query.get(parent.0)
    && let Ok(item_transform) = transform_query.get(trigger.event().entity) {
        commands.entity(parent.0).insert(Shelf(Box::new(parent_transform.clone())));
        commands.entity(trigger.event().entity).insert(Shelf(Box::new(item_transform.clone())));
        commands.entity(parent.0).remove::<GlobalTransform>();
        commands.entity(parent.0).remove::<Transform>();
        commands.entity(trigger.event().entity).remove::<GlobalTransform>();
        commands.entity(trigger.event().entity).remove::<Transform>();
        commands.entity(actor).trigger(|entity| AddToInventoryEvent { entity, item: trigger.event().entity });
    }
}

fn health_item_pickup_observer(
    trigger: On<PickupEvent>,
    mut commands: Commands,
    mut avian_pickup_input_writer: MessageWriter<AvianPickupInput>,
    _held_prop_query: Query<&HeldProp>,
) {
    trace!("OBSERVER: misc_interaction_observer");
    let actor = trigger.event().actor;
    commands.entity(trigger.event().entity).remove::<RigidBodyDisabled>();
    avian_pickup_input_writer.write(AvianPickupInput { actor, action: AvianPickupAction::Pull });
}

fn health_item_inspection_observer(
    trigger: On<InspectEvent>,
    name_query: Query<&ItemDetails>,
    mut commands: Commands,
) {
    trace!("OBSERVER: misc_inspection_observer");
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
                    widgets::label(name.name.clone()),
                ]
        ));
    }
}

fn health_item_use_observer(
    trigger: On<UseEvent>,
    mut commands: Commands,
) {
    trace!("OBSERVER: health_item_use_observer");
    commands.entity(trigger.actor).trigger(|entity| HealEvent { entity, ammount: 10 });
    commands.entity(trigger.entity).despawn();
}
