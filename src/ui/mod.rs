//use bevy_simple_text_input::{TextInput, TextInputPlugin, TextInputSubmitEvent};
use super::*;

mod console;
mod crosshair;
mod inventory_ui;
mod status_bar;

use console::*;
use crosshair::*;
use inventory_ui::*;
use status_bar::*;

#[derive(Component)]
pub struct PlayerUi {
    pub status_bar: bool,
    pub inventory: bool,
    pub console: bool,
    pub crosshair: bool,
}

#[derive(Component)]
pub struct UiEntity;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), spawn_ui)
            .add_systems(
                Update,
                update_ui.run_if(in_state(GameState::Gameplay))
            );
    }
}

fn update_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_ui: Query<&PlayerUi, With<PlayerUi>>,
    player: Query<(Entity, &Character, &Inventory), With<Player>>,
    ui_entities: Query<Entity, With<UiEntity>>,
) {
    for ui_entity in  ui_entities.iter() {
        commands.entity(ui_entity).despawn_recursive();
    }

    if let Ok(player_entity) = player.get_single() {
        if let Ok(player_ui) = player_ui.get_single() {
            if player_ui.status_bar {
                create_ui(&mut commands, &asset_server, &player_entity.1);
            }
            if player_ui.inventory {
                create_inventory_ui(&mut commands, &asset_server, &player_entity.2);
            }
            if player_ui.console {
                create_console_ui(&mut commands, &asset_server);
            }
            if player_ui.crosshair {
                create_crosshair(&mut commands, &asset_server);
            }
        }
    }
}

fn spawn_ui(
    mut commands: Commands,
) {
    commands.spawn(
       PlayerUi {
            status_bar: true,
            inventory: false,
            console: false,
            crosshair: true,
       });
}
