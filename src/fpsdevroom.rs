use avian3d::collision::Collider;
use bevy::asset::RenderAssetUsages;
use bevy::color::palettes::css::{RED};
use bevy::render::render_resource::{Extent3d, TextureFormat, TextureUsages};
use bevy::render::view::RenderLayers;
use bevy::sprite::Material2d;
use bevy::{
    core_pipeline::core_3d::Camera3d, math::vec3, prelude::*, render::camera::ClearColorConfig,
};
use bevy_atmosphere::plugin::AtmospherePlugin;
use bevy_tnua::control_helpers::TnuaSimpleAirActionsCounter;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;
use leafwing_input_manager::{InputManagerBundle, input_map::InputMap};

use std::f32::consts::PI;

use super::CameraConfig;
use super::RenderPlayer;
use super::controller::*;

use crate::*;

#[derive(Debug, PhysicsLayer, Default, Component, Reflect)]
#[reflect(Component)]
pub enum CollisionLayer {
    #[default]
    Default,
    Player,
    Prop,
}

pub struct TargetBundle {
    rigid_body: RigidBody,
    health: Health,
    transform: Transform,
    collider: Collider,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    respawn_timer: Timer,
}

pub struct FpsDevRoomPlugin;

impl Plugin for FpsDevRoomPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::Loading),
                (
                    spawn_basic_scene,
                    spawn_player,
                    spawn_walking_cube,
                    spawn_sphere,
                ).chain()
            )
            .register_type::<CollisionLayer>()
            .add_plugins(AtmospherePlugin)
            .add_systems(Update, player_forward.run_if(in_state(GameState::Gameplay)));
    }
}

fn spawn_basic_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window: Query<&mut Window>,
) {
    trace!("Spawn basic scene");

    if let Ok(mut window) = window.get_single_mut() {
        window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
    }

    info!("Creating DirectionalLightBundle");
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OFFICE,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
    ));

    info!("Loading DevRoom");
    commands.spawn(SceneRoot(
        asset_server.load("levels/devroom.glb#Scene0"),
    ));
    //commands.spawn(SceneBundle { scene: asset_server.load("levels/__temp_scene.glb#Scene0"), ..default() });
    info!("DevRoom Loaded");
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Gun
    info!("Creating Gun");
    let gun = commands
        .spawn((
            Transform::from_translation(vec3(0.1, -0.2, -0.5)),
            SceneRoot(asset_server.load("guns/uzi.glb#Scene0")),
            Item {
                name: Name::new("gun"),
                description: Description("gun".to_string()),
                item_type: ItemType::Weapon(Weapon::default()),
                weight: Weight(0),
            },
        ))
        .id();

    // Player
    info!("Creating Player");
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
                Transform::from_xyz(0.0, 5.0, 0.0),
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
                TnuaAvian3dSensorShape(Collider::capsule(0.1, 0.1)),
                FloatHeight(1.5),
            ),
            (CollisionLayers::new(CollisionLayer::Player, LayerMask::ALL),),
        ))
        .insert((Walk::default(), InputManagerBundle::with_map(input_map)))
        .insert(TnuaSimpleAirActionsCounter::default())
        .insert(AvianPickupActor {
            prop_filter: SpatialQueryFilter::from_mask(CollisionLayer::Prop),
            actor_filter: SpatialQueryFilter::from_mask(CollisionLayer::Player),
            obstacle_filter: SpatialQueryFilter::from_mask(CollisionLayer::Default),
            hold: AvianPickupActorHoldConfig {
                pitch_range: -40.0_f32.to_radians()..=75.0_f32.to_radians(),
                distance_to_allow_holding: 100.0.into(),
                ..default()
            },
            ..default()
        })
        .id();

    // Camera
    info!("Creating Camera");
    commands
        .spawn((
            Camera {
                hdr: true,
                clear_color: ClearColorConfig::Custom(Srgba::rgb(0.0, 0.0, 0.0).into()),
                ..default()
            },
            Camera3d { ..default() },
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

fn spawn_walking_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Cube
    info!("Creating Cube");
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_xyz(-0.9, 1.5, -3.2),
            RigidBody::Dynamic,
            Collider::cuboid(1.0, 1.0, 1.0),
            Item {
                name: Name::new("Cube"),
                description: Description("Cube".to_string()),
                item_type: ItemType::Misc,
                weight: Weight(0),
            },
        ))
        .insert(TnuaController::default())
        .insert(TnuaAvian3dSensorShape(Collider::cuboid(0.5, 0.5, 0.5)))
        .insert(FloatHeight(0.5))
        .insert(Walk::default())
        .insert(DesiredPosition(Vec3 {
            x: -15.0,
            y: 5.0,
            z: -15.0,
        }))
        .insert(Name::new("Cube"));
}

fn spawn_sphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sphere
    info!("Creating Sphere");
    commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(20).unwrap())),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_xyz(-0.9, 1.5, -4.2),
            RigidBody::Dynamic,
            Collider::sphere(0.5),
            CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL),
        ))
        .insert(Name::new("Sphere"))
        .insert(MiscItem);
}

fn player_forward(
    cam_transform: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
    mut player_transform: Query<&mut Transform, With<Player>>,
) {
    trace!("System: player_forward");
    let cam_transform = cam_transform.single();
    let forward = cam_transform.forward();
    let mut player_transform = player_transform.single_mut();
    player_transform.look_to(*forward, Vec3::Y);
}
