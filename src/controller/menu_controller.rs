use crate::{PauseMenuState, UiControllerSettings, UiIndex, UiMenu, UiSettings, UiSoundSettings};
use bevy::prelude::*;

pub struct MenuControllerPlugin;
impl Plugin for MenuControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PauseMenuState::MainMenu), ui_open::<UiMenu>)
            .add_systems(OnExit(PauseMenuState::MainMenu), ui_close::<UiMenu>)
            .add_systems(OnEnter(PauseMenuState::Settings), ui_open::<UiSettings>)
            .add_systems(OnExit(PauseMenuState::Settings), ui_close::<UiSettings>)
            .add_systems(OnEnter(PauseMenuState::SoundSettings), ui_open::<UiSoundSettings>)
            .add_systems(OnExit(PauseMenuState::SoundSettings), ui_close::<UiSoundSettings>)
            .add_systems(OnEnter(PauseMenuState::ControllerSettings), ui_open::<UiControllerSettings>)
            .add_systems(OnExit(PauseMenuState::ControllerSettings), ui_close::<UiControllerSettings>);
    }
}

pub fn ui_open<T: Component>(
    mut node_query: Query<&mut Node, With<T>>,
) {
    for mut node in node_query.iter_mut() {
        node.display = Display::Flex;
    }
}

pub fn ui_close<T: Component>(
    mut node_query: Query<&mut Node, With<T>>,
) {
    for mut node in node_query.iter_mut() {
        node.display = Display::None;
    }
}

pub fn _menu_navigation(
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
