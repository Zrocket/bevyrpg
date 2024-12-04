use avian3d::prelude::*;
//use avian_interpolation3d::prelude::*;
//use avian_pickup::prelude::*;
use bevy::{
    log::LogPlugin,
    prelude::*,
    utils::Duration,
    window::{ WindowResolution, Cursor}
};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use bevy_registry_export::*;
use bevy_sprite3d::*;
use bevy_yoleck::prelude::*;
use blenvy::*;
use clap::Parser;

mod chair;
mod character;
mod computer;
mod console;
mod controller;
mod devroom;
mod dialog;
mod enemy;
mod hunger;
mod interact;
mod inventory;
mod items;
mod level;
mod magic;
mod player;
mod render;
mod rover;
mod shoot;
mod sprites;
mod stealth;
mod trade;
mod ui;
mod utils;

pub use chair::*;
pub use character::*;
pub use computer::*;
pub use console::*;
pub use controller::*;
pub use devroom::*;
pub use dialog::*;
pub use interact::*;
pub use inventory::*;
pub use items::*;
pub use player::*;
pub use render::*;
pub use rover::*;
pub use shoot::*;
pub use sprites::*;
pub use ui::*;
pub use utils::*;
use level::*;
use trade::TradePlugin;


pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    editor: bool,
    #[clap(long)]
    level: Option<String>,
}

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, States)]
pub enum GameState {
    MainMenu,
    UiMenu,
    Inventory,
    Settings,
    VideoSettings,
    SoundSettings,
    ControllerSettings,
    GameplaySettings,
    Gameplay,
    #[default]
    Loading,
}

fn main() {
    let args = Args::parse();

    let mut app = App::new();
    app
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        cursor: Cursor {
                            icon: CursorIcon::Crosshair,
                            //grab_mode: bevy::window::CursorGrabMode::Locked,
                            ..default()
                        },
                        resolution: WindowResolution::new(WIDTH, HEIGHT),
                        title: "Wizard RPG".to_string(),
                        resizable: false,
                        focused: true,
                        ..default()
                        }
                    ),
                    ..default()
                }
            )
            .set(LogPlugin {
                level: bevy::log::Level::INFO,
                ..default()
            })
        )
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
        .add_plugins((
            Sprite3dPlugin,
            PhysicsPlugins::default(),
            //AvianPickupPlugin::default(),
            //AvianInterpolationPlugin::default(),
            GamePlayerPlugin,
            CharacterPlugin,
            DevRoomPlugin,
            UiPlugin,
            ShootPlugin,
            ControllerPlugin,
            InventoryPlugin,
            InteractPlugin,
            DialogPlugin,
            TradePlugin,
            BlenderTranslationPlugin,
            GameRenderPlugin,
            ))
        .add_plugins(BlenvyPlugin::default());
        if args.editor {
            app.add_plugins(YoleckPluginForEditor);
            app.add_plugins(WorldInspectorPlugin::new());
        } else {
            app.add_plugins(YoleckPluginForGame);
        }
        app.add_systems(Update, health_test.run_if(in_state(GameState::Gameplay)))
        .add_systems(Update, inventory_test.run_if(in_state(GameState::Gameplay)))
        .add_systems(Update, inventory_remove_test.run_if(in_state(GameState::Gameplay)))
        .register_type::<RigidBody>()
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Gameplay).on_failure_continue_to_state(GameState::Gameplay)
            .load_collection::<ImageAssets>()
        )
        .run();
}

fn health_test(
    key: Res<ButtonInput<KeyCode>>,
    mut player: Query<(Entity, &Health), With<Player>>,
    mut damage_event_writer: EventWriter<DamageEvent>,
    ) {
    //trace!("Health test");
    let (player_entity, _player) = player.get_single_mut().unwrap();
    if key.just_pressed(KeyCode::KeyK) {
        damage_event_writer.send(DamageEvent {
            target: player_entity,
            ammount: 5,
        });
    }
}

fn inventory_test(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    mut player: Query<Entity, With<Player>>,
    mut event_writer: EventWriter<PickUpEvent>,
    ) {
    let player = player.get_single_mut().unwrap();
    if key.just_pressed(KeyCode::KeyJ) {
        let item = commands.spawn((
            Item {
                item_type: ItemType::None,
                name: Name::new(format!("Test {}", rand::random::<u8>() as char)),
                description: Description("Test".to_string()),
                weight: Weight(0),
                interact: Interactable::Misc,
            },
        )).id();
        event_writer.send(PickUpEvent { actor: player, target: item});
    }
}

fn inventory_remove_test(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    mut player: Query<Entity, With<Player>>,
    mut inventory_query: Query<&Inventory, With<Player>>,
    mut event_writer: EventWriter<RemoveEvent>,
) {
    let player = player.get_single_mut().unwrap();
    if key.just_pressed(KeyCode::KeyL) {
        let mut inventory = inventory_query.get_single_mut().unwrap();
        let item = inventory.items.last().unwrap();
        event_writer.send(RemoveEvent { actor: player, target: *item });
    }
}
