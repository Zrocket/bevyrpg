use bevy::prelude::*;
use avian3d::collision::collider::Collider;

use crate::{GameState, PlayerController};

#[derive(Component)]
pub struct CameraConfig {
    pub height_offset: f32,
}

#[derive(Component)]
pub struct RenderPlayer {
    pub logical_entity: Entity,
}

pub struct GameRenderPlugin;

impl Plugin for GameRenderPlugin {
    fn build(&self, app: &mut App) {
        info!("GameRenderPlugin build");
        app.add_systems(
            Update,
            player_controller_render.run_if(in_state(GameState::Gameplay)),
        );
    }
}

pub fn player_controller_render(
    mut render_query: Query<(&mut Transform, &RenderPlayer), With<RenderPlayer>>,
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
