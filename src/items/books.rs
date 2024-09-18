use bevy::{ecs::reflect, prelude::*};

#[derive(Component, Reflect, Default, Clone, Debug)]
#[reflect(Component)]
pub struct Book {
    pub title: String,
    pub contents: String,
}

pub struct BookPlugin;

impl Plugin for BookPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Book>();
    }
}
