use avian3d::{collision::collision_events::CollisionStart, dynamics::rigid_body::RigidBodyDisabled, physics_transform::{Position, Rotation}};
use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::items::socket;

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
#[component(on_add = on_socket_item_add)]
pub struct SocketItem;

fn on_socket_item_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(socket_test);
}

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct PlugItem;

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct MountPoint;

#[derive(EntityEvent)]
pub struct PlugSocketEvent {
    pub entity: Entity,
    pub plug: Entity,
}

pub struct SocketItemPlugin;
impl Plugin for SocketItemPlugin {
    fn build(&self, app: &mut App) {
       app;
    }
}

#[allow(clippy::type_complexity)]
fn socket_test(
    trigger: On<CollisionStart>,
    mut commands: Commands,
    mut plug_query: Query<(Entity, &mut Position, &mut Rotation), With<PlugItem>>,
    mount_query: Query<(&Position, &Rotation), (With<MountPoint>, Without<PlugItem>)>,
    socket_query: Query<Entity, With<SocketItem>>,
) {
    if let Ok((plug_entity, mut plug_position, mut plug_rotation)) = plug_query.get_mut(trigger.event().collider2)
    && let Ok((mount_position, mount_rotation)) = mount_query.single()
    && let Ok(socket_entity) = socket_query.single() {
        //*plug_position = mount_position.clone();
        *plug_position = *mount_position;
        //*plug_rotation = mount_rotation.clone();
        *plug_rotation = *mount_rotation;
        commands.entity(plug_entity).insert(RigidBodyDisabled);
        commands.entity(socket_entity).trigger(|entity| { PlugSocketEvent { entity, plug: plug_entity } });
    }
}
