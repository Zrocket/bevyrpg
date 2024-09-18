use bevy::prelude::*;

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub enum ArmorType {
    #[default]
    None,
    Chest,
    Leg,
    Foot,
    Head,
    Arm,
    Hand,
    Face,
}

#[derive(Component, Debug, Clone, Reflect, Default)]
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
