use avian_pickup::{AvianPickupPlugin};
use avian_rerecast::AvianBackendPlugin;
use avian3d::prelude::*;
use bevy::{
    color::palettes::css::GREEN, log::LogPlugin, prelude::*, text::FontSmoothing, window::{ CursorGrabMode, CursorOptions, WindowResolution,},
    dev_tools::fps_overlay::{FpsOverlayPlugin, FpsOverlayConfig, FrameTimeGraphConfig},
};
use bevy_asset_loader::prelude::*;
use bevy_bae::BaePlugin;
use bevy_egui::{EguiGlobalSettings, EguiPlugin};
use bevy_hanabi::HanabiPlugin;
//use bevy_hotpatching_experiments::SimpleSubsecondPlugin;
use bevy_ingame_clock::InGameClockPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_landmass::{Landmass3dPlugin, debug::Landmass3dDebugPlugin};
//use bevy_mod_scripting::BMSPlugin;
use bevy_rerecast::NavmeshPlugins;
use bevy_simple_text_input::TextInputPlugin;
use bevy_skein::SkeinPlugin;
use bevy_sprite3d::Sprite3dPlugin;
use bevy_sun_move::{SunMovePlugin, random_stars::RandomStarsPlugin};
use bevy_yarnspinner::prelude::YarnSpinnerPlugin;
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewPlugin;
use bevy_yoleck::prelude::*;
use clap::Parser;
use landmass_rerecast::LandmassRerecastPlugin;

mod audio;
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
mod npc;
mod particles;
mod player;
mod quest;
mod render;
mod rover;
mod shoot;
mod sprites;
mod states;
mod tests;
mod ui;
mod utils;

pub use audio::*;
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
pub use npc::*;
pub use player::*;
pub use quest::*;
pub use render::*;
pub use rover::*;
pub use shoot::*;
pub use sprites::*;
pub use states::*;
pub use ui::*;
pub use utils::*;
use level::*;
use tests::TestsPlugin;

use crate::{enemy::EnemyPlugin, particles::ParticlePlugin};

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    editor: bool,
    #[clap(long)]
    inspector: bool,
    #[clap(long)]
    level: Option<String>,
    #[clap(long)]
    fps: bool,
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
                level: bevy::log::Level::TRACE,
                //filter: "info,wgpu=error,bevy_landmass=trace,bevyrpg=trace".into(),
                filter: "info,wgpu=error".into(),
                ..default()
            }),
    )
    .insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
        ..default()
    })
    // Library Plugins
    .add_plugins((
        PhysicsPlugins::default(),
        AvianPickupPlugin::default(),
        YarnSpinnerPlugin::new(),
        ExampleYarnSpinnerDialogueViewPlugin::new(),
        Sprite3dPlugin,
        SunMovePlugin,
        RandomStarsPlugin,
        Landmass3dPlugin::default(),
        Landmass3dDebugPlugin::default(),
        LandmassRerecastPlugin::default(),
        NavmeshPlugins::default(),
        AvianBackendPlugin::default(),
        BaePlugin::default(),
        HanabiPlugin,
        TextInputPlugin,
    ))
    .add_plugins((
        SkeinPlugin::default(),
        //SimpleSubsecondPlugin::default(),
        //BMSPlugin,
        InGameClockPlugin,
    ))
    // Crate Plugins
    .add_plugins((
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
        AudioPlugin,
        TestsPlugin,
    ))
    .add_plugins((
        StatesPlugin,
        DialogPlugin,
        NavMeshPlugin,
        SpritesPlugin,
        EnemyPlugin,
        ParticlePlugin,
        NpcPlugin,
        QuestPlugin,
        RoverPlugin,
    ));
    app.add_systems(Update, pause_game.run_if(in_state(MetaState::Gameplay)));

    if args.editor {
        app.add_plugins(EguiPlugin::default());
        app.add_plugins((
            YoleckPluginForEditor,
            WorldInspectorPlugin::new(),
        ));
    } else {
        app.add_plugins(YoleckPluginForGame);
    }
    if args.inspector {
        app.add_plugins(EguiPlugin::default());
        app.add_plugins(WorldInspectorPlugin::new());
    }
    if args.fps {
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 42.0,
                    font: default(),
                    font_smoothing: FontSmoothing::default(),
                    ..default()
                },
                text_color: Color::srgb(0.0, 1.0, 0.0),
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: true,
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: true,
                    // The minimum acceptable fps
                    min_fps: 30.0,
                    // The target fps
                    target_fps: 144.0,
                },
            }
        });
    }
    app.register_type::<RigidBody>();

        if let Some(level) = args.level {
            app.world_mut().write_message(ChangeLevelMessage(level));
        } else {
            app.world_mut().write_message(ChangeLevelMessage("levels/World.glb".into()));
        }

        app.run();
}

fn pause_game(
    key: Res<ButtonInput<KeyCode>>,
    game_state: ResMut<State<GameState>>,
    mut  game_state_setter: ResMut<NextState<GameState>>,
    mut  menu_state_setter: ResMut<NextState<MenuState>>,
    mut physics_time: ResMut<Time<Physics>>,
) {
    trace!("SYSTEM: pause_game");
    if key.just_pressed(KeyCode::Comma) {
        match game_state.get() {
            GameState::Gameplay => {
                game_state_setter.set(GameState::Paused);
                menu_state_setter.set(MenuState::MainMenu);
                physics_time.pause();
            },
            GameState::Paused   => {
                game_state_setter.set(GameState::Gameplay);
                menu_state_setter.set(MenuState::Off);
                physics_time.unpause();
            },
            _                   => {}
        }
    }
}
