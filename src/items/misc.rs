use avian_pickup::{input::{AvianPickupAction, AvianPickupInput}, prop::HeldProp};
use avian3d::prelude::RigidBodyDisabled;
use bevy::{color::palettes::css::CRIMSON, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{InspectEvent, Interactable, InteractionEvent, UiInspect, widgets};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[component(on_add = on_misc_add)]
pub struct MiscItem;

pub struct MiscItemPlugin;

impl Plugin for MiscItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MiscItem>();
    }
}

fn on_misc_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(misc_interaction_observer)
        .observe(misc_inspection_observer)
        .insert(Interactable);
}

fn misc_interaction_observer(
    trigger: On<InteractionEvent>,
    mut commands: Commands,
    mut avian_pickup_input_writer: MessageWriter<AvianPickupInput>,
    _held_prop_query: Query<&HeldProp>,
) {
    info!("Misc Interact event observer");
    let actor = trigger.event().actor;
    commands.entity(trigger.event().entity).remove::<RigidBodyDisabled>();
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
