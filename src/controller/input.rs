use std::f32::consts;

use avian3d::prelude::{Collider, RigidBody};
use bevy::{app::Plugin, asset::Assets, ecs::{component::Component, lifecycle::HookContext, spawn::SpawnRelated, world::DeferredWorld}, input::{gamepad::{GamepadButton}, keyboard::KeyCode, mouse::MouseButton}, math::Vec2, time::Timer, utils::default};
use bevy_enhanced_input::{action::Action, actions, bindings, prelude::{Axial, Bindings, Cardinal, DeadZone, EnhancedInputPlugin, Hold, InputAction, InputContextAppExt, SmoothNudge, Tap}, modifier::scale::Scale};
use bevy_tnua::{TnuaConfig, TnuaController, builtins::{TnuaBuiltinClimbConfig, TnuaBuiltinCrouchConfig, TnuaBuiltinDashConfig, TnuaBuiltinJumpConfig, TnuaBuiltinWalkConfig, TnuaBuiltinWallSlideConfig}, control_helpers::TnuaSimpleAirActionsCounter};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
//use bevy_bae::{plan::Plan, prelude::{Operator, Sequence}, tasks};
use bevy_tnua_physics_integration_layer::math::float_consts;

use crate::{IdleTimer, Player, PlayerControlScheme, PlayerControlSchemeConfig, Walk, /*idle, run_from_player, wander*/};

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_plugins(EnhancedInputPlugin)
            .add_input_context::<Player>();
    }
}
#[derive(InputAction)]
#[action_output(Vec2)]
pub struct MovementAction;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct LookAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Weapon1Action;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Weapon2Action;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Weapon3Action;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Weapon4Action;

#[derive(InputAction)]
#[action_output(bool)]
pub struct RunAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct JumpAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct CrouchAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct UpAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct DownAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct InteractAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Interact2Action;

#[derive(InputAction)]
#[action_output(bool)]
pub struct OpenInventoryAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct OpenEquipAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct OpenStatsAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct OpenConsoleAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct OpenQuestAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct FlashlightAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct ShootAction;

#[derive(Component)]
#[component(on_add = on_tnua_npc_controller_add)]
#[require(
    TnuaController::<PlayerControlScheme>::default(),
    TnuaAvian3dSensorShape(Collider::cuboid(0.5, 0.5, 0.5)),
    TnuaSimpleAirActionsCounter::<PlayerControlScheme>::default(),
    RigidBody::Dynamic,
    Walk::default(),
    //Plan::new(),
)]
pub struct TnuaNpcController;

#[derive(Component, Default)]
#[component(on_add = on_tnua_enemy_controller_add)]
#[require(
    TnuaController::<PlayerControlScheme>::default(),
    TnuaAvian3dSensorShape(Collider::cuboid(0.5, 0.5, 0.5)),
    TnuaSimpleAirActionsCounter::<PlayerControlScheme>::default(),
    RigidBody::Dynamic,
    Walk::default(),
    //Plan::new(),
)]
pub struct TnuaEnemyController;

#[derive(Component, Default)]
#[require(
    TnuaController::<PlayerControlScheme>::default(),
    TnuaAvian3dSensorShape(Collider::cuboid(0.5, 0.5, 0.5)),
    TnuaSimpleAirActionsCounter::<PlayerControlScheme>::default(),
    RigidBody::Dynamic,
    Walk::default(),
)]
#[component(on_add = on_tnua_rover_controller_add)]
pub struct TnuaRoverController;

#[derive(Component, Default)]
#[require(
)]
#[component(on_add = on_tnua_player_controller_add)]
pub struct TnuaPlayerController;

fn on_tnua_player_controller_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
        let bei_input_map = actions!(Player[
            (
                Action::<MovementAction>::new(),
                DeadZone::default(),
                //SmoothNudge::new(30.0),
                Bindings::spawn((
                    Cardinal::wasd_keys(),
                    Axial::left_stick(),
                )),
            ),
            (
                Action::<LookAction>::new(),
                DeadZone::default(),
                //SmoothNudge::new(10.0),
                SmoothNudge::default(),
                Scale::splat(0.08),
                Bindings::spawn((
                        //Spawn(Binding::mouse_motion()),
                        Axial::right_stick(),
                )),
            ),
            (
                Action::<Weapon1Action>::new(),
                bindings![KeyCode::Digit1, GamepadButton::DPadLeft],
            ),
            (
                Action::<Weapon2Action>::new(),
                bindings![KeyCode::Digit2, GamepadButton::DPadUp],
            ),
            (
                Action::<Weapon3Action>::new(),
                bindings![KeyCode::Digit3, GamepadButton::DPadRight],
            ),
            (
                Action::<Weapon4Action>::new(),
                bindings![KeyCode::Digit4, GamepadButton::DPadDown],
            ),
            (
                Action::<JumpAction>::new(),
                bindings![KeyCode::Space, GamepadButton::South],
            ),
            (
                Action::<RunAction>::new(),
                bindings![KeyCode::ShiftLeft, GamepadButton::LeftThumb],
            ),
            (
                Action::<CrouchAction>::new(),
                bindings![KeyCode::ControlLeft, GamepadButton::LeftTrigger2],
            ),
            (
                Action::<UpAction>::new(),
                bindings![KeyCode::KeyZ],
            ),
            (
                Action::<DownAction>::new(),
                bindings![KeyCode::KeyX],
            ),
            (
                Action::<InteractAction>::new(),
                Tap::new(0.3),
                bindings![KeyCode::KeyE, GamepadButton::West],
            ),
            (
                Action::<Interact2Action>::new(),
                Hold::new(0.5).one_shot(true),
                //Tap::new(0.5),
                bindings![KeyCode::KeyE, GamepadButton::West],
            ),
            (
                Action::<OpenInventoryAction>::new(),
                bindings![KeyCode::KeyI, GamepadButton::Select],
            ),
            (
                Action::<OpenEquipAction>::new(),
                bindings![KeyCode::KeyK, GamepadButton::Start],
            ),
            (
                Action::<OpenStatsAction>::new(),
                bindings![KeyCode::KeyL],
            ),
            (
                Action::<OpenQuestAction>::new(),
                bindings![KeyCode::KeyH],
            ),
            (
                Action::<OpenConsoleAction>::new(),
                bindings![KeyCode::Backslash],
            ),
            (
                Action::<FlashlightAction>::new(),
                bindings![KeyCode::KeyF, GamepadButton::East],
            ),
            (
                Action::<ShootAction>::new(),
                bindings![MouseButton::Left, GamepadButton::RightTrigger2],
            ),
        ]);

    let mut control_scheme_configs = world.resource_mut::<Assets<PlayerControlSchemeConfig>>();
    let handle = control_scheme_configs.add(PlayerControlSchemeConfig {
        basis: TnuaBuiltinWalkConfig {
            speed: 10.0,
            float_height: 1.5,
            max_slope: float_consts::FRAC_PI_8,
            ..default()
        },
        jump: TnuaBuiltinJumpConfig {
            height: 2.0,
            ..default()
        },
        crouch: TnuaBuiltinCrouchConfig {
            float_offset: -0.7,
            ..default()
        },
        dash: TnuaBuiltinDashConfig {
            horizontal_distance: 5.0,
            ..default()
        },
        knockback: Default::default(),
        wall_slide: TnuaBuiltinWallSlideConfig {
            maintain_distance: Some(0.7),
            ..default()
        },
        wall_jump: TnuaBuiltinJumpConfig {
            height: 4.0,
            takeoff_extra_gravity: 90.0, // 3 times the default
            takeoff_above_velocity: 0.0,
            horizontal_distance: 2.0,
            ..default()
        },
        climb: TnuaBuiltinClimbConfig {
            climb_speed: 10.0,
            ..default()
        },
    });

    world.commands()
        .entity(context.entity)
        .insert((
                TnuaConfig::<PlayerControlScheme>(handle),
                bei_input_map,
        ));
}

fn on_tnua_npc_controller_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let mut control_scheme_configs = world.resource_mut::<Assets<PlayerControlSchemeConfig>>();
    let handle = control_scheme_configs.add(PlayerControlSchemeConfig {
        basis: TnuaBuiltinWalkConfig {
            speed: 10.0,
            float_height: 1.5,
            ..default()
        },
        jump: TnuaBuiltinJumpConfig {
            height: 2.0,
            ..default()
        },
        crouch: TnuaBuiltinCrouchConfig {
            float_offset: -0.7,
            ..default()
        },
        dash: TnuaBuiltinDashConfig {
            horizontal_distance: 5.0,
            ..default()
        },
        knockback: Default::default(),
        wall_slide: TnuaBuiltinWallSlideConfig {
            maintain_distance: Some(0.7),
            ..default()
        },
        wall_jump: TnuaBuiltinJumpConfig {
            height: 4.0,
            takeoff_extra_gravity: 90.0, // 3 times the default
            takeoff_above_velocity: 0.0,
            horizontal_distance: 2.0,
            ..default()
        },
        climb: TnuaBuiltinClimbConfig {
            climb_speed: 10.0,
            ..default()
        },
    });


    /*world.commands()
        .entity(context.entity)
        .insert((
                TnuaConfig::<PlayerControlScheme>(handle),
        ))
        .insert((
            Sequence,
            tasks![
                Operator::new(wander),
                Operator::new(idle),
            ],
            IdleTimer(Timer::from_seconds(5.0, bevy::time::TimerMode::Once))
        ));*/
}

fn on_tnua_enemy_controller_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let mut control_scheme_configs = world.resource_mut::<Assets<PlayerControlSchemeConfig>>();
    let handle = control_scheme_configs.add(PlayerControlSchemeConfig {
        basis: TnuaBuiltinWalkConfig {
            speed: 10.0,
            float_height: 1.5,
            ..default()
        },
        jump: TnuaBuiltinJumpConfig {
            height: 2.0,
            ..default()
        },
        crouch: TnuaBuiltinCrouchConfig {
            float_offset: -0.7,
            ..default()
        },
        dash: TnuaBuiltinDashConfig {
            horizontal_distance: 5.0,
            ..default()
        },
        knockback: Default::default(),
        wall_slide: TnuaBuiltinWallSlideConfig {
            maintain_distance: Some(0.7),
            ..default()
        },
        wall_jump: TnuaBuiltinJumpConfig {
            height: 4.0,
            takeoff_extra_gravity: 90.0, // 3 times the default
            takeoff_above_velocity: 0.0,
            horizontal_distance: 2.0,
            ..default()
        },
        climb: TnuaBuiltinClimbConfig {
            climb_speed: 10.0,
            ..default()
        },
    });

    /*world.commands()
        .entity(context.entity)
        .insert((
                TnuaConfig::<PlayerControlScheme>(handle),
        ))
        .insert((
            Sequence,
            tasks![
                //Operator::new(wander),
                Operator::new(idle),
                Operator::new(run_from_player),
            ],
            IdleTimer(Timer::from_seconds(5.0, bevy::time::TimerMode::Once))
        ));*/
}

fn on_tnua_rover_controller_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let mut control_scheme_configs = world.resource_mut::<Assets<PlayerControlSchemeConfig>>();
    let handle = control_scheme_configs.add(PlayerControlSchemeConfig {
        basis: TnuaBuiltinWalkConfig {
            speed: 10.0,
            float_height: 0.5,
            ..default()
        },
        jump: TnuaBuiltinJumpConfig {
            height: 2.0,
            ..default()
        },
        crouch: TnuaBuiltinCrouchConfig {
            float_offset: -0.7,
            ..default()
        },
        dash: TnuaBuiltinDashConfig {
            horizontal_distance: 5.0,
            ..default()
        },
        knockback: Default::default(),
        wall_slide: TnuaBuiltinWallSlideConfig {
            maintain_distance: Some(0.7),
            ..default()
        },
        wall_jump: TnuaBuiltinJumpConfig {
            height: 4.0,
            takeoff_extra_gravity: 90.0, // 3 times the default
            takeoff_above_velocity: 0.0,
            horizontal_distance: 2.0,
            ..default()
        },
        climb: TnuaBuiltinClimbConfig {
            climb_speed: 10.0,
            ..default()
        },
    });

    world.commands()
        .entity(context.entity)
        .insert((
                TnuaConfig::<PlayerControlScheme>(handle),
        ));
}
