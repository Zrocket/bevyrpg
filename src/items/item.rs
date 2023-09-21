use bevy::{prelude::*, reflect::TypePath};

#[derive(TypePath)]
pub enum ItemType {
    None,
    Weapon,
    Armor,
    Consume,
    Book,
    Ammo,
    Misc,
}

impl Default for ItemType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct Item {
    pub name: String,
    pub description: String,
}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Item>();
    }
}
