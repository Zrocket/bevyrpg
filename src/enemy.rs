use bevy::prelude::*;

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>();
    }
}
