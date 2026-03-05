use std::time::Duration;

use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};
use bevy_sprite3d::*;

use rand::Rng;
use rand::random_range;

use crate::*;

#[derive(Bundle)]
pub struct SpriteBundle {
    face_camera: FaceCamera,
    sprite_type: SpriteType,
    animation: Animation,
}

#[derive(Component, Default)]
pub struct Talkable;

fn sprite_interaction_observer(
    trigger: On<InteractionEvent>,
    mut commands: Commands,
) {
    commands.trigger(DialogMessage { actor: trigger.entity });
}

#[derive(Component)]
#[require(
    FaceCamera,
    YarnNode::default(),
    Talkable,
    Sprite3d {
        pixels_per_metre: 16.,
        double_sided: false,
        ..default()
    },
    Collider::cuboid(0.5, 1., 0.5),
)]
#[component(on_add = on_character_sprite_add)]
pub struct CharacterSprite {
    pub tile_x: usize,
    pub tile_y: usize,
    pub x: f32,
    pub y: f32,
    pub height: usize,
    pub frames: usize,
}

fn on_character_sprite_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let images = world.resource::<ImageAssets>();
    if let Some(character_sprite) = world.get::<CharacterSprite>(context.entity) {

        let frames = character_sprite.frames;
        let tile_x = character_sprite.tile_x;
        let tile_y = character_sprite.tile_y;
        let x = character_sprite.x;
        let y = character_sprite.y;

        let atlas = TextureAtlas {
            index: character_sprite.tile_x,
            layout: images.layout.clone(),
        };

        let image = images.image.clone();

        if character_sprite.frames > 1 {
            let mut rng = rand::rng();
            let mut timer = Timer::from_seconds(0.4, TimerMode::Repeating);
            timer.set_elapsed(Duration::from_secs_f32(random_range(0.0..0.4)));


            world.commands()
                .entity(context.entity)
                .insert(Animation {
                    frames: (0..frames)
                        .map(|j| j + tile_x + tile_y * 30_usize)
                        .collect(),
                    current: 0,
                    timer: timer.clone(),
                });
        }

        world.commands()
            .entity(context.entity)
            .insert(CharacterBundle::default())
            .insert(Sprite {
                image,
                texture_atlas: Some(atlas),
                ..default()
            })
            .insert(
                Transform::from_xyz(x, 1., y),
            )
        .observe(sprite_interaction_observer);
    }

}

#[derive(Component, Clone, Hash, Debug, Eq, PartialEq, Default)]
pub enum SpriteType {
    #[default]
    Character,
    Item,
}

#[derive(Component)]
#[require(
    Sprite3d {
        pixels_per_metre: 16.,
        double_sided: false,
        ..default()
    },
    FaceCamera,
)]
#[component(on_add = on_item_sprite_add)]
pub struct ItemSprite {
    pub tile_x: usize,
    pub tile_y: usize,
    pub x: f32,
    pub y: f32,
    pub height: usize,
    pub frames: usize,
}

fn on_item_sprite_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let images = world.resource::<ImageAssets>();
    if let Some(item_sprite) = world.get::<ItemSprite>(context.entity) {

        let atlas = TextureAtlas {
            index: item_sprite.tile_x,
            layout: images.layout.clone(),
        };

        let image = images.image.clone();

        if item_sprite.frames > 1 {
            let mut rng = rand::rng();
            let mut timer = Timer::from_seconds(0.4, TimerMode::Repeating);
            timer.set_elapsed(Duration::from_secs_f32(random_range(0.0..0.4)));

            let frames = item_sprite.frames;
            let tile_x = item_sprite.tile_x;
            let tile_y = item_sprite.tile_y;

            world.commands()
                .entity(context.entity)
                .insert(Animation {
                    frames: (0..frames)
                        .map(|j| j + tile_x + tile_y * 30_usize)
                        .collect(),
                    current: 0,
                    timer: timer.clone(),
                });
        }

        world.commands()
            .entity(context.entity)
            .insert(Sprite {
                image,
                texture_atlas: Some(atlas),
                ..default()
            });
    }
}

#[derive(AssetCollection, Resource, Default)]
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
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(path = "character_tileset.png")]
    pub image: Handle<Image>,
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
    pub layout2: Handle<TextureAtlasLayout>,
    #[asset(path = "tileset_padded.png")]
    pub tileset: Handle<Image>,
}

#[derive(Message)]
pub struct SpriteMessage {
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

#[derive(Component, Default)]
pub struct FaceCamera;

pub struct SpritesPlugin;
impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        trace!("SpritesPlugin build");
        app
            .insert_resource(ImageAssets::default())
            .add_loading_state(
                LoadingState::new(BootStrap::Preload)
                    .load_collection::<ImageAssets>(),
            )
            .add_message::<SpriteMessage>()
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
    mut sprite_events: MessageReader<SpriteMessage>,
) {
    trace!("Event Handler: sprite_handler");
    let mut rng = rand::rng();

    for event in sprite_events.read() {
        info!("event {} {}", event.tile_x, event.tile_y);
        info!("Sprite Event read");

        let mut timer = Timer::from_seconds(0.4, TimerMode::Repeating);
        info!("Timer declared");
        timer.set_elapsed(Duration::from_secs_f32(random_range(0.0..0.4)));
        info!("atlas layout decalred");

        match event.sprite_type {
            SpriteType::Character => {
                info!("Character Sprite");
                commands.spawn((
                        CharacterSprite {
                            tile_x: event.tile_x,
                            tile_y: event.tile_y,
                            x: event.x,
                            y: event.y,
                            height: event.height,
                            frames: event.frames,
                        },
                    //Transform::from_xyz(event.x, 1., event.y),
                ));
                info!("Character Spawned");
                info!("Character frames: {}", event.frames);
            }
            SpriteType::Item => {
                info!("Item Sprite");
                commands.spawn((
                    ItemSprite {
                        tile_x: event.tile_x,
                        tile_y: event.tile_y,
                        x: event.x,
                        y: event.y,
                        height: event.height,
                        frames: event.frames,
                    },
                ));
            }
        }
    }
}

fn animate_sprites(time: Res<Time>, mut query: Query<(&mut Animation, &mut Sprite)>) {
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
