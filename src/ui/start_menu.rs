use bevy::prelude::*;

use crate::{GameState, PauseMenuState, widgets};

#[derive(Component, Reflect)]
pub struct UiStartMenu;

pub struct StartMenuUiPlugin;
impl Plugin for StartMenuUiPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(OnEnter(GameState::StartMenu), spawn_start_menu);
    }
}

fn spawn_start_menu(
    mut commands: Commands,
) {
    commands.spawn((
        widgets::ui_root("Start Menu"),
        DespawnOnExit(GameState::StartMenu),
        GlobalZIndex(2),
        children![
            widgets::button("Start Game", start_game),
            widgets::button("Settings", start_settings),
            widgets::button("Credits", start_credits),
        ]
    ));
}

fn start_credits(
    _trigger: On<Pointer<Click>>,
    mut game_state: ResMut<NextState<PauseMenuState>>,
) {
    game_state.set(PauseMenuState::Credits);
}

fn start_settings(
    _trigger: On<Pointer<Click>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_state.set(GameState::Paused);
}

fn start_game(
    _trigger: On<Pointer<Click>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_state.set(GameState::Gameplay);
}
