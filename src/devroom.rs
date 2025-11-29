use std::f32::consts::PI;
use avian3d::collision::collider::Collider;
use bevy::prelude::*;
use bevy_sprite3d::Sprite3d;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;

use crate::*;
use crate::items::*;
use crate::sprites::ImageAssets;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
struct FirstPassCube;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
struct MainPassCube;

pub struct DevRoomPlugin;
impl Plugin for DevRoomPlugin {
    fn build(&self, app: &mut App) {
        app
            /*.add_systems(
                OnEnter(GameState::Loading),
                (
                    spawn_walking_cube,
                ).chain()
            )*/
            .add_systems(OnEnter(GameState::Gameplay), spawn_sprites)
            .register_type::<FirstPassCube>()
            .register_type::<MainPassCube>();
            //.add_plugins(SpritesPlugin);
    }
}


fn _spawn_walking_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    trace!("SYSTEM: spawn_walking_cube");

    // Cube
    debug!("Creating Cube");
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

fn spawn_sprites(
    mut commands: Commands,
    images: Res<ImageAssets>,
    //mut sprite_params: Sprite3dParams,
    mut sprite_message: MessageWriter<SpriteMessage>,
) {
    info!("SYSTEM: spawn_sprites");
    sprite_message.write(SpriteMessage { sprite_type: SpriteType::Character, tile_x: 8, tile_y: 0, x: 4.5, y: -4.0, height:1, frames:2 });
    sprite_message.write(SpriteMessage { sprite_type: SpriteType::Character, tile_x: 4, tile_y: 0, x: 1.5, y: -7.0, height: 4, frames: 2});
    sprite_message.write(SpriteMessage { sprite_type: SpriteType::Character, tile_x: 6, tile_y: 0, x: 0.5, y: 2.0, height: 4, frames: 2 });
    sprite_message.write(SpriteMessage { sprite_type: SpriteType::Character, tile_x: 0, tile_y: 19, x: 3.5, y: 1.0, height: 1, frames: 1 });
    sprite_message.write(SpriteMessage { sprite_type: SpriteType::Character, tile_x: 1, tile_y: 19, x: 4.0, y: 6.0, height: 1, frames: 1 });
    sprite_message.write(SpriteMessage { sprite_type: SpriteType::Character, tile_x: 4, tile_y: 19, x: 0.0, y: 5.0, height: 1, frames: 1 });
    sprite_message.write(SpriteMessage { sprite_type: SpriteType::Character, tile_x: 5, tile_y: 19, x: -4.0, y: 5.4, height:1, frames: 1});
    sprite_message.write(SpriteMessage { sprite_type: SpriteType::Character, tile_x: 2, tile_y: 19, x: -0.5, y: -8.5, height:1, frames: 1 });
    sprite_message.write(SpriteMessage { sprite_type: SpriteType::Character, tile_x: 13, tile_y: 16, x: 4.2, y: -8., height: 2, frames: 1 });

    let atlas = TextureAtlas {
        layout: images.layout2.clone(),
        index: 30 * 32 + 14,
        //index: 18,
    };

    commands.spawn((
        Sprite {
            image: images.tileset.clone(),
            texture_atlas: Some(atlas),
            ..default()
        },
        Sprite3d {
            pixels_per_metre: 16.,
            emissive: Srgba::rgb(1.0, 0.5, 0.0).into(),
            unlit: true,
            ..default()
        },
        //.bundle_with_atlas(&mut sprite_params, atlas),
        Transform::from_xyz(2.0, 0.5, -5.5),
        Animation {
            frames: vec![30 * 32 + 14, 30 * 32 + 15, 30 * 32 + 16],
            current: 0,
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        },
        FaceCamera {},
    ));

    let atlas = TextureAtlas {
        layout: images.layout2.clone(),
        index: 22 * 30 + 22,
    };

    commands.spawn((
        /*Sprite3dBuilder {
            image: images.tileset.clone(),
            pixels_per_metre: 16.,
            emissive: LinearRgba::rgb(165. / 255., 1.0, 160. / 255.),
            unlit: true,
            ..default()
        }*/
        Sprite3d {
            pixels_per_metre: 16.,
            double_sided: false,
            emissive: LinearRgba::rgb(165. / 255., 1.0, 160. / 255.),
            unlit: true,
            ..default()
        },
        Sprite {
            image: images.tileset.clone(),
            texture_atlas: Some(atlas),
            ..default()
        },
        //.bundle_with_atlas(&mut sprite_params, atlas),
        Transform::from_xyz(-5., 0.7, 6.5),
        FaceCamera {},
    ));
}
