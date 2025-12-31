use bevy::prelude::*;

use crate::{Interactable, InteractionEvent, book_ui::display_book_ui};

#[derive(EntityEvent)]
pub struct OpenBookEvent {
    pub entity: Entity,
}

#[derive(Component, Reflect, Default, Clone, Debug)]
#[reflect(Component)]
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
        app.register_type::<Book>()
            .add_systems(Update, register_book_items);
    }
}

fn register_book_items(
    mut commands: Commands,
    mut unregistered_items_query: Query<Entity, (With<Book>, Without<Interactable>)>,
) {
    for unregistered_item in unregistered_items_query.iter_mut() {
        println!("ASDASD");
        commands.entity(unregistered_item)
            .observe(book_interaction_observer)
            .insert(Interactable)
            .observe(display_book_ui);
    }
}

fn book_interaction_observer(
    trigger: On<InteractionEvent>,
    mut commands: Commands,
) {
    let book = trigger.entity;
    commands.entity(book).trigger(|entity| OpenBookEvent { entity });
}
