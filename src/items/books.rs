use bevy::prelude::*;

use crate::InteractEvent;

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
            .add_observer(book_observer_handler);
    }
}

fn book_observer_handler(
    trigger: Trigger<InteractEvent, Book>
) {
    let player = trigger.event().actor;
    let book = trigger.entity();
}
