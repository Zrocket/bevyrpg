use avian3d::collision::Collider;
use bevy::{
    core_pipeline::core_3d::Camera3d, math::vec3, prelude::*, render::camera::ClearColorConfig,
};
use bevy_tnua::control_helpers::TnuaSimpleAirActionsCounter;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;
use leafwing_input_manager::{InputManagerBundle, input_map::InputMap};

use std::f32::consts::PI;

use super::CameraConfig;
use super::RenderPlayer;
use super::controller::*;

use crate::*;

pub struct DevRoomPlugin;

impl Plugin for DevRoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), spawn_basic_scene)
            //.add_systems(OnEnter(GameState::Gameplay), spawn_sprites)
            .add_systems(Update, player_forward.run_if(in_state(GameState::Gameplay)));
        //.add_plugins(SpritesPlugin);
    }
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
        asset_server.load("levels/__temp_scene.glb#Scene0"),
    ));
    //commands.spawn(SceneBundle { scene: asset_server.load("levels/__temp_scene.glb#Scene0"), ..default() });
    info!("DevRoom Loaded");

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
                interact: Interactable::None,
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
            Collider::capsule(0.1, 1.5),
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
        ))
        .insert((Walk::default(), InputManagerBundle::with_map(input_map)))
        .insert(TnuaSimpleAirActionsCounter::default())
        .insert(AvianPickupActor::default())
        .id();

    let rand_character: CharacterBundle = rand::random();
    // Cube
    info!("Creating Cube");
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_xyz(-0.9, 1.5, -3.2),
            RigidBody::Dynamic,
            Collider::cuboid(0.5, 0.5, 0.5),
            Item {
                name: Name::new("Cube"),
                description: Description("Cube".to_string()),
                item_type: ItemType::Misc,
                interact: Interactable::Misc,
                weight: Weight(0),
            },
        ))
        .insert(rand_character)
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

    // Sphere
    info!("Creating Sphere");
    commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(20).unwrap())),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_xyz(-0.9, 1.5, -4.2),
            Interactable::Misc,
            RigidBody::Dynamic,
            Collider::sphere(0.5),
        ))
        .insert(Name::new("Sphere"));

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
        ))
        .add_child(gun);
}

fn player_forward(
    cam_transform: Query<&Transform, (With<Camera>, Without<Player>)>,
    mut player_transform: Query<&mut Transform, With<Player>>,
) {
    trace!("System: player_forward");
    let cam_transform = cam_transform.single();
    let forward = cam_transform.forward();
    let mut player_transform = player_transform.single_mut();
    player_transform.look_to(*forward, Vec3::Y);
}

/*fn spawn_sprites(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
    mut sprite_event: EventWriter<SpriteEvent>,
) {
    info!("Spawn sprites System");
    sprite_event.send(SpriteEvent { sprite_type: SpriteType::Character, tile_x: 8, tile_y: 0, x: 4.5, y: -4.0, height:1, frames:2 });
    sprite_event.send(SpriteEvent { sprite_type: SpriteType::Character, tile_x: 4, tile_y: 0, x: 1.5, y: -7.0, height: 4, frames: 2});
    sprite_event.send(SpriteEvent { sprite_type: SpriteType::Character, tile_x: 6, tile_y: 0, x: 0.5, y: 2.0, height: 4, frames: 2 });
    sprite_event.send(SpriteEvent { sprite_type: SpriteType::Character, tile_x: 0, tile_y: 19, x: 3.5, y: 1.0, height: 1, frames: 1 });
    sprite_event.send(SpriteEvent { sprite_type: SpriteType::Character, tile_x: 1, tile_y: 19, x: 4.0, y: 6.0, height: 1, frames: 1 });
    sprite_event.send(SpriteEvent { sprite_type: SpriteType::Character, tile_x: 4, tile_y: 19, x: 0.0, y: 5.0, height: 1, frames: 1 });
    sprite_event.send(SpriteEvent { sprite_type: SpriteType::Character, tile_x: 5, tile_y: 19, x: -4.0, y: 5.4, height:1, frames: 1});
    sprite_event.send(SpriteEvent { sprite_type: SpriteType::Character, tile_x: 2, tile_y: 19, x: -0.5, y: -8.5, height:1, frames: 1 });
    sprite_event.send(SpriteEvent { sprite_type: SpriteType::Character, tile_x: 13, tile_y: 16, x: 4.2, y: -8., height: 2, frames: 1 });

    let atlas = TextureAtlas {
        layout: images.layout.clone(),
        index: 30 * 32 + 14,
    };

    commands.spawn((
        Sprite3d {
            image: images.tileset.clone(),
            pixels_per_metre: 16.,
            transform: Transform::from_xyz(2.0, 0.5, -5.5),
            emissive: Srgba::rgb(1.0, 0.5, 0.0).into(),
            unlit: true,
            ..default()
        }
        .bundle_with_atlas(&mut sprite_params, atlas),
        Animation {
            frames: vec![30 * 32 + 14, 30 * 32 + 15, 30 * 32 + 16],
            current: 0,
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        },
        FaceCamera {},
    ));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 300.0,
            color: Color::srgb(1.0, 231. / 255., 221. / 255.),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 1.8, -5.5),
        ..default()
    });

    let atlas = TextureAtlas {
        layout: images.layout.clone(),
        index: 22 * 30 + 22,
    };

    commands.spawn((
        Sprite3d {
            image: images.tileset.clone(),
            pixels_per_metre: 16.,
            transform: Transform::from_xyz(-5., 0.7, 6.5),
            emissive: LinearRgba::rgb(165. / 255., 1.0, 160. / 255.),
            unlit: true,
            ..default()
        }
        .bundle_with_atlas(&mut sprite_params, atlas),
        FaceCamera {},
    ));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 100.0,
            color: Srgba::rgb(91. / 255., 1.0, 92. / 255.).into(),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-5., 1.1, 6.5),
        ..default()
    });
}*/
