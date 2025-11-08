use avian3d::{prelude::{CoefficientCombine, Collider, CollisionLayers, Friction, GravityScale, LayerMask, LockedAxes, RigidBody, SpatialQuery, SpatialQueryFilter}};
use avian_pickup::actor::{AvianPickupActor, AvianPickupActorHoldConfig};
use bevy::{camera::Exposure, pbr::{Atmosphere, AtmosphereSettings}, post_process::bloom::Bloom, prelude::*, render::view::Hdr};
use bevy_tnua::{control_helpers::{TnuaBlipReuseAvoidance, TnuaSimpleAirActionsCounter}, prelude::TnuaController, TnuaObstacleRadar};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use leafwing_input_manager::prelude::InputMap;

use crate::{Action, CameraConfig, CharacterBundle, CollisionLayer, Description, Experience, FloatHeight, GameState, Health, Inventory, Item, Mana, MaxHealth, MaxMana, PlayerController, PlayerControllerConfig, PlayerControllerInput, RayHit, RenderPlayer, Walk, Weight, level::DAGunAssets};

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
pub struct PlayerSpawner;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerTrigger;

pub struct GamePlayerPlugin;
impl Plugin for GamePlayerPlugin {
    fn build(&self, app: &mut App) {
        info!("GamePlayerPlugin build");
        app.register_type::<Player>()
            .register_type::<PlayerCamera>()
            .register_type::<PlayerSpawner>()
            .register_type::<PlayerTrigger>()
            .add_systems(OnEnter(GameState::Postload), spawn_player)
            .add_systems(Update, (
                    player_forward.run_if(in_state(GameState::Gameplay)),
                    check_player_triggers.run_if(in_state(GameState::Gameplay)),
                )
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gun_assets: Res<DAGunAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    player_spawner_query: Query<&GlobalTransform, With<PlayerSpawner>>,
) {
    trace!("SYSTEM: spawn_player");

    let mut spawn_point = Transform::from_xyz(0.0, 5.0, 0.0);

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
        (Action::Up, KeyCode::KeyQ),
        (Action::Down, KeyCode::KeyE),
        (Action::Interact, KeyCode::KeyF),
        (Action::OpenInventory, KeyCode::KeyI),
        (Action::OpenConsole, KeyCode::Backslash),
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
        //.insert((Walk::default(), InputManagerBundle::with_map(input_map)))
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
        .insert(RayHit(Entity::PLACEHOLDER))
        .insert(Name::new("Player"))
        .insert(PlayerState::Grounded)
        .insert(TnuaObstacleRadar::new(1.0, 3.0))
        .insert(TnuaBlipReuseAvoidance::default())
        .insert(PlayerControllerConfig::default())
        .id();

    // Camera
    debug!("Creating Camera");
    commands
        .spawn((
            Camera {
                clear_color: ClearColorConfig::Custom(Srgba::rgb(0.0, 0.0, 0.0).into()),
                ..default()
            },
            Hdr,
            Camera3d { ..default() },
            Atmosphere::EARTH,
            AtmosphereSettings {
                aerial_view_lut_max_distance: 3.2e5,
                scene_units_to_m: 1e+4,
                ..Default::default()
            },
            Exposure::SUNLIGHT,
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
        .add_child(gun);
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
