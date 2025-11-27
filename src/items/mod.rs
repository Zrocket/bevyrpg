mod ammo;
mod armor;
mod books;
mod consume;
mod container;
mod misc;
mod weapons;

use std::iter;

pub use ammo::*;
pub use armor::*;
use avian_pickup::prop::HeldProp;
use avian3d::prelude::CollisionLayers;
pub use books::*;
pub use consume::*;
pub use container::*;
pub use misc::*;
pub use weapons::*;

use bevy::{ecs::system::IntoObserverSystem, prelude::*};

use crate::{Interactable, Name, level::CollisionLayer};

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
            ))
            .add_observer(disabled_held_prop_collision)
            .add_observer(enable_dropped_prop_collision);
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

fn disabled_held_prop_collision(
    add: On<Add, HeldProp>,
    children_query: Query<&Children>,
    mut collision_layers_query: Query<&mut CollisionLayers>,
) {
    let rigid_body = add.entity;
    for child in iter::once(rigid_body).chain(children_query.iter_descendants(rigid_body)) {
        let Ok(mut collision_layers) = collision_layers_query.get_mut(child) else {
            continue;
        };
        collision_layers.filters.remove(CollisionLayer::Player);
    }
}

fn enable_dropped_prop_collision(
    remove: On<Remove, HeldProp>,
    children_query: Query<&Children>,
    mut collision_layers_query: Query<&mut CollisionLayers>,
) {
    let rigid_body = remove.entity;
    for child in iter::once(rigid_body).chain(children_query.iter_descendants(rigid_body)) {
        let Ok(mut collision_layers) = collision_layers_query.get_mut(child) else {
            continue;
        };
        collision_layers.filters.add(CollisionLayer::Player);
    }
}
