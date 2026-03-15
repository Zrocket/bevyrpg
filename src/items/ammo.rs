use bevy::{color::palettes::css::CRIMSON, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{InspectEvent, Interactable, InteractionEvent, ItemDetails, UiInspect, UseEvent, widgets};

#[derive(Debug, Clone, Reflect, Default)]
pub enum AmmoType {
    #[default]
    None,
}

#[derive(Debug, Clone, Component,  Reflect, Default)]
#[reflect(Component)]
#[component(on_add = on_ammo_add)]
#[require(
    Interactable,
)]
#[type_path("api")]
pub struct Ammo;

#[derive(Debug, Clone, Component,  Reflect, Default)]
#[reflect(Component)]
pub struct AmmoPouch(pub i32);

pub struct AmmoPlugin;

impl Plugin for AmmoPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ammo>()
            .register_type::<AmmoPouch>();
    }
}

fn on_ammo_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    trace!("HOOK: on_ammo_add");
    world.commands()
        .entity(context.entity)
        .observe(ammo_interaction_observer)
        .observe(ammo_inspection_observer)
        .observe(ammo_use_observer);
}

fn ammo_interaction_observer(
        trigger: On<InteractionEvent>
) {
    trace!("OBSERVER: ammo_interaction_observer");
    let _player = trigger.event().actor;
    let _ammo = trigger.entity;
}

fn ammo_inspection_observer(
    trigger: On<InspectEvent>,
    name_query: Query<&ItemDetails>,
    mut commands: Commands,
) {
    trace!("OBSERVER: ammo_inspection_observer");
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

fn ammo_use_observer(
    trigger: On<UseEvent>,
    mut commands: Commands,
    mut ammo_query: Query<&mut AmmoPouch>,
) {
    trace!("OBSERVER: ammo_use_observer");
    if let Ok(mut ammo_pouch) = ammo_query.get_mut(trigger.actor) {
        ammo_pouch.0 += 10;
        commands.entity(trigger.entity).despawn();
    }
}
