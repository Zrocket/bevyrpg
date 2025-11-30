use avian3d::{prelude::{CoefficientCombine, Collider, CollisionLayers, Friction, GravityScale, LayerMask, LockedAxes, RigidBody, SpatialQuery, SpatialQueryFilter}};
use avian_pickup::actor::{AvianPickupActor, AvianPickupActorHoldConfig};
use bevy::{camera::Exposure, core_pipeline::tonemapping::Tonemapping, pbr::{Atmosphere, AtmosphereSettings}, post_process::bloom::Bloom, prelude::*, render::view::Hdr};
use bevy_egui::PrimaryEguiContext;
use bevy_tnua::{control_helpers::{TnuaBlipReuseAvoidance, TnuaSimpleAirActionsCounter}, prelude::TnuaController, TnuaObstacleRadar};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use leafwing_input_manager::prelude::{ActionState, InputMap};

use crate::{Action, CameraConfig, CharacterBundle, CollisionLayer, DeathEvent, Description, Experience, FloatHeight, GameState, Health, Inventory, Item, Mana, MaxHealth, MaxMana, PlayerController, PlayerControllerConfig, PlayerControllerInput, RayHit, RenderPlayer, Walk, Weight, level::DAGunAssets};

#[derive(Clone, Component, Hash, Debug, Eq, PartialEq, Default, States)]
pub enum PlayerState {
    #[default]
    Grounded,
    Ladder(Entity),
    UnGrounded,
    Sitting,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
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

#[derive(Message)]
pub struct SpawnPlayerMessage;

pub struct GamePlayerPlugin;
impl Plugin for GamePlayerPlugin {
    fn build(&self, app: &mut App) {
        info!("GamePlayerPlugin build");
        app.register_type::<Player>()
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
            );
    }
}

fn init_player(
    mut spawn_player_message_writer: MessageWriter<SpawnPlayerMessage>,
) {
    spawn_player_message_writer.write(SpawnPlayerMessage);
}

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

    for message in spawn_player_message_reader.read() {
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
                    name: Name::new("gun"),
                    description: Description("gun".to_string()),
                    weight: Weight(0),
                },
            ))
            .id();

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

        // Player
        debug!("Creating Player");
        let input_map = InputMap::new([
            (Action::Jump, KeyCode::Space),
            (Action::Run, KeyCode::ShiftLeft),
            (Action::Left, KeyCode::KeyA),
            (Action::Right, KeyCode::KeyD),
            (Action::Forward, KeyCode::KeyW),
            (Action::Backward, KeyCode::KeyS),
            (Action::Crouch, KeyCode::ControlLeft),
            (Action::Up, KeyCode::KeyZ),
            (Action::Down, KeyCode::KeyX),
            (Action::Interact, KeyCode::KeyO),
            (Action::OpenInventory, KeyCode::KeyI),
            (Action::OpenConsole, KeyCode::Backslash),
            (Action::Flashlight, KeyCode::KeyF),
        ]);

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
                    Inventory { ..default() },
                    TnuaController::default(),
                    TnuaAvian3dSensorShape(Collider::capsule(0.1, 0.5)),
                    FloatHeight(1.5),
                ),
                (CollisionLayers::new(CollisionLayer::Player, LayerMask::ALL),),
            ))
            .insert((Walk::default(), input_map))
            .insert(TnuaSimpleAirActionsCounter::default())
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
            .insert(TnuaBlipReuseAvoidance::default())
            .insert(PlayerControllerConfig::default())
            .observe(player_death_event_observer)
            .id();

        // Camera


        debug!("Creating Camera");
        if let Ok(mut render_player) = player_camera_query.single_mut() {
            *render_player = RenderPlayer { logical_entity };
        } else {
            commands
                .spawn((
                    Camera {
                        clear_color: ClearColorConfig::Custom(Srgba::rgb(0.0, 0.0, 0.0).into()),
                        ..default()
                    },
                    Hdr,
                    Camera3d { ..default() },
                    PrimaryEguiContext,
                    Atmosphere::EARTH,
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
    trigger: On<DeathEvent>,
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_state.set(GameState::GameOver);
}
