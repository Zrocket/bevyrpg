use avian_pickup::input::AvianPickupInput;
use bevy::prelude::*;

use crate::{interact::Interaction, InteractEvent};

#[derive(Component, Reflect, Default, Clone, Debug)]
#[reflect(Component)]
pub struct Book {
    pub title: String,
    pub contents: String,
}

impl Interaction for Book {
    fn interact(
        &self,
        _commands: &mut Commands,
        _actor: &Entity,
        _prop: &Entity,
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
    let _player = trigger.event().actor;
    let _book = trigger.entity();
}
