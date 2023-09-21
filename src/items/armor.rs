use bevy::prelude::*;

#[derive(Reflect)]
pub enum ArmorType {
    None,
    Chest,
    Leg,
    Foot,
    Head,
    Arm,
    Hand,
    Face,
}

impl Default for ArmorType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Armor {
    armor_type: ArmorType,
    defense: i32,
}

pub struct ArmorPlugin;

impl Plugin for ArmorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Armor>();
    }
}
