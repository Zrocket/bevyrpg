use avian3d::{collision::collision_events::CollisionStart, dynamics::rigid_body::RigidBodyDisabled, physics_transform::{Position, Rotation}};
use bevy::prelude::*;

use crate::RegisteredItem;

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

pub struct SocketItemPlugin;
impl Plugin for SocketItemPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(Update, register_socket_items);
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
