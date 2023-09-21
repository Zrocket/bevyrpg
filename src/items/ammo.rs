use bevy::prelude::*;

#[derive(Reflect)]
pub enum AmmoType {
    None,
}

impl Default for AmmoType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Ammo;

pub struct AmmoPlugin;

impl Plugin for AmmoPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ammo>();
    }
}
