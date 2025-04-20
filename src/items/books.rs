use avian_pickup::input::AvianPickupInput;
use bevy::{ecs::system::QueryLens, prelude::*};

use crate::{interact::Interaction, InteractEvent, Interactable};

#[derive(Component, Reflect, Default, Clone, Debug)]
#[reflect(Component)]
pub struct Book {
    pub title: String,
    pub contents: String,
}

impl Interaction for Book {
    fn interact(
        &self,
        actor: &Entity,
        query: QueryLens<&Interactable>
    ) -> Option<AvianPickupInput>
    {
        None
    }
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
