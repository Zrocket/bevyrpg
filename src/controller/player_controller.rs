use std::{cmp::Ordering, f32::consts::*};

use avian3d::{math::{AdjustPrecision, Vector3}, prelude::RigidBodyDisabled};
use bevy::{input::mouse, prelude::*};
use bevy_enhanced_input::{action::Action, prelude::{ActionEvents, Start}};
use bevy_tnua::{
    TnuaControllerPlugin, TnuaObstacleRadar, TnuaScheme, TnuaUserControlsSystems, builtins::{TnuaBuiltinClimb, TnuaBuiltinCrouch, TnuaBuiltinDash, TnuaBuiltinJump, TnuaBuiltinKnockback, TnuaBuiltinWalk, TnuaBuiltinWallSlide}, control_helpers::{TnuaAirActionDefinition, TnuaAirActionsTracker, TnuaBlipReuseAvoidance, TnuaHasTargetEntity, TnuaSimpleAirActionsCounter}, controller::TnuaController, math::{AsF32, Float}, radar_lens::{TnuaBlipSpatialRelation, TnuaRadarLens}
};
use bevy_tnua_avian3d::{TnuaAvian3dPlugin, TnuaSpatialExtAvian3d};

use crate::{CrouchAction, DownAction, FlashlightAction, JumpAction, LookAction, MovementAction, ObstacleQueryHelper, Player, PlayerFlashlight, PlayerState, RunAction, UpAction};

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
            sensitivity: 0.01,
        }
    }
}

#[derive(TnuaScheme)]
#[scheme(basis = TnuaBuiltinWalk)]
pub enum PlayerControlScheme {
    Jump(TnuaBuiltinJump),
    Crouch(TnuaBuiltinCrouch),
    Dash(TnuaBuiltinDash),
    Knockback(TnuaBuiltinKnockback),
    WallSlide(TnuaBuiltinWallSlide, Entity),
    WallJump(TnuaBuiltinJump),
    Climb(
        TnuaBuiltinClimb,
        Entity,
        Vector3,
    ),
}

impl TnuaAirActionDefinition for PlayerControlScheme {
    fn is_air_action(action: Self::ActionDiscriminant) -> bool {
        match action {
            PlayerControlSchemeActionDiscriminant::Jump => true,
            PlayerControlSchemeActionDiscriminant::Crouch => false,
            PlayerControlSchemeActionDiscriminant::Dash => false,
            PlayerControlSchemeActionDiscriminant::Knockback => true,
            PlayerControlSchemeActionDiscriminant::WallSlide => true,
            PlayerControlSchemeActionDiscriminant::WallJump => true,
            PlayerControlSchemeActionDiscriminant::Climb => true,
        }
    }
}

impl TnuaHasTargetEntity for PlayerControlScheme {
    fn target_entity(action_state: &Self::ActionState) -> Option<Entity> {
        match action_state {
            PlayerControlSchemeActionState::Jump(_) => None,
            PlayerControlSchemeActionState::Crouch(_) => None,
            PlayerControlSchemeActionState::Dash(_) => None,
            PlayerControlSchemeActionState::Knockback(_) => None,
            PlayerControlSchemeActionState::WallSlide(_, entity) => Some(*entity),
            PlayerControlSchemeActionState::WallJump(_) => None,
            PlayerControlSchemeActionState::Climb(_, entity, _) => Some(*entity),
        }
    }
}

#[derive(Component, Debug, PartialEq, Default)]
pub enum FallingThroughControlScheme {
    JumpThroughOnly,
    WithoutHelper,
    #[default]
    SingleFall,
    KeepFalling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimensionality {
    Dim2,
    Dim3,
}

#[derive(Component)]
pub struct PlayerControllerConfig {
    pub dimensionality: Dimensionality,
    pub actions_in_air: usize,
    pub one_way_platforms_min_proximity: Float,
    pub falling_through: FallingThroughControlScheme,
}

impl Default for PlayerControllerConfig {
    fn default() -> Self {
        Self {
            dimensionality: Dimensionality::Dim3,
            actions_in_air: 1,
            one_way_platforms_min_proximity: 1.0,
            falling_through: FallingThroughControlScheme::SingleFall,
        }
    }
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
        app
            .add_observer(toggle_flashlight)
            .add_plugins(TnuaAvian3dPlugin::new(FixedUpdate))
            .add_plugins(TnuaControllerPlugin::<PlayerControlScheme>::new(FixedUpdate))
            .add_systems(
            Update,
            (
                player_controller_input,
                player_controller_look,
            )
            .chain()
        )
        .add_systems(Update, tnua_player_input.in_set(TnuaUserControlsSystems));
    }
}

#[allow(clippy::complexity)]
pub fn player_controller_input(
    movement_action: Single<&Action<MovementAction>>,
    look_action: Single<&Action<LookAction>>,
    down_action: Single<&ActionEvents, With<Action<DownAction>>>,
    up_action: Single<&ActionEvents, With<Action<UpAction>>>,
    run_action: Single<&ActionEvents, With<Action<RunAction>>>,
    mut mouse_events_reader: MessageReader<mouse::MouseMotion>,
    mut player_controller_query: Query<(&PlayerController, &mut PlayerControllerInput)>,
) {
    for (player_controller, mut player_input) in player_controller_query
        .iter_mut()
            .filter(|(controller, _)| controller.enable_input)
            {
                let mut mouse_delta = Vec2::ZERO;
                if mouse_events_reader.is_empty() {
                    mouse_delta[0] += look_action[0];
                    mouse_delta[1] -= look_action[1];
                } else {
                    for mouse_event in mouse_events_reader.read() {
                        mouse_delta += mouse_event.delta;
                        mouse_delta *= player_controller.sensitivity;
                    }
                }

                player_input.pitch = (player_input.pitch - mouse_delta.y)
                    .clamp(-FRAC_PI_2 + ANGLE_EPSILON, FRAC_PI_2 - ANGLE_EPSILON);
                player_input.yaw -= mouse_delta.x;
                if player_input.yaw.abs() > PI {
                    player_input.yaw = player_input.yaw.rem_euclid(TAU);
                }

                let up: f32 = up_action.contains(ActionEvents::FIRE).into();
                let down: f32 = down_action.contains(ActionEvents::FIRE).into();

                println!("LOOK: {:?}, {:?}", look_action[0], look_action[1]);

                player_input.movement = Vec3::new(
                    movement_action[0],
                    up - down,
                    movement_action[1]
                );
                player_input.sprint = run_action.contains(ActionEvents::START);
            }
}

pub fn player_controller_look(mut query: Query<(&mut PlayerController, &PlayerControllerInput)>) {
    for (mut controller, input) in query.iter_mut() {
        controller.pitch = input.pitch;
        controller.yaw = input.yaw;
    }
}

fn toggle_flashlight(
    _trigger: On<Start<FlashlightAction>>,
    mut flashlight_query: Query<&mut SpotLight, With<PlayerFlashlight>>,
) {
    if let Ok(mut flashlight) = flashlight_query.single_mut() {
        if flashlight.intensity == 0. {
            flashlight.intensity = 1_000_000.0;
        } else {
            flashlight.intensity = 0.;
        }
    }
}

// Query for the `ActionState` component in your game logic systems!
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn tnua_player_input(
    mut commands: Commands,
    jump_action: Single<&ActionEvents, With<Action<JumpAction>>>,
    run_action: Single<&ActionEvents, With<Action<RunAction>>>,
    crouch_action: Single<&ActionEvents, With<Action<CrouchAction>>>,
    mut tnua_query: Query<(
        &PlayerControllerConfig,
        &mut TnuaController<PlayerControlScheme>,
        &mut TnuaSimpleAirActionsCounter<PlayerControlScheme>,
        &mut PlayerState,
        //&ActionState<LeafwingAction>,
        &PlayerControllerInput,
        &TnuaObstacleRadar,
        &mut TnuaBlipReuseAvoidance<PlayerControlScheme>,
        Entity,
        ), With<Player>>,
    spatial_ext: TnuaSpatialExtAvian3d,
    obstacle_query: Query<ObstacleQueryHelper>,
    asset_server: Res<AssetServer>,
) {
    // Get player's tnua controller, otherwise return
    let Ok((player_controller_config,
            mut tnua_controller,
            mut air_actions_counter,
            mut player_state,
            //action_state,
            player_controller_input,
            obstacle_radar,
            mut blip_reuse_avoiodance,
            player_entity
            )) = tnua_query.single_mut() else {
        return;
    };

    tnua_controller.initiate_action_feeding();

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
    //if action_state.pressed(&LeafwingAction::Jump) {
    if jump_action.contains(ActionEvents::FIRE) {
        tnua_controller.action(PlayerControlScheme::Jump(Default::default()));
        if *player_state == PlayerState::Sitting {
            *player_state = PlayerState::Grounded;
            commands.entity(player_entity).remove::<RigidBodyDisabled>();
        }
    }

    //if action_state.just_pressed(&LeafwingAction::Crouch) {
    if crouch_action.contains(ActionEvents::START) {
        tnua_controller.action_start(PlayerControlScheme::Crouch(Default::default()));
    }
    //if action_state.just_released(&LeafwingAction::Crouch) {
    if crouch_action.contains(ActionEvents::COMPLETE) {
        tnua_controller.action_end(PlayerControlSchemeActionDiscriminant::Crouch);
    }

    if *player_state == PlayerState::Sitting {
        return;
    }

    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    tnua_controller.basis = TnuaBuiltinWalk {
        desired_motion: movement_direction.normalize_or_zero(),
        ..default()
    };

    //if action_state.just_pressed(&LeafwingAction::Run) {
    if run_action.contains(ActionEvents::START) {
        tnua_controller.action_trigger(PlayerControlScheme::Dash(TnuaBuiltinDash {
            displacement: movement_direction.normalize_or_zero(),
            ..default()
            }));
    }

    let radar_lens = TnuaRadarLens::new(obstacle_radar, &spatial_ext);

    let already_sliding_on = if let Some(PlayerControlSchemeActionState::WallSlide(_, entity)) =
        tnua_controller.current_action.as_ref()
        && obstacle_radar.has_blip(*entity)
    {
        Some(*entity)
    } else {
        None
    };

    let already_climbing_on =
        if let Some(PlayerControlSchemeActionState::Climb(state, entity, initiation_direction)) =
            tnua_controller.current_action.as_ref()
                && obstacle_radar.has_blip(*entity) {
                    Some((*entity, state.input.clone(), *initiation_direction))
                } else {
                    None
                };

    let mut walljump_candidate = None;
    let screen_space_direction = movement_direction.clamp_length_max(1.0);


    'blips_loop: for blip in radar_lens.iter_blips() {
        if !blip_reuse_avoiodance.should_avoid(blip.entity())
            && obstacle_query
                .get(blip.entity())
                .expect("ObstacleQueryHelper has nothing that could fail when missing")
                .climbable
        {
            if let Some((climbable_entity, action, initiation_direction)) =
                already_climbing_on.as_ref()
            {
                if *climbable_entity != blip.entity() {
                    continue 'blips_loop;
                }
                let dot_initiation = movement_direction.dot(*initiation_direction);
                let initiation_direction = if 0.5 < dot_initiation {
                    *initiation_direction
                } else {
                    Vector3::ZERO
                };
                if initiation_direction == Vector3::ZERO {
                    let right_left = screen_space_direction.dot(Vector3::X);
                    if 0.5 <= right_left.abs() {
                        continue 'blips_loop;
                    }
                }

                let mut action = TnuaBuiltinClimb {
                    anchor: blip.closest_point().get(),
                    desired_climb_motion: screen_space_direction.dot(Vector3::NEG_Z)
                        * Vector3::Y,
                    desired_vec_to_anchor: action.desired_vec_to_anchor,
                    desired_forward: action.desired_forward,
                    ..default()
                };

                const LOOK_ABOVE_OR_BELOW: Float = 5.0;
                match action
                    .desired_climb_motion
                    .dot(Vector3::Y)
                    .partial_cmp(&0.0)
                    .unwrap()
                {
                    Ordering::Less => {
                        if tnua_controller.is_airborne().unwrap() {
                            let extent = blip
                                .probe_extent_from_closest_point(-Dir3::Y, LOOK_ABOVE_OR_BELOW);
                            if extent < 0.9 * LOOK_ABOVE_OR_BELOW {
                                action.hard_stop_down =
                                    Some(blip.closest_point().get() - extent * Vector3::Y);
                            }
                        } else if initiation_direction == Vector3::ZERO {
                            continue 'blips_loop;
                        } else {
                            action.desired_climb_motion = Vector3::ZERO;
                        }
                    }
                    Ordering::Equal => {
                    }
                    // Climbing up
                    Ordering::Greater => {
                        let extent =
                            blip.probe_extent_from_closest_point(Dir3::Y, LOOK_ABOVE_OR_BELOW);
                        if extent < 0.9 * LOOK_ABOVE_OR_BELOW {
                            action.hard_stop_up =
                                Some(blip.closest_point().get() + extent * Vector3::Y);
                        }
                    }
                }

                tnua_controller.action(PlayerControlScheme::Climb(
                        action,
                        blip.entity(),
                        initiation_direction,
                ));
            } else if let TnuaBlipSpatialRelation::Aeside(blip_drection) =
                blip.spatial_relation(0.5)
                && 0.5 < movement_direction.dot(blip_drection.adjust_precision())
            {
                let direction_to_anchor = match player_controller_config.dimensionality {
                    Dimensionality::Dim2 => Vector3::ZERO,
                    Dimensionality::Dim3 => -blip
                        .normal_from_closest_point()
                        .reject_from_normalized(Vector3::Y),
                };
                tnua_controller.action(PlayerControlScheme::Climb(
                        TnuaBuiltinClimb {
                            anchor: blip.closest_point().get(),
                            desired_vec_to_anchor: 0.5 * direction_to_anchor,
                            desired_forward: Dir3::new(direction_to_anchor.f32()).ok(),
                            ..default()
                        },
                        blip.entity(),
                        movement_direction.normalize_or_zero(),
                ));
            }
        }
        if !blip.is_interactable() {
            continue;
        }
        match blip.spatial_relation(0.5) {
            TnuaBlipSpatialRelation::Invalid => {}
            TnuaBlipSpatialRelation::Above => {}
            TnuaBlipSpatialRelation::Below => {}
            TnuaBlipSpatialRelation::Aeside(blip_drection) => {
                let dot_thresholdl = if already_sliding_on == Some(blip.entity()) {
                    -0.1
                } else {
                    0.0
                };
                if tnua_controller.is_airborne().unwrap() {
                    let dot_direction = movement_direction.dot(blip_drection.adjust_precision());
                    if dot_direction <= -0.7 {
                        if let Some((best_entity, best_dot, best_direction)) =
                            walljump_candidate.as_mut()
                        {
                            if *best_dot < dot_direction {
                                *best_entity = blip.entity();
                                *best_dot = dot_direction;
                                *best_direction = blip_drection;
                            }
                        } else {
                            walljump_candidate =
                                Some((blip.entity(), dot_direction, blip_drection));
                        }
                    }
                    if dot_thresholdl < dot_direction
                        && 0.8 < blip.flat_wall_score(Dir3::Y, &[-1.0, 1.0])
                    {
                        let Ok(normal) = Dir3::new(blip.normal_from_closest_point().f32())
                        else {
                            continue;
                        };
                        tnua_controller.action(PlayerControlScheme::WallSlide(
                                TnuaBuiltinWallSlide {
                                    contact_point_with_wall: blip.closest_point().get(),
                                    normal,
                                    force_forward: Some(blip_drection),
                                },
                                blip.entity(),
                        ));
                    }
                }
            }
        }
    }
}
