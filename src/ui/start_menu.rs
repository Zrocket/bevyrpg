use bevy::prelude::*;

use crate::{GameState, widgets};

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
        ]
    ));
}

fn start_game(
    _trigger: On<Pointer<Click>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_state.set(GameState::Gameplay);
}
