use std::time::Duration;

use bevy::prelude::*;
use bevy_sprite3d::*;

use bevy_trait_query::RegisterExt;
use rand::Rng;

use crate::*;

#[derive(Bundle)]
pub struct SpriteBundle {
    face_camera: FaceCamera,
    sprite_type: SpriteType,
    animation: Animation,
}

#[derive(Component)]
pub struct Talkable;
impl interact::Interaction for Talkable {
    fn interact(&self,commands: &mut Commands, _entity:Entity, prop:Entity,) {
        println!("Talkable Interaction Impl");
        commands.trigger_targets(DialogEvent {actor:prop}, prop);
    }
}
impl Inspectable for Talkable {
    fn inspect(
        &self,
        commands: &mut Commands,
        actor: Entity,
        prop: Entity,
    ) {
        println!("Talkable Inspectable Impl");
    }
}

#[derive(Component, Clone, Hash, Debug, Eq, PartialEq, Default)]
pub enum SpriteType {
    #[default]
    Character,
    Item,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(texture_atlas(
        tile_size_x = 16,
        tile_size_y = 32,
        columns = 20,
        rows = 1,
        padding_x = 0,
        padding_y = 0,
        offset_x = 0,
        offset_y = 0
    ))]
    pub character_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "character_tileset.png")]
    pub character_tileset: Handle<Image>,
    #[asset(texture_atlas(
        tile_size_x = 16,
        tile_size_y = 16,
        columns = 30,
        rows = 35,
        padding_x = 10,
        padding_y = 10,
        offset_x = 5,
        offset_y = 5
    ))]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(path = "tileset_padded.png")]
    pub tileset: Handle<Image>,
}

#[derive(Event)]
pub struct SpriteEvent {
    pub sprite_type: SpriteType,
    pub tile_x: usize,
    pub tile_y: usize,
    pub x: f32,
    pub y: f32,
    pub height: usize,
    pub frames: usize,
}

#[derive(Component)]
pub struct Animation {
    pub frames: Vec<usize>,
    pub current: usize,
    pub timer: Timer,
}

#[derive(Component)]
pub struct FaceCamera;

pub struct SpritesPlugin;
impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        trace!("SpritesPlugin build");
        app.add_loading_state(
            LoadingState::new(GameState::Preload)
                .load_collection::<ImageAssets>(),
        )
        .register_component_as::<dyn interact::Interaction, Talkable>()
        .register_component_as::<dyn interact::Inspectable, Talkable>();
        app.add_event::<SpriteEvent>()
            .add_systems(Update, sprite_handler.run_if(in_state(GameState::Gameplay)))
            .add_systems(Update, face_camera.run_if(in_state(GameState::Gameplay)))
            .add_systems(
                Update,
                animate_sprites.run_if(in_state(GameState::Gameplay)),
            );
    }
}

fn sprite_handler(
    mut commands: Commands,
    mut sprite_events: EventReader<SpriteEvent>,
    images: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
) {
    trace!("Event Handler: sprite_handler");
    let mut rng = rand::rng();

    for event in sprite_events.read() {
        info!("event {} {}", event.tile_x, event.tile_y);
        info!("Sprite Event read");
        let mut atlas = TextureAtlas {
            index: event.tile_x,
            ..default()
        };

        let mut timer = Timer::from_seconds(0.4, TimerMode::Repeating);
        info!("Timer declared");
        timer.set_elapsed(Duration::from_secs_f32(rng.random_range(0.0..0.4)));
        atlas.layout = images.character_layout.clone();
        info!("atlas layout decalred");

        match event.sprite_type {
            SpriteType::Character => {
                let atlas = TextureAtlas {
                    layout: images.character_layout.clone(),
                    index: event.tile_x,
                };

                info!("Character Sprite");
                let mut c = commands.spawn((
                    Sprite3dBuilder {
                        image: images.character_tileset.clone(),
                        pixels_per_metre: 16.,
                        double_sided: false,
                        ..default()
                    }
                    .bundle_with_atlas(&mut sprite_params, atlas),
                    FaceCamera {},
                    CharacterBundle::default(),
                    Collider::cuboid(0.5, 1., 0.5),
                    YarnNode::default(),
                    //KinematicCharacterController::default(),
                    RigidBody::Kinematic,
                    Transform::from_xyz(event.x, 1., event.y),
                    Talkable,
                ));
                info!("Character Spawned");
                info!("Character frames: {}", event.frames);
                if event.frames > 1 {
                    //info!("Character Frame");
                    c.insert(Animation {
                        frames: (0..event.frames)
                            .map(|j| j + event.tile_x + event.tile_y * 30_usize)
                            .collect(),
                        current: 0,
                        timer: timer.clone(),
                    });
                }
            }
            SpriteType::Item => {
                let atlas = TextureAtlas {
                    layout: images.layout.clone(),
                    index: event.tile_x,
                };

                info!("Item Sprite");
                let mut c = commands.spawn((
                    Sprite3dBuilder {
                        image: images.tileset.clone(),
                        pixels_per_metre: 16.,
                        double_sided: false,
                        ..default()
                    }
                    .bundle_with_atlas(&mut sprite_params, atlas),
                    FaceCamera {},
                ));
                if event.frames > 1 {
                    c.insert(Animation {
                        frames: (0..event.frames)
                            .map(|j| j + event.tile_x + event.tile_y * 30_usize)
                            .collect(),
                        current: 0,
                        timer: timer.clone(),
                    });
                }
            }
        }
    }
}

fn animate_sprites(time: Res<Time>, mut query: Query<(&mut Animation, &mut Sprite3d)>) {
    trace!("System: animate_sprites");
    for (mut animation, mut sprite) in query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            let atlas = sprite.texture_atlas.as_mut().unwrap();
            atlas.index = animation.frames[animation.current];
            animation.current += 1;
            animation.current %= animation.frames.len();
        }
    }
}

fn face_camera(
    cam_query: Query<&Transform, With<Camera>>,
    mut query: Query<&mut Transform, (With<FaceCamera>, Without<Camera>)>,
) {
    trace!("System: face_camera");
    if let Ok(cam_transform) = cam_query.single() {
        for mut transform in query.iter_mut() {
            let mut delta = cam_transform.translation - transform.translation;
            delta.y = 0.0;
            delta += transform.translation;
            transform.look_at(delta, Vec3::Y);
        }
    }
}
