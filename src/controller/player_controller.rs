use std::f32::consts::*;

use avian3d::{math::{AdjustPrecision, Vector3}, prelude::RigidBodyDisabled};
use bevy::{input::mouse, prelude::*, render::view::screenshot};
use bevy_tnua::{
    TnuaObstacleRadar, builtins::{TnuaBuiltinClimb, TnuaBuiltinCrouch, TnuaBuiltinDash, TnuaBuiltinJump, TnuaBuiltinKnockback, TnuaBuiltinWalk, TnuaBuiltinWallSlide}, control_helpers::{TnuaBlipReuseAvoidance, TnuaSimpleAirActionsCounter}, controller::TnuaController, math::{AsF32, Float}, radar_lens::{TnuaBlipSpatialRelation, TnuaRadarLens}
};
use bevy_tnua_avian3d::TnuaSpatialExtAvian3d;
use leafwing_input_manager::prelude::*;

use crate::{Player, PlayerFlashlight, PlayerState};

// Used as padding by camera pitching (up/down) to avoid spooky math problems
const ANGLE_EPSILON: f32 = 0.001953125;

// If the distance to the ground is less than this value, the player is considered grounded
const _GROUNDED_DISTANCE: f32 = 0.125;

const _SLIGHT_SCALE_DOWN: f32 = 0.9375;

#[derive(Component)]
pub struct PlayerController {
    pub pitch: f32,
    pub yaw: f32,
    pub enable_input: bool,
    pub sensitivity: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        PlayerController {
            pitch: 0.0,
            yaw: 0.0,
            enable_input: true,
            sensitivity: 0.001,
        }
    }
}

#[derive(Component)]
pub struct PlayerControllerConfig {
    pub speed: f32,
    pub walk: TnuaBuiltinWalk,
    pub air_actions: usize,
    pub jump: TnuaBuiltinJump,
    pub crouch: TnuaBuiltinCrouch,
    pub run_distance: f32,
    pub run: TnuaBuiltinDash,
    pub one_way_platforms_min_proximity: f32,
    pub knockback: TnuaBuiltinKnockback,
    pub wall_slide: TnuaBuiltinWallSlide,
    pub climb_speed: f32,
    pub climb: TnuaBuiltinClimb,
}

impl Default for PlayerControllerConfig {
    fn default() -> Self {
        Self {
            speed: 20.,
            walk: TnuaBuiltinWalk {
                float_height: 2.0,
                max_slope: FRAC_PI_4,
                ..default()
            },
            air_actions: 1,
            jump: TnuaBuiltinJump { 
                height: 4.0,
                ..default()
            },
            crouch: TnuaBuiltinCrouch {
                float_offset: -0.9,
                ..default()
            },
            run_distance: 10.0,
            run: TnuaBuiltinDash::default(),
            one_way_platforms_min_proximity: 1.0,
            knockback: TnuaBuiltinKnockback::default(),
            wall_slide: TnuaBuiltinWallSlide::default(),
            climb_speed: 10.0,
            climb: TnuaBuiltinClimb::default(),
        }
    }
}

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Run,
    Jump,
    Forward,
    Backward,
    Left,
    Right,
    Crouch,
    Up,
    Down,
    Interact,
    OpenInventory,
    OpenConsole,
    Flashlight,
}

#[derive(Component, Default, Debug)]
pub struct PlayerControllerInput {
    pub sprint: bool,
    pub jump: bool,
    pub crouch: bool,
    pub pitch: f32,
    pub yaw: f32,
    pub movement: Vec3,
}

pub struct PlayerControllerPlugin;
impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_controller_input,
                player_controller_look,
                tnua_player_input,
                toggle_flashlight,
            )
            .chain()
        );
    }
}

pub fn player_controller_input(
    key_input_query: Query<&ActionState<Action>, With<Player>>,
    mut mouse_events_reader: MessageReader<mouse::MouseMotion>,
    mut player_controller_query: Query<(&PlayerController, &mut PlayerControllerInput)>,
) {
    for (player_controller, mut player_input) in player_controller_query
        .iter_mut()
        .filter(|(controller, _)| controller.enable_input)
    {
        let mut mouse_delta = Vec2::ZERO;
        for mouse_event in mouse_events_reader.read() {
            mouse_delta += mouse_event.delta;
        }
        mouse_delta *= player_controller.sensitivity;

        player_input.pitch = (player_input.pitch - mouse_delta.y)
            .clamp(-FRAC_PI_2 + ANGLE_EPSILON, FRAC_PI_2 - ANGLE_EPSILON);
        player_input.yaw -= mouse_delta.x;
        if player_input.yaw.abs() > PI {
            player_input.yaw = player_input.yaw.rem_euclid(TAU);
        }

        if let Ok(key_input) = key_input_query.single() {
            player_input.movement = Vec3::new(
                get_axis(key_input, &Action::Right, &Action::Left),
                get_axis(key_input, &Action::Up, &Action::Down),
                get_axis(key_input, &Action::Forward, &Action::Backward),
            );
            player_input.sprint = key_input.pressed(&Action::Run);
        }
    }
}

fn get_axis(key_input: &ActionState<Action>, key_pos: &Action, key_neg: &Action) -> f32 {
    get_pressed(key_input, key_pos) - get_pressed(key_input, key_neg)
}

fn get_pressed(key_input: &ActionState<Action>, key: &Action) -> f32 {
    if key_input.pressed(key) { 1.0 } else { 0.0 }
}

pub fn player_controller_look(mut query: Query<(&mut PlayerController, &PlayerControllerInput)>) {
    for (mut controller, input) in query.iter_mut() {
        controller.pitch = input.pitch;
        controller.yaw = input.yaw;
    }
}

fn toggle_flashlight(
    key_input_query: Query<&ActionState<Action>, With<Player>>,
    mut flashlight_query: Query<&mut SpotLight, With<PlayerFlashlight>>,
) {
    if let Ok(mut flashlight) = flashlight_query.single_mut() && let Ok(key_input) = key_input_query.single() {
        if key_input.just_pressed(&Action::Flashlight) {
            if flashlight.intensity == 0. {
                flashlight.intensity = 1_000_000.0;
            } else {
                flashlight.intensity = 0.;
            }
        }
    }
}

// Query for the `ActionState` component in your game logic systems!
#[allow(clippy::type_complexity)]
pub fn tnua_player_input(
    mut commands: Commands,
    mut tnua_query: Query<(
        &PlayerControllerConfig,
        &mut TnuaController,
        &mut TnuaSimpleAirActionsCounter,
        &mut PlayerState,
        &ActionState<Action>,
        &PlayerControllerInput,
        &TnuaObstacleRadar,
        &mut TnuaBlipReuseAvoidance,
        Entity,
        ), With<Player>>,
    spatial_ext: TnuaSpatialExtAvian3d,
) {
    // Get player's tnua controller, otherwise return
    let Ok((player_controller_config,
            mut tnua_controller,
            mut air_actions_counter,
            mut player_state,
            action_state,
            player_controller_input,
            obstacle_radar,
            mut blip_reuse_avoiodance,
            player_entity
            )) = tnua_query.single_mut() else {
        return;
    };

    // Creates a 3D rotation matrix from a normalized rotation axis and angle (in radians).
    // returns a 3x3 column major matrix.
    let mut move_to_world = Mat3::from_axis_angle(Vec3::Y, player_controller_input.yaw);
    move_to_world.z_axis *= -1.0; // Forward is -Z
    move_to_world.y_axis = Vec3::Y; // Vertical movement aligned with world up
    let movement_direction = move_to_world * player_controller_input.movement;

    air_actions_counter.update(tnua_controller.as_mut());

    // This also needs to be called once per frame. It checks which obstacles needs to be
    // blocked - e.g. because we've just finished an action on them and we don't want to
    // reinitiate that action.
    blip_reuse_avoiodance.update(tnua_controller.as_ref(), obstacle_radar);

    // Each action has a button-like state of its own that you can check
    //println!(
    //    "Air Actions Counter: {}",
    //    air_actions_counter.air_count_for(TnuaBuiltinJump::NAME)
    //);
    //println!("Action State: {}", action_state.just_pressed(&Action::Jump));
    //if action_state.just_pressed(&Action::Jump) && air_actions_counter.air_count_for(TnuaBuiltinJump::NAME) == 0 {
    if action_state.pressed(&Action::Jump) {
        tnua_controller.action(TnuaBuiltinJump {
            allow_in_air: false,
            // The height is the only mandatory field of the jump button.
            height: 1.5,
            // `TnuaBuiltinJump` also has customization fields with sensible defaults.
            ..Default::default()
        });
        if *player_state == PlayerState::Sitting {
            *player_state = PlayerState::Grounded;
            commands.entity(player_entity).remove::<RigidBodyDisabled>();
        }
    }

    if action_state.pressed(&Action::Crouch) {
        tnua_controller.action(TnuaBuiltinCrouch {
            float_offset: -1.5,
            height_change_impulse_for_duration: 0.1,
            height_change_impulse_limit: 0.3,
            uncancellable: false,
        });
    }
    //air_actions_counter.update(tnua_controller.as_mut());

    let mut acceleration = 10.0;

    if player_controller_input.sprint {
        acceleration = 15.0;
    }

    if *player_state == PlayerState::Sitting {
        return;
    }

    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    tnua_controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: movement_direction.normalize_or_zero() * acceleration,
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 1.5,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });

    let already_climbing_on = 
        tnua_controller
        .concrete_action::<TnuaBuiltinClimb>()
        .and_then(|(action, _)| {
            let entity = action
                .climbable_entity
                .filter(|entity| obstacle_radar.has_blip(*entity))?;
            Some((entity, action.clone()))
        });

    let radar_lens = TnuaRadarLens::new(obstacle_radar, &spatial_ext);

    let screen_space_direction = player_controller_input.movement.clamp_length_max(1.0);

    'blips_loop: for blip in radar_lens.iter_blips() {
        if !blip_reuse_avoiodance.should_avoid(blip.entity()) {
            if let Some((climbable_entity, action)) = already_climbing_on.as_ref() {
                if *climbable_entity != blip.entity() {
                    continue 'blips_loop;
                }
                let dot_initiation = player_controller_input.movement.dot(action.initiation_direction);
                let initiation_direction = if 0.5 < dot_initiation {
                    action.initiation_direction
                } else {
                    Vector3::ZERO
                };
                if initiation_direction == Vector3::ZERO {
                    let right_left = screen_space_direction.dot(Vector3::X);
                    if 0.5 <= right_left.abs() {
                        continue 'blips_loop;
                    }
                }
            }
        }

        if let TnuaBlipSpatialRelation::Aeside(blip_direction) = blip.spatial_relation(0.5) {
            if 0.5 < player_controller_input.movement.dot(blip_direction.adjust_precision()).abs() {
                  let direction_to_anchor = blip.normal_from_closest_point().reject_from_normalized(Vector3::Y);
                  if let PlayerState::Ladder(ladder) = *player_state {
                      tnua_controller.action(TnuaBuiltinClimb {
                          climbable_entity: Some(ladder),
                          anchor: blip.closest_point().get(),
                          desired_vec_to_anchor: 0.5 * direction_to_anchor,
                          desired_forward: Dir3::new(direction_to_anchor.f32()).ok(),
                          initiation_direction: player_controller_input.movement.normalize_or_zero(),
                          desired_climb_velocity: Vector3::new(0., 10., 0.),
                          ..default()
                      });
                }
            }
        }

    }
}
