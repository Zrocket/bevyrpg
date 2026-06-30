use avian_pickup::actor::{AvianPickupActor, AvianPickupActorHoldConfig};
use avian3d::prelude::{CoefficientCombine, Collider, CollisionLayers, Friction, GravityScale, LayerMask, LockedAxes, RigidBody, SpatialQuery, SpatialQueryFilter, SweptCcd, CollisionMargin};
use bevy::{camera::Exposure, core_pipeline::tonemapping::Tonemapping, ecs::{lifecycle::HookContext, world::DeferredWorld}, pbr::{Atmosphere, AtmosphereSettings, ScatteringMedium}, post_process::bloom::Bloom, prelude::*, render::view::Hdr};
use bevy_egui::PrimaryEguiContext;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_tnua::{TnuaController, TnuaObstacleRadar, control_helpers::{TnuaBlipReuseAvoidance, TnuaSimpleAirActionsCounter}};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use bevy_seedling::spatial::SpatialListener3D;
use moonshine_save::prelude::Save;

use crate::{BootStrap, CameraConfig, CharacterBundle, CollisionLayer, DeathEvent, Description, Experience, FetchQuest, FloatHeight, GameState, Health, ItemDetails, Mana, MaxHealth, MaxMana, MetaState, PlayerControlScheme, PlayerController, PlayerControllerConfig, PlayerControllerInput, Quest, QuestOf, RayHit, RenderPlayer, Sleep, TnuaPlayerController, Walk, Weight, add_to_inventory_observer, display_equip_event_observer, display_inventory_event_observer, display_quest_event_observer, display_stats_event_observer, drink_event_observer, eat_event_observer, level::DAGunAssets, remove_from_inventory_observer};

/// The current state of the player Entity.
#[derive(Clone, Component, Hash, Debug, Eq, PartialEq, Default, States)]
pub enum PlayerState {
    #[default]
    Grounded,
    Computer,
    Ladder(Entity),
    UnGrounded,
    Sitting,
    Sleeping,
    NoClip,
}

/// The state of the player camera
///
/// CameraState::Player -> camrea follows the player entity
/// CameraState::Indipendent -> camera is moving indipendently of the player entity
///
/// Usefull for more cinematic camera control
#[derive(Clone, Component, Hash, Debug, Eq, PartialEq, Default, States)]
pub enum CameraState {
    #[default]
    Player,
    Indipendent,
}

/// The main player component.
///
/// Only one Entity should ever have a player component.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    Save,
    Name::new("Player"),
    TnuaPlayerController,
    CameraConfig {
        height_offset: 0.0,
        //radius_scale: 0.75,
    },
    Collider::capsule(0.1, 0.5),
    SweptCcd::default(),
    CollisionMargin(0.1),
    Friction {
        combine_rule: CoefficientCombine::Min,
        ..default()
    },
    SpatialListener3D,
    RayHit(None),
    Walk::default(),
    FloatHeight(1.5),
    RigidBody::Dynamic,
    LockedAxes::ROTATION_LOCKED,
    CollisionLayers::new(CollisionLayer::Player, LayerMask::ALL),
    GravityScale(1.0),
    //TnuaGravity(Vec3::ZERO),
    PlayerState::Grounded,
    TnuaObstacleRadar::new(1.0, 3.0),
    TnuaBlipReuseAvoidance::<PlayerControlScheme>::default(),
    PlayerControllerConfig::default(),
    TnuaSimpleAirActionsCounter::<PlayerControlScheme>::default(),
    TnuaController::<PlayerControlScheme>::default(),
    PlayerController::default(),
    PlayerControllerInput::default(),
    TnuaAvian3dSensorShape(Collider::capsule(0.1, 0.5)),
    AvianPickupActor {
        prop_filter: SpatialQueryFilter::from_mask(CollisionLayer::Prop),
        actor_filter: SpatialQueryFilter::from_mask(CollisionLayer::Player),
        obstacle_filter: SpatialQueryFilter::from_mask(CollisionLayer::Default),
        hold: AvianPickupActorHoldConfig {
            pitch_range: -40.0_f32.to_radians()..=75.0_f32.to_radians(),
            distance_to_allow_holding: 100.0,
            preferred_distance: 1.5,
            ..default()
        },
        ..default()
    },
    Sleep::default(),
)]
#[component(on_add = on_player_add)]
pub struct Player;

/// The main player camera
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    Name::new("PlayerCamera"),
    Camera {
        clear_color: ClearColorConfig::Custom(Srgba::rgb(0.0, 0.0, 0.0).into()),
        ..default()
    },
    Camera3d { ..default() },
    Projection::Perspective(PerspectiveProjection {
        fov: std::f32::consts::PI / 2.0,
        ..default()
    }),
    AtmosphereSettings {
        aerial_view_lut_max_distance: 3.2e5,
        scene_units_to_m: 1e+4,
        ..Default::default()
    },
    Hdr,
    Exposure::SUNLIGHT,
    Tonemapping::AcesFitted,
    Transform {
        translation: Vec3 { y: 2., ..default() },
        ..default()
    },
    Bloom::NATURAL,
)]
#[component(on_add = on_player_camera_add)]
pub struct PlayerCamera;

/// The player flashlight.
///
/// Emits light.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    SpotLight {
        intensity: 1_000_000.0,
        shadows_enabled: true,
        ..default()
    },
)]
pub struct PlayerFlashlight;

/// Player Spawner
///
/// Entities marked with a PlayerSpawner component designate the location
/// in game-space where the player spawns
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[type_path("api")]
pub struct PlayerSpawner;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[type_path("api")]
pub struct PlayerTrigger;

/// Marker Component for the currently active player weapon.
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
            .init_state::<CameraState>()
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
            .add_systems(Update, spawn_player_observer.run_if(resource_exists::<DAGunAssets>))
            //.add_systems(OnEnter(BootStrap::Postload), init_player)
            .add_systems(OnEnter(MetaState::Gameplay), init_player)
            .add_systems(Update, (
                    player_forward.run_if(/*in_state(GameState::Gameplay)*/ in_state(CameraState::Player)),
                    check_player_triggers.run_if(in_state(GameState::Gameplay)),
                )
            );
            //.add_systems(Update, toggle_player_noclip.run_if(input_just_pressed(KeyCode::KeyB)));
    }
}

fn on_player_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let player = world.commands()
        .entity(context.entity)
        .insert(
            CharacterBundle {
                mana: Mana(100),
                max_mana: MaxMana(100),
                health: Health(100),
                max_health: MaxHealth(100),
                experience: Experience(100),
                ..default()
            },
        )
        .observe(player_death_event_observer)
        .observe(add_to_inventory_observer::<Player>)
        .observe(remove_from_inventory_observer::<Player>)
        .observe(display_inventory_event_observer)
        .observe(display_quest_event_observer)
        .observe(display_equip_event_observer)
        .observe(display_stats_event_observer)
        .observe(eat_event_observer)
        .observe(drink_event_observer)
        .id();

    if let Some(mut render_player_query) = world.try_query::<&mut RenderPlayer>() {
        let mut query = world.query(&mut render_player_query);
        if let Ok(mut render_player) = query.single_mut() {
            println!("{:?}", player);
            println!("{:?}", render_player.logical_entity);
            render_player.logical_entity = player;
            println!("{:?}", render_player.logical_entity);
        }
    }
}

fn on_player_camera_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let mut scattering_mediums = world.resource_mut::<Assets<ScatteringMedium>>();
    let scattering_mediums_handle = scattering_mediums.add(ScatteringMedium::default());

    world.commands()
        .entity(context.entity)
        .insert(Atmosphere::earthlike(scattering_mediums_handle))
        .insert(PrimaryEguiContext);
}

fn init_player(
    mut spawn_player_message_writer: MessageWriter<SpawnPlayerMessage>,
    player_query: Query<Entity, With<Player>>,
) {
    trace!("SYSTEM: init_player");
    for _player in player_query.iter() {
        return;
    }
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
                ItemDetails {
                    name: "gun".to_string(),
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
                ),
            ))
            .id();

        commands.spawn((
            Quest {
                description: "TEST".to_string()
            },
            FetchQuest::new(1, "Water".to_string()),
            QuestOf(logical_entity),
            ));

        // Camera

        debug!("Creating Camera");
        if let Ok(mut render_player) = player_camera_query.single_mut() {
            *render_player = RenderPlayer { logical_entity };
        } else {
            let flashlight = commands
                .spawn((
                    PlayerFlashlight,
                ))
                .id();
            commands
                .spawn((
                    RenderPlayer { logical_entity },
                    PlayerCamera,
                ))
                //.add_child(gun)
                .add_child(flashlight);
        }
    }
}

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
