mod ammo;
mod armor;
mod books;
mod consume;
mod misc;
mod weapons;

pub use ammo::*;
pub use armor::*;
pub use books::*;
pub use consume::*;
pub use misc::*;
pub use weapons::*;

use bevy::prelude::*;

use crate::{Interactable, Name};

#[derive(Component, Reflect, Clone)]
pub enum ItemType {
    None,
    Weapon(Weapon),
    Armor(Armor),
    Consume(Consume),
    Book(Book),
    Ammo(Ammo),
    Misc,
}

impl Default for ItemType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Component, Reflect, Clone, Default)]
pub struct Weight(pub i32);
#[derive(Component, Reflect, Clone, Default)]
pub struct Description(pub String);

#[derive(Bundle, Clone, Default)]
pub struct Item {
    pub name: Name,
    pub description: Description,
    pub item_type: ItemType,
    pub weight: Weight,
    pub interact: Interactable,
}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ItemType>();
    }
}
