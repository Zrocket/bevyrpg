use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{Interactable, InteractionEvent, book_ui::display_book_ui};

#[derive(EntityEvent)]
pub struct OpenBookEvent {
    pub entity: Entity,
}

#[derive(Component, Reflect, Default, Clone, Debug)]
#[reflect(Component)]
#[component(on_add = on_book_add)]
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
    world.commands()
        .entity(context.entity)
        .observe(book_interaction_observer)
        .insert(Interactable)
        .observe(display_book_ui);
}

fn book_interaction_observer(
    trigger: On<InteractionEvent>,
    mut commands: Commands,
) {
    let book = trigger.entity;
    commands.entity(book).trigger(|entity| OpenBookEvent { entity });
}
