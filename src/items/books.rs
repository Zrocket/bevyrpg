use bevy::prelude::*;

use crate::{Interactable, InteractionEvent};

#[derive(EntityEvent)]
pub struct OpenBookEvent {
    entity: Entity,
}

#[derive(Component, Reflect, Default, Clone, Debug)]
#[reflect(Component)]
pub struct Book {
    pub title: String,
    pub contents: String,
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
        commands.entity(unregistered_item).observe(book_interaction_observer)
            .insert(Interactable);
    }
}

fn book_interaction_observer(
    trigger: On<InteractionEvent, Book>
) {
    let _player = trigger.event().actor;
    let _book = trigger.entity;
}
