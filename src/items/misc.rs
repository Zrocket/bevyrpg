use avian_pickup::{input::{AvianPickupAction, AvianPickupInput}, prop::HeldProp};
use bevy::{color::palettes::css::CRIMSON, prelude::*};

use crate::{InspectEvent, Interactable, InteractionEvent, UiInspect, widgets};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MiscItem;

pub struct MiscItemPlugin;

impl Plugin for MiscItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MiscItem>()
            .add_systems(Update, register_misc_items);
    }
}

fn register_misc_items(
    mut commands: Commands,
    mut unregistered_items_query: Query<Entity, (With<MiscItem>, Without<Interactable>)>,
) {
    for unregistered_item in unregistered_items_query.iter_mut() {
        commands.entity(unregistered_item)
            .observe(misc_interaction_observer)
            .observe(misc_inspection_observer)
            .insert(Interactable);
    }
}

fn misc_interaction_observer(
    trigger: On<InteractionEvent>,
    mut avian_pickup_input_writer: MessageWriter<AvianPickupInput>,
    _held_prop_query: Query<&HeldProp>,
) {
    info!("Misc Interact event observer");
    let actor = trigger.event().actor;
    avian_pickup_input_writer.write(AvianPickupInput { actor, action: AvianPickupAction::Pull });
}

fn misc_inspection_observer(
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
