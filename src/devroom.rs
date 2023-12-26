use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, math::vec3};
use rand::Rng;

use crate::*;

#[derive(AssetCollection, Resource)]
struct ImageAssets {
    #[asset(texture_atlas(
        tile_size_x = 16.,
        tile_size_y = 16.,
        columns = 30,
        rows = 35,
        padding_x = 10.,
        padding_y = 10.,
        offset_x = 5.,
        offset_y = 5.
    ))]
    #[asset(path = "tileset_padded.png")]
    tileset: Handle<TextureAtlas>,
}

#[derive(Component)]
struct Animation {
    frames: Vec<usize>,
    current: usize,
    timer: Timer,
}

#[derive(Component)]
struct FaceCamera;

pub struct DevRoomPlugin;

impl Plugin for DevRoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), spawn_basic_scene)
            .add_collection_to_loading_state::<_, ImageAssets>(GameState::Loading)
            .add_systems(OnEnter(GameState::Gameplay), spawn_sprites)
            .add_systems(Update, face_camera.run_if(in_state(GameState::Gameplay)))
            .add_systems(Update, animate_sprites.run_if(in_state(GameState::Gameplay)))
            .add_systems(Update, player_forward.run_if(in_state(GameState::Gameplay)));
    }
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
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
        },
    ));

    // Gun
    let gun = commands
        .spawn(
            SceneBundle {
                scene: asset_server.load("guns/uzi.glb#Scene0"),
                //scene: asset_server.load("guns/shotgun.glb#Scene0"),
                //scene: asset_server.load("guns/revolver-python.glb#Scene0"),
                //scene: asset_server.load("guns/pistol-coonan.glb#Scene0"),
                //scene: asset_server.load("guns/smg-mp5x.glb#Scene0"),
                //scene: asset_server.load("guns/sniper.glb#Scene0"),
                transform: Transform::from_translation(vec3(0.1, -0.2, -0.5)),
                ..default()
            }
        )
        .insert(Weapon)
        .id();

    // Player
    let logical_entity = commands
        .spawn((
            Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.1),
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
            TransformBundle::from_transform(Transform::from_xyz(0.0, 1.0, 0.0)),
            LogicalPlayer,
            FpsControllerInput {
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController { ..default() },
        ))
        .insert(CameraConfig {
            height_offset: 0.0,
            radius_scale: 0.75,
        })
        .insert(Player)
        .insert(Character {
            mana: 100,
            max_mana: 100,
            health: 100,
            max_health: 100,
            experience: 100,
            ..default()
        })
        .insert(Inventory { ..default() })
        .id();

    // Cube
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_xyz(-0.9, 0.5, -3.2),
        ..default()
    },
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5, 0.5),
    ))
    .insert(
            Character {
                mana: 100,
                max_mana: 100,
                health: 100,
                max_health: 100,
                experience: 100,
                ..default()
            }
           );
    // Sphere
    commands.spawn(PbrBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: 0.6,
                subdivisions: 20,
            })
            .unwrap(),
        ),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_xyz(-0.9, 0.5, -4.2),
        ..default()
    });

    // Camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
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
        RenderPlayer{ logical_entity },
    ))
    .add_child(gun);
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
            let mut c = commands.spawn((
                AtlasSprite3d {
                    atlas: images.tileset.clone(),
                    pixels_per_metre: 16.,
                    index: (tile_x + (tile_y - i) * 30) as usize,
                    transform: Transform::from_xyz(x as f32, i as f32 + 0.499, y),
                    ..default()
                }
                .bundle(&mut sprite_params),
                FaceCamera {},
                Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.1),
                Character {
                    mana: 100,
                    max_mana: 100,
                    health: 100,
                    max_health: 100,
                    experience: 100,
                    ..default()
                }
            ));

            if frames > 1 {
                c.insert(Animation {
                    frames: (0..frames)
                        .map(|j| j + tile_x + (tile_y - i) * 30 as usize)
                        .collect(),
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

    commands.spawn((
        AtlasSprite3d {
            atlas: images.tileset.clone(),
            pixels_per_metre: 16.,
            index: 30 * 32 + 14,
            transform: Transform::from_xyz(2.0, 0.5, -5.5),
            emissive: Color::rgb(1.0, 0.5, 0.0),
            unlit: true,
            ..default()
        }
        .bundle(&mut sprite_params),
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
            color: Color::rgb(1.0, 231. / 255., 221. / 255.),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 1.8, -5.5),
        ..default()
    });

    commands.spawn((
        AtlasSprite3d {
            atlas: images.tileset.clone(),
            pixels_per_metre: 16.,
            index: 22 * 30 + 22,
            transform: Transform::from_xyz(-5., 0.7, 6.5),
            emissive: Color::rgb(165. / 255., 1.0, 160. / 255.),
            unlit: true,
            ..default()
        }
        .bundle(&mut sprite_params),
        FaceCamera {},
    ));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 100.0,
            color: Color::rgb(91. / 255., 1.0, 92. / 255.),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-5., 1.1, 6.5),
        ..default()
    });
}

fn player_forward(
    cam_transform: Query<&Transform, (With<Camera>, Without<Player>)>,
    mut player_transform: Query<&mut Transform, With<Player>>,
    ) {
    let cam_transform = cam_transform.single();
    let mut forward = cam_transform.forward();
    //forward.y = 0.0;
    let mut player_transform  = player_transform.single_mut();
    player_transform.look_to(forward, Vec3::Y);
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
