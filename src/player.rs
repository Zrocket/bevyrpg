use avian3d::prelude::{CoefficientCombine, Collider, CollisionLayers, Friction, GravityScale, LayerMask, LockedAxes, RigidBody, SpatialQueryFilter};
use avian_pickup::actor::{AvianPickupActor, AvianPickupActorHoldConfig};
use bevy::{prelude::*, render::RenderPlugin};
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_tnua::{control_helpers::TnuaSimpleAirActionsCounter, prelude::TnuaController};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use leafwing_input_manager::prelude::InputMap;

use crate::{level::DA_GunAssets, Action, CameraConfig, CharacterBundle, CollisionLayer, Description, Experience, FloatHeight, GameState, Health, Inventory, Item, Mana, MaxHealth, MaxMana, PlayerController, PlayerControllerInput, RayHit, RenderPlayer, Walk, Weight};

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, States)]
pub enum PlayerState {
    #[default]
    Grounded,
    Ladder,
    UnGrounded,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerCamera;

pub struct GamePlayerPlugin;
impl Plugin for GamePlayerPlugin {
    fn build(&self, app: &mut App) {
        info!("GamePlayerPlugin build");
        app.register_type::<Player>()
            .register_type::<PlayerCamera>()
            .add_systems(OnEnter(GameState::Loading), spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gun_assets: Res<DA_GunAssets>,
    gltf_assets: Res<Assets<Gltf>>,
) {
    trace!("SYSTEM: spawn_player");

    // Gun
    debug!("Creating Gun");
    //let temp = gun_assets.uzi.clone_weak();
    let uzi = gltf_assets.get(&gun_assets.uzi).unwrap().scenes[0].path().unwrap();
    //let temp = uzi.scenes[0].path().unwrap();
    let gun = commands
        .spawn((
            Transform::from_translation(vec3(0.1, -0.2, -0.5)),
           // SceneRoot(asset_server.load("guns/uzi.glb#Scene0")),
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
        .id();

    // Camera
    debug!("Creating Camera");
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
            AtmosphereCamera::default(),
        ))
        .add_child(gun);
}

