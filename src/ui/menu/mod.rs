use bevy::prelude::*;
use crate::{GameState, PauseMenuState, menu::{controller_settings::ControllerSettingsMenuUiPlugin, gameplay_settings::GameplaySettingsMenuUiPlugin, settings::SettingsMenuUiPlugin, sound_settings::SoundSettingsMenuUiPlugin, video_settings::VideoSettingsMenuUiPlugin}};

use super::widgets;

pub mod controller_settings;
pub mod gameplay_settings;
pub mod settings;
pub mod sound_settings;
pub mod video_settings;

#[derive(Component, Reflect)]
pub struct UiMenu;

pub struct MenuUiPlugin;
impl Plugin for MenuUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UiMenu>()
            .add_systems(OnEnter(PauseMenuState::MainMenu), spawn_pause_menu)
            .add_plugins((
                    SettingsMenuUiPlugin,
                    ControllerSettingsMenuUiPlugin,
                    GameplaySettingsMenuUiPlugin,
                    SoundSettingsMenuUiPlugin,
                    VideoSettingsMenuUiPlugin,
            ));
    }
}

fn spawn_pause_menu(
    mut commands: Commands,
) {
    commands.spawn((
            widgets::ui_root("Pause Menu"),
            DespawnOnExit(PauseMenuState::MainMenu),
            GlobalZIndex(2),
            children![
                widgets::button("Settings", enter_settings_menu)
            ]
    ));
}

fn enter_settings_menu(
    _: On<Pointer<Click>>,
    mut pause_menu_state: ResMut<NextState<PauseMenuState>>,
) {
    pause_menu_state.set(PauseMenuState::Settings);
}
