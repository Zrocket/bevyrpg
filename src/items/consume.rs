use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Consume;

pub struct ComsumePlugin {
}

impl Plugin for ComsumePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Consume>();
    }
}
