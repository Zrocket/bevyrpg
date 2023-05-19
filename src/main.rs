use rand::{distributions::Standard, prelude::SliceRandom, Rng};
use std::f32::consts::TAU;

use bevy::{
    core_pipeline::{bloom::BloomSettings, clear_color::ClearColorConfig},
    prelude::*,
    utils::Duration,
    window::CursorGrabMode,
    window::WindowResolution,
};
use bevy_asset_loader::prelude::*;
use bevy_fps_controller::controller::*;
use bevy_mod_picking::*;
use bevy_mod_ui_texture_atlas_image::*;
use bevy_rapier3d::prelude::*;
use bevy_sprite3d::*;

mod character;
mod devroom;
mod player;
mod ui;

pub use character::*;
pub use devroom::*;
pub use player::*;

pub use ui::*;

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
    Loading,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Gameplay),
        )
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
        .add_plugin(CharacterPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(DevRoomPlugin)
        .add_system(manage_cursor.in_set(OnUpdate(GameState::Gameplay)))
        .add_system(health_test.in_set(OnUpdate(GameState::Gameplay)))
        .run();
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

fn health_test(
    key: Res<Input<KeyCode>>,
    mut player: Query<(Entity, &mut Character), With<Player>>,
) {
    let mut player = player.get_single_mut().unwrap();
    if key.just_pressed(KeyCode::K) {
        player.1.health -= 10;
    }
}
