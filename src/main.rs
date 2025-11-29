use avian3d::prelude::*;
use bevy::{
    log::LogPlugin, prelude::*, window::{ CursorGrabMode, CursorOptions, WindowResolution,}
};
use bevy_asset_loader::prelude::*;
use bevy_egui::EguiGlobalSettings;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_skein::SkeinPlugin;
use bevy_sprite3d::Sprite3dPlugin;
use bevy_yoleck::prelude::*;
use clap::Parser;

mod character;
mod console;
mod controller;
mod devroom;
mod dialog;
mod enemy;
mod furniture;
mod interact;
mod inventory;
mod items;
mod level;
mod magic;
mod navmesh;
mod player;
mod render;
mod rover;
mod shoot;
mod sprites;
//mod tests;
mod ui;
mod utils;

pub use character::*;
pub use console::*;
pub use controller::*;
pub use devroom::*;
pub use dialog::*;
pub use furniture::*;
pub use interact::*;
pub use inventory::*;
pub use items::*;
pub use navmesh::*;
pub use player::*;
pub use render::*;
pub use rover::*;
pub use shoot::*;
pub use sprites::*;
pub use ui::*;
pub use utils::*;
use level::*;
//use tests::TestsPlugin;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    editor: bool,
    #[clap(long)]
    inspector: bool,
    #[clap(long)]
    level: Option<String>,
}

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, SubStates, Reflect)]
#[source(GameState = GameState::Paused)]
pub enum PauseMenuState {
    ControllerSettings,
    GameplaySettings,
    #[default]
    MainMenu,
    Settings,
    SoundSettings,
    VideoSettings,
}

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, States, Reflect)]
pub enum GameState {
    Console,
    Gameplay,
    Inventory,
    Loading,
    MainMenu,
    Paused,
    #[default]
    Preload,
    Postload,
    GameOver,
    StartMenu,
}

fn main() {
    trace!("MAIN");
    let args = Args::parse();
    let mut app = App::new();
    app
    .insert_resource(EguiGlobalSettings {
        auto_create_primary_context: false,
        ..default()
    })
    .add_plugins(
        DefaultPlugins .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT),
                    title: "Bevy RPG".to_string(),
                    resizable: false,
                    focused: true,
                    ..default()
                }),
                primary_cursor_options: Some(CursorOptions {
                    grab_mode: CursorGrabMode::Locked,
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
        BlenderTranslationPlugin,
        GameRenderPlugin,
        FurniturePlugin,
        ItemPlugin,
        SkeinPlugin::default(),
    ))
    .add_plugins((
            //TestsPlugin,
            Sprite3dPlugin,
            DialogPlugin,
            NavMeshPlugin,
            SpritesPlugin,
    ));
    app.add_systems(Update, pause_game);

    if args.editor {
        app.add_plugins((
            YoleckPluginForEditor,
            WorldInspectorPlugin::new(),
        ));
    } else {
        app.add_plugins(YoleckPluginForGame);
    }
    if args.inspector {
        app.add_plugins(WorldInspectorPlugin::new());
    }
    app.register_type::<RigidBody>()
        .register_type::<GameState>()
        .register_type::<PauseMenuState>()
        .init_state::<GameState>()
        .add_sub_state::<PauseMenuState>()
        .add_loading_state(
            LoadingState::new(GameState::Preload)
                .continue_to_state(GameState::Loading)
                .on_failure_continue_to_state(GameState::Gameplay)
        )
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Postload)
                .on_failure_continue_to_state(GameState::Postload)
        )
        .add_loading_state(
            LoadingState::new(GameState::Postload)
                .continue_to_state(GameState::Gameplay)
                .on_failure_continue_to_state(GameState::Gameplay)
        );

        if let Some(level) = args.level {
            app.world_mut().write_message(ChangeLevelMessage(level.into()));
        } else {
            app.world_mut().write_message(ChangeLevelMessage("levels/World.glb".into()));
        }

        app.run();
}

fn pause_game(
    key: Res<ButtonInput<KeyCode>>,
    game_state: ResMut<State<GameState>>,
    mut  game_state_setter: ResMut<NextState<GameState>>,
    mut physics_time: ResMut<Time<Physics>>,
    //pause_menu_state: ResMut<State<PauseMenuState>>,
    //mut pause_menu_state_setter: ResMut<NextState<PauseMenuState>>,
) {
    trace!("SYSTEM: pause_game");
    if key.just_pressed(KeyCode::Comma) {
        match game_state.get() {
            GameState::Gameplay => {
                game_state_setter.set(GameState::Paused);
                physics_time.pause();
            },
            GameState::Paused   => {
                game_state_setter.set(GameState::Gameplay);
                physics_time.unpause();
            },
            _                   => {}
        }
    }
}
