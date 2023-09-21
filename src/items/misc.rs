use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MiscItem;

pub struct MiscItemPlugin {
}

impl Plugin for MiscItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MiscItem>();
    }
}
