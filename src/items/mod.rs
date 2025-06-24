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

use crate::Name;

#[derive(Component, Reflect, Clone, Default)]
pub struct Weight(pub i32);
#[derive(Component, Reflect, Clone, Default)]
pub struct Description(pub String);

#[derive(Component, Clone, Default)]
pub struct Item {
    pub name: Name,
    pub description: Description,
    pub weight: Weight,
}

pub struct ItemPlugin;
impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
   //                 AmmoPlugin,
  //                  ArmorPlugin,
 //                   BookPlugin,
                    MiscItemPlugin,
//                    WeaponPlugin,
            ));
    }
}
