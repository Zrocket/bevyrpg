use bevy::prelude::*;
use bevy_tnua::{TnuaController, prelude::TnuaBuiltinWalk};

use crate::{PlayerControlScheme, Rover, RoverCamera};

pub(crate) const ROVER_ROATION_SPEED: f32 = 0.01;

#[derive(Component, Default, Debug)]
pub struct RoverMovementInput {
    pub rotation: Quat,
    pub movement: Vec3,
    pub cam_rot: Quat,
}

#[derive(EntityEvent)]
pub struct RoverForwardEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct RoverBackwardEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct RoverLeftEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct RoverRightEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct RoverCameraUpEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct RoverCameraDownEvent {
    pub entity: Entity,
}

pub(crate) fn apply_rover_movement(
    mut rover_query: Query<(&mut TnuaController<PlayerControlScheme>, &RoverMovementInput, &mut Transform), With<Rover>>,
) {
    if let Ok((mut tnua_controller, input, mut transform)) = rover_query.single_mut() {
        tnua_controller.initiate_action_feeding();

        tnua_controller.basis = TnuaBuiltinWalk {
            desired_motion: input.movement.normalize_or_zero(),
            ..default()
        };

        transform.rotate(input.rotation);
    }
}

pub(crate) fn on_rover_forward_observer(
    _trigger: On<RoverForwardEvent>,
    mut rover_query: Query<(&GlobalTransform, &mut RoverMovementInput), With<Rover>>,
    mut toggle: Local<bool>,
) {
    if let Ok((global_transform, mut input)) = rover_query.single_mut() {
        let mut move_to_world = Mat3::from_quat(global_transform.rotation());
        move_to_world.z_axis *= -1.0;
        move_to_world.y_axis = Vec3::Y;
        let movement_direction = move_to_world * Vec3::Z;

        if !*toggle {
            input.movement = movement_direction;
            *toggle = true;
        } else {
            input.movement = Vec3::ZERO;
            *toggle = false;
        }

    }
}

pub(crate) fn on_rover_backward_observer(
    _trigger: On<RoverBackwardEvent>,
    mut rover_query: Query<(&GlobalTransform, &mut RoverMovementInput), With<Rover>>,
    mut toggle: Local<bool>,
) {
    if let Ok((global_transform, mut input)) = rover_query.single_mut() {
        let mut move_to_world = Mat3::from_quat(global_transform.rotation());
        move_to_world.z_axis *= -1.0;
        move_to_world.y_axis = Vec3::Y;
        let movement_direction = move_to_world * -Vec3::Z;

        if !*toggle {
            input.movement = movement_direction;
            *toggle = true;
        } else {
            input.movement = Vec3::ZERO;
            *toggle = false;
        }
    }
}

pub(crate) fn on_rover_right_observer(
    _trigger: On<RoverRightEvent>,
    mut rover_query: Query<&mut RoverMovementInput, With<Rover>>,
    mut toggle: Local<bool>,
) {
    if let Ok(mut input) = rover_query.single_mut() {
        if !*toggle {
            input.rotation = Quat::from_rotation_y(-ROVER_ROATION_SPEED);
            *toggle = true;
        } else {
            input.rotation = Quat::from_rotation_y(0.);
            *toggle = false;
        }
    }
}

pub(crate) fn on_rover_left_observer(
    _trigger: On<RoverLeftEvent>,
    mut rover_query: Query<&mut RoverMovementInput, With<Rover>>,
    mut toggle: Local<bool>,
) {
    if let Ok(mut input) = rover_query.single_mut() {
        if !*toggle {
            input.rotation = Quat::from_rotation_y(ROVER_ROATION_SPEED);
            *toggle = true;
        } else {
            input.rotation = Quat::from_rotation_y(0.);
            *toggle = false;
        }
    }
}

pub(crate) fn on_rover_camera_up_observer(
    trigger: On<RoverCameraUpEvent>,
    mut camera_query: Query<&mut Transform, With<RoverCamera>>,
    mut toggle: Local<bool>,
) {
    if let Ok(mut camera_transform) = camera_query.single_mut() {
        if !*toggle {
            camera_transform.rotate(Quat::from_rotation_x(0.1));
            *toggle = true;
        } else {
            *toggle = false;
        }
    }
}

pub(crate) fn on_rover_camera_down_observer(
    trigger: On<RoverCameraDownEvent>,
    mut camera_query: Query<&mut Transform, With<RoverCamera>>,
    mut toggle: Local<bool>,
) {
    if let Ok(mut camera_transform) = camera_query.single_mut() {
        if !*toggle {
            camera_transform.rotate(Quat::from_rotation_x(-0.1));
            *toggle = true;
        } else {
            *toggle = false;
        }
    }
}
