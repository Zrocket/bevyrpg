use std::f32::consts::TAU;

use bevy::{
    prelude::*,
    utils::Duration,
    window::{CursorGrabMode, WindowResolution, Cursor},
};
use bevy_asset_loader::prelude::*;
use bevy_fps_controller::controller::*;
use bevy_rapier3d::prelude::*;
use bevy_sprite3d::*;
use serde::{Deserialize, Serialize};

mod character;
mod devroom;
mod player;
mod ui;
mod shoot;
mod interact;
mod controller;
mod items;
mod inventory;

pub use character::*;
pub use devroom::*;
pub use player::*;
pub use ui::*;
pub use shoot::*;
pub use interact::*;
pub use controller::*;
pub use items::*;
pub use inventory::*;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

#[derive(Clone, Hash, Debug, Eq, PartialEq, States, Default)]
pub enum GameState {
    MainMenu,
    UiMenu,
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
                cursor: Cursor {
                    icon: CursorIcon::Crosshair,
                    ..default()
                },
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                title: "Wizard RPG".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(Sprite3dPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(FpsControllerPlugin)
        .add_plugins(CharacterPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(DevRoomPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(ShootPlugin)
        .add_plugins(ControllerPlugin)
        .add_plugins(InventoryPlugin)
        .add_systems(Update, health_test.run_if(in_state(GameState::Gameplay)))
        .run();
}

fn health_test(
    key: Res<Input<KeyCode>>,
    mut player: Query<(Entity, &Character), With<Player>>,
    mut damage_event_writer: EventWriter<DamageEvent>,
    ) {
    let (player_entity, player) = player.get_single_mut().unwrap();
    if key.just_pressed(KeyCode::K) {
        damage_event_writer.send(DamageEvent {
            target: player_entity,
            ammount: 5,
        });
    }
    //println!("{}, {}", player.health, (player.health as f32 / player.max_health as f32) * 100.);
}
