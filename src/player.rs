use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerCamera;

pub struct GamePlayerPlugin;

impl Plugin for GamePlayerPlugin {
    fn build(&self, app: &mut App) {
        info!("GamePlayerPlugin build");
        app.register_type::<Player>()
            .register_type::<PlayerCamera>();
    }
}
