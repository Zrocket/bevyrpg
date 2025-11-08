use bevy::prelude::*;

use super::widgets;

use crate::{PauseMenuState};

#[derive(Component, Reflect)]
pub struct UiSettings;

pub struct SettingsMenuUiPlugin;
impl Plugin for SettingsMenuUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app
           .register_type::<UiSettings>()
           .add_systems(OnEnter(PauseMenuState::Settings), spawn_settings_menu);
    }
}

fn spawn_settings_menu(
    mut commands: Commands,
) {
    commands.spawn((
            widgets::ui_root("Settings Menu"),
            DespawnOnExit(PauseMenuState::Settings),
            GlobalZIndex(2),
            UiSettings,
            children![
                widgets::button("Gameplay Settings", enter_gameplay_settings_menu),
                widgets::button("Controller Settings", enter_controller_settings_menu),
                widgets::button("Video Settings", enter_video_settings_menu),
                widgets::button("Sound Settings", enter_sound_settings_menu),
                widgets::button("Back", back_to_main_menu),
            ]
    ));
}

fn back_to_main_menu(
    _: On<Pointer<Click>>,
    mut pause_menu_state: ResMut<NextState<PauseMenuState>>,
) {
    pause_menu_state.set(PauseMenuState::MainMenu);
}

fn enter_sound_settings_menu(
    _: On<Pointer<Click>>,
    mut pause_menu_state: ResMut<NextState<PauseMenuState>>,
) {
    pause_menu_state.set(PauseMenuState::SoundSettings);
}

fn enter_video_settings_menu(
    _: On<Pointer<Click>>,
    mut pause_menu_state: ResMut<NextState<PauseMenuState>>,
) {
    pause_menu_state.set(PauseMenuState::VideoSettings);
}

fn enter_controller_settings_menu(
    _: On<Pointer<Click>>,
    mut pause_menu_state: ResMut<NextState<PauseMenuState>>,
) {
    pause_menu_state.set(PauseMenuState::ControllerSettings);
}

fn enter_gameplay_settings_menu(
    _: On<Pointer<Click>>,
    mut pause_menu_state: ResMut<NextState<PauseMenuState>>,
) {
    pause_menu_state.set(PauseMenuState::GameplaySettings);
}
