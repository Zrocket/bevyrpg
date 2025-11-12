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

use bevy::{ecs::system::IntoObserverSystem, prelude::*};

use crate::{Interactable, Name};

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct UnregisteredItem;

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct Weight(pub i32);

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
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

/*fn register_items<E, B, M, I>(
    mut commands: Commands,
    observer_system: I,
    mut unregistered_items_query: Query<Entity, Without<Interactable>>,
)
where
    E: EntityEvent,
    B: Component,
    I: IntoObserverSystem<E, B, M>,
{
    let observer_system = IntoObserverSystem::into_system(observer_system);
    let mut unregistered_items_query = unregistered_items_query.transmute_lens_filtered::<Entity, (Without<Interactable>, With<B>)>();
    for unregistered_item in unregistered_items_query.query().iter_mut() {
        commands.entity(unregistered_item).observe(observer_system)
            .insert(Interactable);
    }
}*/
