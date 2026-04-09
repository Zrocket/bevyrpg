use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_cctv_cam_add)]
struct CctvCam;

fn on_cctv_cam_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(on_cctv_right_observer)
        .observe(on_cctv_left_observer);
}

#[derive(EntityEvent)]
pub struct CctvLeftEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct CctvRightEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct CctvUpEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct CctvDownEvent {
    pub entity: Entity,
}

pub(crate) struct CctvPlugin;
impl Plugin for CctvPlugin {
    fn build(&self, app: &mut App) {
       app;
    }
}

fn on_cctv_right_observer(
    _trigger: On<CctvRightEvent>,
    mut cctv_query: Query<&mut Transform, With<CctvCam>>,
) {
    if let Ok(mut cctv_transform) = cctv_query.single_mut() {
        cctv_transform.rotate(Quat::from_rotation_y(5.0));
    }
}

fn on_cctv_left_observer(
    _trigger: On<CctvLeftEvent>,
    mut cctv_query: Query<&mut Transform, With<CctvCam>>,
) {
    if let Ok(mut cctv_transform) = cctv_query.single_mut() {
        cctv_transform.rotate(Quat::from_rotation_y(-5.0));
    }
}
