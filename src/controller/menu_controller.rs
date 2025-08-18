use crate::{ActiveUi, PauseMenuState, Player, UiIndex, UiMenu};
use bevy::prelude::*;

pub struct MenuControllerPlugin;
impl Plugin for MenuControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PauseMenuState::MainMenu), open_pause_menu)
            .add_systems(OnExit(PauseMenuState::MainMenu), close_pause_menu)
            .add_systems(Update, menu_navigation);
    }
}

pub fn _manage_menu(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    mut menu_node_query: Query<&mut Node, With<UiMenu>>,
) {
    if key.just_pressed(KeyCode::Semicolon)
    && let Ok(mut menu_node) = menu_node_query.single_mut() {
        match menu_node.display {
            Display::None => menu_node.display = Display::Flex,
            _ => menu_node.display = Display::None,
        }
    }
}

pub fn open_pause_menu(
    mut menu_node_query: Query<&mut Node, With<UiMenu>>,
) {
    if let Ok(mut menu_node) = menu_node_query.single_mut() {
        menu_node.display = Display::Flex;
    }
}

pub fn close_pause_menu(
    mut menu_node_query: Query<&mut Node, With<UiMenu>>,
) {
    if let Ok(mut menu_node) = menu_node_query.single_mut() {
        menu_node.display = Display::None;
    }
}

pub fn menu_navigation(
    key: Res<ButtonInput<KeyCode>>,
    mut menu_node_query: Query<&mut Node, With<UiMenu>>,
    mut index_query: Query<&mut UiIndex, With<UiMenu>>,
) {
    if let Ok(mut _menu_node) = menu_node_query.single_mut() {
        for mut index in index_query.iter_mut() {
            // navigate up
            if (key.just_pressed(KeyCode::KeyW) || key.just_pressed(KeyCode::ArrowUp)) && index.0 > 0 {
                index.0 -= 1;
            // navigate down
            } else if key.just_pressed(KeyCode::KeyS) || key.just_pressed(KeyCode::ArrowDown) {
                index.0 += 1;
            // select
            } else if key.just_pressed(KeyCode::Enter) {
                todo!();
            }
        }
    }
}
