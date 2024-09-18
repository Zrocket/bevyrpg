use bevy::prelude::*;

#[derive(Debug, Clone, Reflect, Default)]
pub enum AmmoType {
    #[default]
    None,
}

#[derive(Debug, Clone, Reflect, Default)]
pub struct Ammo;

pub struct AmmoPlugin;

impl Plugin for AmmoPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ammo>();
    }
}
