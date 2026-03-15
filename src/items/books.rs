use bevy::{color::palettes::css::CRIMSON, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{InspectEvent, Interactable, InteractionEvent, ItemDetails, UiInspect, UseEvent, book_ui::display_book_ui, widgets};

#[derive(EntityEvent)]
pub struct OpenBookEvent {
    pub entity: Entity,
}

#[derive(Component, Reflect, Default, Clone, Debug)]
#[reflect(Component)]
#[component(on_add = on_book_add)]
#[require(
    Interactable,
)]
#[type_path("api")]
pub struct Book {
    pub title: String,
    pub contents: String,
}

#[derive(Component, Reflect, Default, Clone, Debug)]
#[reflect(Component)]
pub struct BookPages {
    pub pages: Vec<String>,
}

pub struct BookPlugin;

impl Plugin for BookPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Book>();
    }
}

fn on_book_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    trace!("HOOK: on_book_add");
    world.commands()
        .entity(context.entity)
        .observe(book_interaction_observer)
        .observe(book_inspection_observer)
        .observe(book_use_observer)
        .observe(display_book_ui);
}

fn book_interaction_observer(
    trigger: On<InteractionEvent>,
    mut commands: Commands,
) {
    trace!("OBSERVER: book_interaction_observer");
    commands.entity(trigger.entity).trigger(|entity| OpenBookEvent { entity });
}

fn book_inspection_observer(
    trigger: On<InspectEvent>,
    name_query: Query<&ItemDetails>,
    mut commands: Commands,
) {
    trace!("OBSERVER: book_inspection_observer");
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

fn book_use_observer(
    trigger: On<UseEvent>,
    mut commands: Commands,
) {
    trace!("OBSERVER: book_use_observer");
    commands.entity(trigger.entity).trigger(|entity| OpenBookEvent { entity });
}
