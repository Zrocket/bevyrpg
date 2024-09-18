use bevy::prelude::*;

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Hunger;

pub struct HungerPlugin;

impl Plugin for HungerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Hunger>();
    }
}
