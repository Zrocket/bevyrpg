use avian_pickup::prelude::*;
use avian3d::prelude::*;
use bevy::{
    asset::RenderAssetUsages, log::LogPlugin, prelude::*, render::render_resource::{Extent3d, TextureFormat, TextureUsages}, window::{
        //Cursor,
        CursorGrabMode,
        CursorOptions,
        WindowResolution,
    }
};
use bevy_asset_loader::prelude::*;
use bevy_console::ConsoleOpen;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_sprite3d::Sprite3dPlugin;
use bevy_yoleck::prelude::*;
use blenvy::BlenvyPlugin;
use clap::Parser;

mod chair;
mod character;
mod computer;
mod console;
mod controller;
mod devroom;
mod fpsdevroom;
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
mod tests;
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
use level::*;
pub use player::*;
pub use render::*;
pub use rover::*;
pub use shoot::*;
pub use sprites::*;
use tests::TestsPlugin;
use trade::*;
use trade::TradePlugin;
pub use ui::*;
pub use utils::*;

pub const RESOLUTION_HEIGHT: f32 = 720.0;
pub const RESOLUTION_WIDTH: f32 = 1280.0;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    editor: bool,
    #[clap(long)]
    level: Option<String>,
}

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, SubStates)]
#[source(GameState = GameState::Paused)]
pub enum PauseMenuState {
    #[default]
    MainMenu,
    Settings,
    ControllerSettings,
    GameplaySettings,
    VideoSettings,
    SoundSettings,
}

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, States)]
pub enum GameState {
    MainMenu,
    Inventory,
    Console,
    Gameplay,
    Paused,
    #[default]
    Loading,
}

fn main() {
    trace!("MAIN");
    let args = Args::parse();

    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins .set(WindowPlugin {
                primary_window: Some(Window {
                    cursor_options: CursorOptions {
                        grab_mode: CursorGrabMode::Locked,
                        ..default()
                    },
                    resolution: WindowResolution::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT),
                    title: "Wizard RPG".to_string(),
                    resizable: false,
                    focused: true,
                    ..default()
                }),
                ..default()
            })
            .set(LogPlugin {
                level: bevy::log::Level::INFO,
                ..default()
            }),
    )
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
        ..default()
    })
    .add_plugins((
        PhysicsPlugins::default(),
        GamePlayerPlugin,
        CharacterPlugin,
        DevRoomPlugin,
        UiPlugin,
        ShootPlugin,
        ControllerPlugin,
        InventoryPlugin,
        InteractPlugin,
        MyConsolePlugin,
        TradePlugin,
        BlenderTranslationPlugin,
        GameRenderPlugin,
        ChairPlugin,
        BlenvyPlugin::default(),
    ))
    .add_plugins((
            TestsPlugin,
            Sprite3dPlugin,
            DialogPlugin,
            ComputerPlugin,
            ItemPlugin,
    ));
    //.add_plugins(TestsPlugin)
    //.add_plugins(Sprite3dPlugin)
    //.add_plugins(DialogPlugin)
    //.add_plugins(ComputerPlugin)
    //.add_plugins(ItemPlugin);
    //.add_plugins(WorldInspectorPlugin::new());

    if args.editor {
        app.add_plugins((
            YoleckPluginForEditor,
            WorldInspectorPlugin::new(),
        ));
    } else {
        app.add_plugins(YoleckPluginForGame);
    }
    app.register_type::<RigidBody>()
        .init_state::<GameState>()
        .add_sub_state::<PauseMenuState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Gameplay)
                .on_failure_continue_to_state(GameState::Gameplay), //.load_collection::<ImageAssets>(),
        );
        app.run();
}

fn pause_game(
    key: Res<ButtonInput<KeyCode>>,
    console_open: Res<ConsoleOpen>,
    mut game_state: ResMut<State<GameState>>,
    mut pause_menu_state: ResMut<State<PauseMenuState>>,
) {
    trace!("SYSTEM: pause_game");
    if console_open.open {
    }
}
