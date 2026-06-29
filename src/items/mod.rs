use bevy::prelude::*;
use avian_pickup::prop::HeldProp;
use avian3d::prelude::{CollisionLayers, CollisionStart, Position, RigidBodyDisabled, Rotation};
use std::iter;

use crate::{level::CollisionLayer};

mod ammo;
mod armor;
mod books;
mod consumable;
mod container;
mod drillable;
mod equip;
mod health_pack;
mod mana_pack;
mod misc;
mod sample;
mod weapons;

pub use ammo::*;
pub use armor::*;
pub use books::*;
pub use consumable::*;
pub use container::*;
pub use drillable::*;
pub use equip::*;
pub use misc::*;
pub use sample::*;
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
pub struct SocketItem;

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct PlugItem;

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct MountPoint;

#[derive(EntityEvent)]
pub struct PlugSocketEvent {
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
            ))
            .add_observer(disabled_held_prop_collision)
            .add_observer(enable_dropped_prop_collision)
            .add_systems(Update, register_socket_items);
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

fn register_socket_items(
    mut commands: Commands,
    mut socket_query: Query<Entity, (With<SocketItem>, Without<RegisteredItem>)>
) {
    for socket in socket_query.iter_mut() {
        commands.entity(socket)
            .observe(socket_test)
            .insert(RegisteredItem);
    }
}

#[allow(clippy::type_complexity)]
fn socket_test(
    trigger: On<CollisionStart>,
    mut commands: Commands,
    mut plug_query: Query<(Entity, &mut Position, &mut Rotation), With<PlugItem>>,
    mount_query: Query<(&Position, &Rotation), (With<MountPoint>, Without<PlugItem>)>,
) {
    if let Ok((plug_entity, mut plug_position, mut plug_rotation)) = plug_query.get_mut(trigger.event().collider2)
    && let Ok((mount_position, mount_rotation)) = mount_query.single() {
        *plug_position = mount_position.clone();
        *plug_rotation = mount_rotation.clone();
        commands.entity(plug_entity).insert(RigidBodyDisabled);
    }
}
