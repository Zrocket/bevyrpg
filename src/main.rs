use std::f32::consts::TAU;
use rand::{prelude::SliceRandom, Rng, distributions::Standard};

use bevy::{prelude::*, core_pipeline::{clear_color::ClearColorConfig, bloom::BloomSettings}, utils::Duration, window::CursorGrabMode, window::WindowResolution};
use bevy_fps_controller::controller::*;
use bevy_mod_picking::*;
use bevy_sprite3d::*;
use bevy_asset_loader::prelude::*;
use bevy_rapier3d::prelude::*;

mod player;
mod character;

pub use player::*;
pub use character::*;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

const CAM_DISTANCE: f32 = 21.0;
const CAM_HEIGHT: f32 = 7.0;
const CAM_SPEED: f32 = 0.1;

#[derive(Clone, Hash, Debug, Eq, PartialEq, States, Default)]
pub enum GameState {
    MainMenu,
    Gameplay,
    #[default]
    Loading
}

#[derive(AssetCollection, Resource)]
struct ImageAssets {
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16.,
                          columns = 30, rows = 35, padding_x = 10., padding_y = 10.,
                          offset_x = 5., offset_y = 5.))]
    #[asset(path = "tileset_padded.png")]
    tileset: Handle<TextureAtlas>,
}

#[derive(Component)]
struct FaceCamera;

#[derive(Component)]
struct Animation {
    frames: Vec<usize>,
    current: usize,
    timer: Timer,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::Gameplay)
        )
        .add_collection_to_loading_state::<_, ImageAssets>(GameState::Loading)
        .insert_resource(RapierConfiguration::default())
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.25,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                title: "Wizard RPG".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(Sprite3dPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(FpsControllerPlugin)
        .add_system(spawn_basic_scene.in_schedule(OnEnter(GameState::Loading)))
        .add_system(face_camera.in_set(OnUpdate(GameState::Gameplay)))
        .add_system(animate_sprites.in_set(OnUpdate(GameState::Gameplay)))
        .add_system(manage_cursor.in_set(OnUpdate(GameState::Gameplay)))
        .add_system(create_ui.in_schedule(OnEnter(GameState::Gameplay)))
        .add_system(spawn_sprites.in_schedule(OnEnter(GameState::Gameplay)))
        .run();
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-38.0, 40.0, 34.0),
        ..default()
    });


    // Ground
    commands.spawn((
        Collider::cuboid(20.0, 0.25, 20.0),
        RigidBody::Fixed,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: -20.0,
                max_x: 20.0,
                min_y: -0.25,
                max_y: 0.25,
                min_z: -20.0,
                max_z: 20.0,
            })),
            transform: Transform::from_xyz(0.0, -0.25, 0.0),
            ..default()
        }
        ));


    // Player
    commands.spawn((Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.1),
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        ActiveEvents::COLLISION_EVENTS,
        Velocity::zero(),
        RigidBody::Dynamic,
        Sleeping::disabled(),
        LockedAxes::ROTATION_LOCKED,
        AdditionalMassProperties::Mass(1.0),
        GravityScale(0.0),
        Ccd { enabled: true },
        TransformBundle::from_transform(Transform::from_xyz(0.0, 3.0, 0.0)),
        LogicalPlayer(0),
        FpsControllerInput {
            pitch: -TAU / 12.0,
            yaw: TAU * 5.0 / 8.0,
            ..default()
        },
        FpsController { ..default() }
    ));

    // Cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_xyz(-0.9, 0.5, -3.2),
        ..default()
    });

    // Sphere
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::try_from(shape::Icosphere { radius: 0.6, subdivisions: 20 }).unwrap()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_xyz(-0.9, 0.5, -4.2),
        ..default()
    });

    // Camera
    commands.spawn((Camera3dBundle {
        camera: Camera { hdr: true, ..default() },
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 0.0)),
            ..default()
        },
        projection: Projection::Perspective(PerspectiveProjection {
            fov: std::f32::consts::PI / 6.0,
            ..default()
        }),
        ..default()
    },
    RenderPlayer(0)));
}

fn spawn_sprites(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
) {

    let mut rng = rand::thread_rng();

    let mut entity = |(x, y), tile_x, tile_y, height, frames| {
        let mut timer = Timer::from_seconds(0.4, TimerMode::Repeating);
        timer.set_elapsed(Duration::from_secs_f32(rng.gen_range(0.0..0.4)));

        for i in 0usize..height {
            let mut c = commands.spawn((AtlasSprite3d {
                atlas: images.tileset.clone(),
                pixels_per_metre: 16.,
                index: (tile_x + (tile_y - i) * 30) as usize,
                transform: Transform::from_xyz(x as f32, i as f32 + 0.499, y),
                ..default()
            }.bundle(&mut sprite_params),
            FaceCamera {},
            ));

            if frames > 1 {
                c.insert(Animation {
                    frames: (0..frames).map(|j| j + tile_x + (tile_y - i) * 30 as usize).collect(),
                    current: 0,
                    timer: timer.clone(),
                });
            }
        }
    };

    entity((4.5, -4.0), 8, 27, 2, 2);
    entity((1.5, -7.0), 4, 27, 2, 2);
    entity((0.5, 2.0), 6, 27, 2, 2);

    entity((3.5, 1.0), 0, 19, 1, 1);
    entity((4.0, 6.0), 1, 19, 1, 1);
    entity((0.0, 5.0), 4, 19, 1, 1);
    entity((-4.0, 5.4), 5, 19, 1, 1);
    entity((-0.5, -8.5), 2, 19, 1, 1);

    entity((4.2, -8.), 13, 16, 2, 1);

    commands.spawn((AtlasSprite3d {
            atlas: images.tileset.clone(),
            pixels_per_metre: 16.,
            index: 30*32 + 14,
            transform: Transform::from_xyz(2.0, 0.5, -5.5),
            emissive: Color::rgb(1.0, 0.5, 0.0),
            unlit: true,
            ..default()
        }.bundle(&mut sprite_params),
        Animation {
            frames: vec![30*32 + 14, 30*32 + 15, 30*32 + 16],
            current: 0,
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        },
        FaceCamera {}
    ));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 300.0,
            color: Color::rgb(1.0, 231./255., 221./255.),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 1.8, -5.5),
        ..default()
    });

    commands.spawn((AtlasSprite3d {
        atlas: images.tileset.clone(),
        pixels_per_metre: 16.,
        index: 22*30 + 22,
        transform: Transform::from_xyz(-5., 0.7, 6.5),
        emissive: Color::rgb(165./255., 1.0, 160./255.),
        unlit: true,
        ..default()
    }.bundle(&mut sprite_params),
    FaceCamera {}
    ));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 100.0,
            color: Color::rgb(91./255., 1.0, 92./255.),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-5., 1.1, 6.5),
        ..default()
    });
}

fn create_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Auto),
            flex_direction: FlexDirection::ColumnReverse,
            ..default()
        },
        ..default()
    })
    .with_children(|commands| {
        commands.spawn(NodeBundle {
        background_color: BackgroundColor::from(Color::GREEN),
        visibility: Visibility::Visible,
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Auto),
            ..default()
        },
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(NodeBundle {
                visibility: Visibility::Visible,
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            })
            .insert(TextBundle {
                text: Text::from_section(
                      "Player Health",
                      TextStyle { 
                          font: asset_server.load("FiraSans-Bold.ttf"),
                          font_size: 50.0,
                          color: Color::BLACK 
                      }
                ),
                style: Style {
                    ..default()
                },
                ..default()
            });
            commands.spawn(NodeBundle {
                background_color: BackgroundColor::from(Color::RED),
                visibility: Visibility::Visible,
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Auto),
                    ..default()
                },
                ..default()
            });
        });
    });
}

fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut AtlasSprite3dComponent)>,
) {
    for (mut animation, mut sprite) in query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            sprite.index = animation.frames[animation.current];
            animation.current += 1;
            animation.current %= animation.frames.len();
        }
    }
}

fn face_camera(
    cam_query: Query<&Transform, With<Camera>>,
    mut query: Query<&mut Transform, (With<FaceCamera>, Without<Camera>)>,
) {
    let cam_transform = cam_query.single();
    for mut transform in query.iter_mut() {
        let mut delta = cam_transform.translation - transform.translation;
        delta.y = 0.0;
        delta += transform.translation;
        transform.look_at(delta, Vec3::Y);
    }
}

fn manage_cursor(
    mut windows: Query<&mut Window>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut controllers: Query<&mut FpsController>,
) {
    let mut window = windows.get_single_mut().unwrap();
    if btn.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        //window.set_cursor_grab_mode(CursorGrabMode::Locked);
        window.cursor.visible = false;
        //window.set_cursor_visibility(false);
        for mut controller in &mut controllers {
            controller.enable_input = true;
        }
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        //window.set_cursor_grab_mode(CursorGrabMode::None);
        window.cursor.visible = true;
        //window.set_cursor_visibility(true);
        for mut controller in &mut controllers {
            controller.enable_input = false;
        }
    }
}
