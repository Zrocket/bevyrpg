use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{Interactable, InteractionEvent, UseEvent, book_ui::display_book_ui};

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
        .observe(display_book_ui);
}

fn book_interaction_observer(
    trigger: On<InteractionEvent>,
    mut commands: Commands,
) {
    trace!("OBSERVER: book_interaction_observer");
    commands.entity(trigger.entity).trigger(|entity| OpenBookEvent { entity });
}

fn book_use_observer(
    trigger: On<UseEvent>,
    mut commands: Commands,
) {
    trace!("OBSERVER: book_use_observer");
    commands.entity(trigger.entity).trigger(|entity| OpenBookEvent { entity });
}
