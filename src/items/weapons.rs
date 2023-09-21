use bevy::prelude::*;

pub enum WeaponType {
    None,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Weapon;

pub struct WeaponPlugin {
}

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Weapon>();
    }
}
