use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player;

pub struct GamePlayerPlugin;

impl Plugin for GamePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>();
    }
}
