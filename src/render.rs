use std::time::Duration;

use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use avian3d::{collision::collider::Collider};

use crate::{CameraState, Player, PlayerController};

pub const RESOLUTION_HEIGHT: u32 = 720;
pub const RESOLUTION_WIDTH: u32 = 1280;

#[derive(Component)]
pub struct CameraConfig {
    pub height_offset: f32,
}

#[derive(Component)]
#[component(
    on_add = on_camera_interpolation_add,
    //on_remove = on_camera_interpolation_remove
)]
pub struct CameraInterpolation {
    pub duration: Duration,
    pub start_time: Duration,
    //pub start_pos: Transform,
    //pub desired_pos: Transform,
}

#[derive(Component)]
#[component(
    on_add = on_camera_interpolation_add,
    //on_remove = on_camera_interpolation_remove
)]
pub struct CameraInterpolation2 {
    pub duration: Duration,
    pub start_time: Duration,
    pub start_pos: Transform,
    pub desired_pos: Transform,
}

fn on_camera_interpolation_add(
    mut world: DeferredWorld,
    _context: HookContext,
) {
    let mut camera_state = world.resource_mut::<NextState<CameraState>>();
    camera_state.set(CameraState::Indipendent);
}

fn on_camera_interpolation_remove(
    mut world: DeferredWorld,
    _context: HookContext,
) {
    let mut camera_state = world.resource_mut::<NextState<CameraState>>();
    camera_state.set(CameraState::Player);
}


#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CameraTarget;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RenderPlayer {
    pub logical_entity: Entity,
}

pub struct GameRenderPlugin;
impl Plugin for GameRenderPlugin {
    fn build(&self, app: &mut App) {
        info!("GameRenderPlugin build");
        app
            .register_type::<RenderPlayer>()
            .register_type::<CameraTarget>()
            .add_systems(
            Update,
                //player_controller_render.run_if(in_state(GameState::Gameplay)),
                player_controller_render.run_if(in_state(CameraState::Player)
            ))
            .add_systems(Update, interpolate_camera.run_if(in_state(CameraState::Indipendent)))
            .add_systems(Update, interpolate_camera_2.run_if(in_state(CameraState::Indipendent)));
    }
}

pub fn player_controller_render(
    //mut render_query: Query<(&mut Transform, &RenderPlayer), With<RenderPlayer>>,
    mut render_query: Query<(&mut Transform, &RenderPlayer)>,
    logical_query: Query<
        (&Transform, &Collider, &PlayerController, &CameraConfig),
        Without<RenderPlayer>,
    >,
) {
    trace!("SYSTEM: player_controller_render");

    for (mut render_transform, render_player) in render_query.iter_mut() {
        if let Ok((logical_transform, collider, controller, camera_config)) =
            logical_query.get(render_player.logical_entity)
        {
            let collider_offset = collider_y_offset(collider);
            let camera_offset = Vec3::Y * camera_config.height_offset;
            render_transform.translation =
                logical_transform.translation + collider_offset + camera_offset;
            render_transform.rotation =
                Quat::from_euler(EulerRot::YXZ, controller.yaw, controller.pitch, 0.0);
        }
    }
}

/// Returns the offset that puts a point at the center of the player transform to the bottom of the collider.
/// Needed for when we want to originate something at the foot of the player.
fn collider_y_offset(collider: &Collider) -> Vec3 {
    trace!("SYSTEM: collider_y_offset");
    let collider = collider.shape();
    Vec3::Y
        * if let Some(cylinder) = collider.as_cylinder() {
            cylinder.half_height
        } else if let Some(capsule) = collider.as_capsule() {
            capsule.half_height() + capsule.radius
        } else {
            panic!("Controller must use a cylinder or capsule collider")
        }
}

fn interpolate_camera(
    mut commands: Commands,
    time: Res<Time>,
    mut camera_query: Query<(Entity, &CameraInterpolation, &mut Transform, &mut RenderPlayer)>,
    logical_query: Query<
        (&Transform, &Collider, &PlayerController, &CameraConfig),
        Without<RenderPlayer>,
    >,
    mut camera_state: ResMut<NextState<CameraState>>,
) {
    for (
        camera_entity,
        camera_interp,
        mut camera_transform,
        render_player,
    ) in camera_query.iter_mut() {
        if let Ok((logical_transform, logical_collider, logical_controller, logical_camera_config)) = logical_query.get(render_player.logical_entity) {

            if camera_interp.duration <= time.elapsed() {
                commands.entity(camera_entity).remove::<CameraInterpolation>();
                camera_state.set(CameraState::Player);
                return;
            }

            let collider_offset = collider_y_offset(logical_collider);
            let camera_offset = Vec3::Y * logical_camera_config.height_offset;
            let desired_transform = logical_transform.translation + collider_offset + camera_offset;
            let desired_rotation = Quat::from_euler(EulerRot::YXZ, logical_controller.yaw, logical_controller.pitch, 0.0);
            let normalized_time = (time.elapsed() - camera_interp.start_time).div_duration_f32(camera_interp.duration - time.elapsed());
            let ease_function = EaseFunction::SmoothStep;

            if let Some(ease_normal) = ease_function.sample(normalized_time) {
                camera_transform.translation = camera_transform.translation.slerp(desired_transform, ease_normal);
                camera_transform.rotation = camera_transform.rotation.slerp(desired_rotation, ease_normal);
            } else {
                commands.entity(camera_entity).remove::<CameraInterpolation>();
                camera_state.set(CameraState::Player);
                return;
            }
        }
    }
}

fn interpolate_camera_2(
    mut commands: Commands,
    time: Res<Time>,
    mut camera_query: Query<(Entity, &CameraInterpolation2, &mut Transform, &mut RenderPlayer)>,
    player_query: Query<Entity, With<Player>>,
    logical_query: Query<
        (&Collider, &CameraConfig),
        Without<RenderPlayer>,
    >,
) {
    for (
        camera_entity,
        camera_interp,
        mut camera_transform,
        render_player,
    ) in camera_query.iter_mut() {
        if let Ok((logical_collider, logical_camera_config)) = logical_query.get(render_player.logical_entity)
        && let Ok(player_entity) = player_query.single() {

            if camera_interp.duration <= time.elapsed() {
                commands.entity(camera_entity).remove::<CameraInterpolation2>();
                return;
            }

            let collider_offset = collider_y_offset(logical_collider);
            let camera_offset = Vec3::Y * logical_camera_config.height_offset;
            let desired_transform = camera_interp.desired_pos.translation + collider_offset + camera_offset;
            let desired_rotation = camera_interp.desired_pos.rotation;
            let normalized_time = (time.elapsed() - camera_interp.start_time).div_duration_f32(camera_interp.duration - time.elapsed());
            let ease_function = EaseFunction::SmoothStep;

            if let Some(ease_normal) = ease_function.sample(normalized_time) {
                camera_transform.translation = camera_transform.translation.slerp(desired_transform, ease_normal);
                camera_transform.rotation = camera_transform.rotation.slerp(desired_rotation, ease_normal);
            } else {
                commands.entity(camera_entity).remove::<CameraInterpolation2>();
                return;
            }
        }
    }
}
