use avian_pickup::actor::{AvianPickupActor, AvianPickupActorHoldConfig};
use avian3d::prelude::{CoefficientCombine, Collider, CollisionLayers, Friction, GravityScale, LayerMask, LockedAxes, RigidBody, SpatialQuery, SpatialQueryFilter};
use bevy::{camera::Exposure, core_pipeline::tonemapping::Tonemapping, ecs::{lifecycle::HookContext, world::DeferredWorld}, input::common_conditions::input_just_pressed, pbr::{Atmosphere, AtmosphereSettings, ScatteringMedium}, post_process::bloom::Bloom, prelude::*, render::view::Hdr};
use bevy_egui::PrimaryEguiContext;
use bevy_enhanced_input::{{actions, bindings}, action::Action, condition::{hold::Hold, tap::Tap}};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_tnua::{TnuaConfig, TnuaController, TnuaObstacleRadar, builtins::{TnuaBuiltinClimbConfig, TnuaBuiltinCrouchConfig, TnuaBuiltinDashConfig, TnuaBuiltinJumpConfig, TnuaBuiltinWalkConfig, TnuaBuiltinWallSlideConfig}, control_helpers::{TnuaBlipReuseAvoidance, TnuaSimpleAirActionsCounter}};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;

use crate::{BackwardAction, CameraConfig, CharacterBundle, CollisionLayer, CrouchAction, DeathEvent, Description, DownAction, Experience, FlashlightAction, FloatHeight, ForwardAction, GameState, Health, Item, JumpAction, LeftAction, Mana, MaxHealth, MaxMana, OpenConsoleAction, OpenEquipAction, OpenInventoryAction, PlayerControlScheme, PlayerControlSchemeConfig, PlayerController, PlayerControllerConfig, PlayerControllerInput, RayHit, RenderPlayer, RightAction, TnuaPlayerController, UpAction, Walk, Weapon1Action, InteractAction, Interact2Action, OpenStatsAction, RunAction, Weapon2Action, Weapon3Action, Weapon4Action, Weight, add_to_inventory_observer, display_equip_event_observer, display_inventory_event_observer, display_quest_event_observer, display_stats_event_observer, level::DAGunAssets, remove_from_inventory_observer};

#[derive(Clone, Component, Hash, Debug, Eq, PartialEq, Default, States)]
pub enum PlayerState {
    #[default]
    Grounded,
    Ladder(Entity),
    UnGrounded,
    Sitting,
    NoClip,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
/*#[require(
    Name::new("Player"),
    Collider::capsule(0.1, 0.5),
    Friction {
        combine_rule: CoefficientCombine::Min,
        ..default()
    },
    RigidBody::Dynamic,
    LockedAxes::ROTATION_LOCKED,
    GravityScale(1.0),
    CameraConfig {
        height_offset: 0.0,
        //radius_scale: 0.75,
    },
    Walk::default(),
    PlayerState::Grounded,
)]*/
#[component(on_add = on_player_add)]
pub struct Player;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    Camera {
        clear_color: ClearColorConfig::Custom(Srgba::rgb(0.0, 0.0, 0.0).into()),
        ..default()
    },
    Hdr,
    Camera3d { ..default() },
    AtmosphereSettings {
        aerial_view_lut_max_distance: 3.2e5,
        scene_units_to_m: 1e+4,
        ..Default::default()
    },
    Exposure::SUNLIGHT,
    Tonemapping::AcesFitted,
    Bloom::NATURAL,
    Projection::Perspective(PerspectiveProjection {
        fov: std::f32::consts::PI / 2.0,
        ..default()
    }),
    Transform {
        translation: Vec3 { y: 2., ..default() },
        ..default()
    },
)]
#[component(on_add = on_player_camera_add)]
pub struct PlayerCamera;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerFlashlight;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerSpawner;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerTrigger;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ActiveWeapon;

#[derive(Message)]
pub struct SpawnPlayerMessage;

pub struct GamePlayerPlugin;
impl Plugin for GamePlayerPlugin {
    fn build(&self, app: &mut App) {
        info!("GamePlayerPlugin build");
        app.register_type::<Player>()
            .add_plugins(NoCameraPlayerPlugin)
            .insert_resource(bevy_flycam::KeyBindings {
                move_ascend: KeyCode::PageUp,
                move_descend: KeyCode::PageDown,
                ..default()
            })
            .register_type::<PlayerCamera>()
            .register_type::<PlayerSpawner>()
            .register_type::<PlayerTrigger>()
            .add_message::<SpawnPlayerMessage>()
            //.add_systems(OnEnter(GameState::Postload), spawn_player_observer)
            .add_systems(Update, spawn_player_observer.run_if(resource_exists::<DAGunAssets>))
            .add_systems(OnEnter(GameState::Postload), init_player)
            .add_systems(Update, (
                    player_forward.run_if(in_state(GameState::Gameplay)),
                    check_player_triggers.run_if(in_state(GameState::Gameplay)),
                )
            )
            .add_systems(Update, toggle_player_noclip.run_if(input_just_pressed(KeyCode::KeyB)));
    }
}

fn on_player_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(player_death_event_observer)
        .observe(add_to_inventory_observer::<Player>)
        .observe(remove_from_inventory_observer::<Player>)
        .observe(display_inventory_event_observer)
        .observe(display_quest_event_observer)
        .observe(display_equip_event_observer)
        .observe(display_stats_event_observer);
}

fn on_player_camera_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let mut scattering_mediums = world.resource_mut::<Assets<ScatteringMedium>>();
    let scattering_mediums_handle = scattering_mediums.add(ScatteringMedium::default());

    world.commands()
        .entity(context.entity)
        .insert(Atmosphere::earthlike(scattering_mediums_handle));
}

fn init_player(
    mut spawn_player_message_writer: MessageWriter<SpawnPlayerMessage>,
) {
    spawn_player_message_writer.write(SpawnPlayerMessage);
}

#[allow(clippy::too_many_arguments)]
fn spawn_player_observer(
    mut commands: Commands,
    mut spawn_player_message_reader: MessageReader<SpawnPlayerMessage>,
    asset_server: Res<AssetServer>,
    gun_assets: Res<DAGunAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    player_spawner_query: Query<&GlobalTransform, With<PlayerSpawner>>,
    mut player_camera_query: Query<&mut RenderPlayer, With<PlayerCamera>>,
    mut player_query: Query<Entity, With<Player>>,
    mut control_scheme_configs: ResMut<Assets<PlayerControlSchemeConfig>>,
) {
    trace!("SYSTEM: spawn_player");

    for _message in spawn_player_message_reader.read() {
        if let Ok(player) = player_query.single_mut() {
            commands.entity(player).despawn();
        }
        let mut spawn_point = Transform::from_xyz(0.0, 50.0, 0.0);

        if let Ok(player_spawner) = player_spawner_query.single() {
            spawn_point.translation = player_spawner.translation();
        }

        // Gun
        debug!("Creating Gun");
        //let temp = gun_assets.uzi.clone_weak();
        let uzi = gltf_assets.get(&gun_assets.uzi).unwrap().scenes[0].path().unwrap();
        //let temp = uzi.scenes[0].path().unwrap();
        let gun = commands
            .spawn((
                Transform::from_translation(vec3(0.1, -0.2, -0.5)),
                //SceneRoot(asset_server.load("guns/uzi.glb#Scene0")),
                SceneRoot(asset_server.load(uzi)),
                Item {
                    description: Description("gun".to_string()),
                    weight: Weight(0),
                },
                Name::new("gun"),
                ActiveWeapon,
            ))
            .id();

        let bei_input_map = actions!(Player[
            (
                Action::<Weapon1Action>::new(),
                bindings![KeyCode::Digit1],
            ),
            (
                Action::<Weapon2Action>::new(),
                bindings![KeyCode::Digit2],
            ),
            (
                Action::<Weapon3Action>::new(),
                bindings![KeyCode::Digit3],
            ),
            (
                Action::<Weapon4Action>::new(),
                bindings![KeyCode::Digit4],
            ),
            (
                Action::<JumpAction>::new(),
                bindings![KeyCode::Space],
            ),
            (
                Action::<RunAction>::new(),
                bindings![KeyCode::ShiftLeft],
            ),
            (
                Action::<LeftAction>::new(),
                bindings![KeyCode::KeyA],
            ),
            (
                Action::<RightAction>::new(),
                bindings![KeyCode::KeyD],
            ),
            (
                Action::<ForwardAction>::new(),
                bindings![KeyCode::KeyW],
            ),
            (
                Action::<BackwardAction>::new(),
                bindings![KeyCode::KeyS],
            ),
            (
                Action::<CrouchAction>::new(),
                bindings![KeyCode::ControlLeft],
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
                Tap::new(0.5),
                bindings![KeyCode::KeyE],
            ),
            (
                Action::<Interact2Action>::new(),
                Hold::new(0.5),
                bindings![KeyCode::KeyE],
            ),
            (
                Action::<OpenInventoryAction>::new(),
                bindings![KeyCode::KeyI],
            ),
            (
                Action::<OpenEquipAction>::new(),
                bindings![KeyCode::KeyK],
            ),
            (
                Action::<OpenStatsAction>::new(),
                bindings![KeyCode::KeyL],
            ),
            (
                Action::<OpenConsoleAction>::new(),
                bindings![KeyCode::Backslash],
            ),
            (
                Action::<FlashlightAction>::new(),
                bindings![KeyCode::KeyF],
            ),
        ]);

        // Player
        debug!("Creating Player");

        let logical_entity = commands
            .spawn((
                (
                    Collider::capsule(0.1, 0.5),
                    Friction {
                        combine_rule: CoefficientCombine::Min,
                        ..default()
                    },
                    RigidBody::Dynamic,
                    LockedAxes::ROTATION_LOCKED,
                    GravityScale(1.0),
                    spawn_point,
                    CameraConfig {
                        height_offset: 0.0,
                        //radius_scale: 0.75,
                    },
                    Player,
                    PlayerController::default(),
                    PlayerControllerInput::default(),
                    CharacterBundle {
                        mana: Mana(100),
                        max_mana: MaxMana(100),
                        health: Health(100),
                        max_health: MaxHealth(100),
                        experience: Experience(100),
                        ..default()
                    },
                    TnuaController::<PlayerControlScheme>::default(),
                    TnuaConfig::<PlayerControlScheme>(control_scheme_configs.add(PlayerControlSchemeConfig {
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
                    })),
                    TnuaAvian3dSensorShape(Collider::capsule(0.1, 0.5)),
                    FloatHeight(1.5),
                ),
                (CollisionLayers::new(CollisionLayer::Player, LayerMask::ALL),),
            ))
            .insert(Walk::default())
            .insert(bei_input_map)
            .insert(TnuaSimpleAirActionsCounter::<PlayerControlScheme>::default())
            .insert(AvianPickupActor {
                prop_filter: SpatialQueryFilter::from_mask(CollisionLayer::Prop),
                actor_filter: SpatialQueryFilter::from_mask(CollisionLayer::Player),
                obstacle_filter: SpatialQueryFilter::from_mask(CollisionLayer::Default),
                hold: AvianPickupActorHoldConfig {
                    pitch_range: -40.0_f32.to_radians()..=75.0_f32.to_radians(),
                    distance_to_allow_holding: 100.0,
                    ..default()
                },
                ..default()
            })
            .insert(RayHit(None))
            .insert(Name::new("Player"))
            .insert(PlayerState::Grounded)
            .insert(TnuaObstacleRadar::new(1.0, 3.0))
            .insert(TnuaBlipReuseAvoidance::<PlayerControlScheme>::default())
            .insert(PlayerControllerConfig::default())
            .observe(player_death_event_observer)
            .observe(add_to_inventory_observer::<Player>)
            .observe(remove_from_inventory_observer::<Player>)
            .observe(display_inventory_event_observer)
            .observe(display_equip_event_observer)
            .observe(display_stats_event_observer)
            .id();

        // Camera


        debug!("Creating Camera");
        if let Ok(mut render_player) = player_camera_query.single_mut() {
            *render_player = RenderPlayer { logical_entity };
        } else {
            let flashlight = commands
                .spawn((
                    SpotLight {
                        intensity: 1_000_000.0,
                        shadows_enabled: true,
                        ..default()
                    },
                    PlayerFlashlight,
                ))
                .id();
            commands
                .spawn((
                    Camera {
                        clear_color: ClearColorConfig::Custom(Srgba::rgb(0.0, 0.0, 0.0).into()),
                        ..default()
                    },
                    Hdr,
                    Camera3d { ..default() },
                    PrimaryEguiContext,
                    //Atmosphere::Earth,
                    AtmosphereSettings {
                        aerial_view_lut_max_distance: 3.2e5,
                        scene_units_to_m: 1e+4,
                        ..Default::default()
                    },
                    Exposure::SUNLIGHT,
                    Tonemapping::AcesFitted,
                    Bloom::NATURAL,
                    Projection::Perspective(PerspectiveProjection {
                        fov: std::f32::consts::PI / 2.0,
                        ..default()
                    }),
                    Transform {
                        translation: Vec3 { y: 2., ..default() },
                        ..default()
                    },
                    RenderPlayer { logical_entity },
                    PlayerCamera,
                ))
                .add_child(gun)
                .add_child(flashlight);
        }
    }
}

/*#[allow(clippy::too_many_arguments)]
fn spawn_player_observer(
    mut commands: Commands,
    mut spawn_player_message_reader: MessageReader<SpawnPlayerMessage>,
    asset_server: Res<AssetServer>,
    gun_assets: Res<DAGunAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    player_spawner_query: Query<&GlobalTransform, With<PlayerSpawner>>,
    mut player_camera_query: Query<&mut RenderPlayer, With<PlayerCamera>>,
    mut player_query: Query<Entity, With<Player>>,
) {
    trace!("SYSTEM: spawn_player");

    for _message in spawn_player_message_reader.read() {
        if let Ok(player) = player_query.single_mut() {
            commands.entity(player).despawn();
        }
        let mut spawn_point = Transform::from_xyz(0.0, 50.0, 0.0);

        if let Ok(player_spawner) = player_spawner_query.single() {
            spawn_point.translation = player_spawner.translation();
        }

        // Gun
        debug!("Creating Gun");
        //let temp = gun_assets.uzi.clone_weak();
        let uzi = gltf_assets.get(&gun_assets.uzi).unwrap().scenes[0].path().unwrap();
        //let temp = uzi.scenes[0].path().unwrap();
        let gun = commands
            .spawn((
                Transform::from_translation(vec3(0.1, -0.2, -0.5)),
                //SceneRoot(asset_server.load("guns/uzi.glb#Scene0")),
                SceneRoot(asset_server.load(uzi)),
                Item {
                    description: Description("gun".to_string()),
                    weight: Weight(0),
                },
                Name::new("gun"),
                ActiveWeapon,
            ))
            .id();

        // Player
        debug!("Creating Player");

        let logical_entity = commands
            .spawn((
                (
                    spawn_point,
                    Player,
                    TnuaPlayerController,
                    CharacterBundle {
                        mana: Mana(100),
                        max_mana: MaxMana(100),
                        health: Health(100),
                        max_health: MaxHealth(100),
                        experience: Experience(100),
                        ..default()
                    },
                ),
            ))
            .id();

        // Camera
        debug!("Creating Camera");
        if let Ok(mut render_player) = player_camera_query.single_mut() {
            *render_player = RenderPlayer { logical_entity };
        } else {
            let flashlight = commands
                .spawn((
                    SpotLight {
                        intensity: 1_000_000.0,
                        shadows_enabled: true,
                        ..default()
                    },
                    PlayerFlashlight,
                ))
                .id();
            commands
                .spawn((
                    PrimaryEguiContext,
                    RenderPlayer { logical_entity },
                    PlayerCamera,
                ))
                .add_child(gun)
                .add_child(flashlight);
        }
    }
}*/

fn toggle_player_noclip(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut PlayerState), With<Player>>,
    mut player_camera_query: Query<Entity, With<PlayerCamera>>,
) {
    if let Ok((player_entity, mut player_state)) = player_query.single_mut()
    && let Ok(player_camera) = player_camera_query.single_mut() {
        if *player_state == PlayerState::NoClip {
            commands.entity(player_camera).remove::<FlyCam>();
            commands.entity(player_entity).insert(Collider::capsule(0.1, 0.5));
            *player_state = PlayerState::Grounded;
        } else {
            commands.entity(player_entity).remove::<Collider>();
            commands.entity(player_camera).insert(FlyCam);
            *player_state = PlayerState::NoClip;
        }
    }
}

fn player_forward(
    cam_transform: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
    mut player_transform: Query<&mut Transform, With<Player>>,
) {
    trace!("SYSTEM: player_forward");
    if let Ok(cam_transform) = cam_transform.single() &&
        let Ok(mut player_transform) = player_transform.single_mut() {
            let forward = cam_transform.forward();
            player_transform.look_to(*forward, Vec3::Y);
    }


    /*if let Ok(cam_transform) = cam_transform.single() {
        let forward = cam_transform.forward();
        if let Ok(mut player_transform) = player_transform.single_mut() {
            player_transform.look_to(*forward, Vec3::Y);
        }
    }*/
}

fn check_player_triggers(
    spatial_query: SpatialQuery,
    _player_query: Query<&Collider, With<Player>>,
    trigger_query: Query<&GlobalTransform, With<PlayerTrigger>>,
) {
    for trigger_transform in trigger_query.iter() {
        let _temp = spatial_query.shape_intersections(
            &Collider::cuboid(1.0, 1.0, 1.0),
            trigger_transform.translation(),
            trigger_transform.rotation(),
            &SpatialQueryFilter::from_mask(CollisionLayer::Player)
        );
    }
}

fn player_death_event_observer(
    _trigger: On<DeathEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_state.set(GameState::GameOver);
}
