/*use crate::{PauseMenuState, UiControllerSettings, UiMenu, UiSettings, UiSoundSettings};
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
}*/
