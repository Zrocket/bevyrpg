use bevy::prelude::*;

use crate::{MenuState, MetaState, widgets};

#[derive(Component, Reflect)]
pub struct UiStartMenu;

pub struct StartMenuUiPlugin;
impl Plugin for StartMenuUiPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(OnEnter(MetaState::MainMenu), spawn_start_menu);
    }
}

fn spawn_start_menu(
    mut commands: Commands,
) {
    commands.spawn((
        widgets::ui_root("Start Menu"),
        DespawnOnExit(MetaState::MainMenu),
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
    mut game_state: ResMut<NextState<MenuState>>,
) {
    game_state.set(MenuState::Credits);
}

fn start_settings(
    _trigger: On<Pointer<Click>>,
    mut game_state: ResMut<NextState<MenuState>>,
) {
    game_state.set(MenuState::Settings);
}

fn start_game(
    _trigger: On<Pointer<Click>>,
    mut game_state: ResMut<NextState<MetaState>>,
) {
    game_state.set(MetaState::Gameplay);
}
