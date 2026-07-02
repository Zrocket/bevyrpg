use bevy::prelude::*;
use avian_pickup::prop::HeldProp;
use avian3d::prelude::CollisionLayers;
use std::iter;

mod ammo;
mod armor;
mod books;
mod cart;
mod consumable;
mod container;
mod drillable;
mod equip;
mod health_pack;
mod key;
mod mana_pack;
mod misc;
mod sample;
mod socket;
mod weapons;

pub use ammo::*;
pub use armor::*;
pub use books::*;
pub use cart::*;
pub use consumable::*;
pub use container::*;
pub use drillable::*;
pub use equip::*;
pub use health_pack::*;
pub use key::*;
pub use mana_pack::*;
pub use misc::*;
pub use sample::*;
pub use socket::*;
pub use weapons::*;

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct RegisteredItem;

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct Weight(pub i32);

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct Description(pub String);

#[derive(EntityEvent)]
pub struct ItemInteractionEvent {
    entity: Entity,
}

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct ItemDetails {
    pub name: String,
    pub description: Description,
    pub weight: Weight,
}

pub struct ItemPlugin;
impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<ItemDetails>()
            .register_type::<SocketItem>()
            .register_type::<PlugItem>()
            .register_type::<MountPoint>()
            .add_plugins((
                    AmmoPlugin,
                    ArmorPlugin,
                    BookPlugin,
                    MiscItemPlugin,
                    ContainerPlugin,
                    WeaponPlugin,
                    SampleItemPlugin,
                    DrillableItemPlugin,
                    SocketItemPlugin,
            ))
            .add_observer(disabled_held_prop_collision)
            .add_observer(enable_dropped_prop_collision);
    }
}

fn disabled_held_prop_collision(
    add: On<Add, HeldProp>,
    children_query: Query<&Children>,
    mut collision_layers_query: Query<&CollisionLayers>,
) {
    let rigid_body = add.entity;
    for child in iter::once(rigid_body).chain(children_query.iter_descendants(rigid_body)) {
        let Ok(mut collision_layers) = collision_layers_query.get(child) else {
            continue;
        };
        //collision_layers.filters.remove(CollisionLayer::Player);
    }
}

fn enable_dropped_prop_collision(
    remove: On<Remove, HeldProp>,
    children_query: Query<&Children>,
    mut collision_layers_query: Query<&CollisionLayers>,
) {
    let rigid_body = remove.entity;
    for child in iter::once(rigid_body).chain(children_query.iter_descendants(rigid_body)) {
        let Ok(mut collision_layers) = collision_layers_query.get(child) else {
            continue;
        };
        //collision_layers.filters.add(CollisionLayer::Player);
    }
}
