use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Book;

pub struct BookPlugin {
}

impl Plugin for BookPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Book>();
    }
}
