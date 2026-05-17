use bevy::prelude::*;
use crate::{MenuState, credits::CreditsMenuUiPlugin, load_game::LoadGameMenuUiPlugin, loading::LoadingMenuUiPlugin, menu::{controller_settings::ControllerSettingsMenuUiPlugin, gameplay_settings::GameplaySettingsMenuUiPlugin, settings::SettingsMenuUiPlugin, sound_settings::SoundSettingsMenuUiPlugin, video_settings::VideoSettingsMenuUiPlugin}};

use super::widgets;

pub mod controller_settings;
pub mod credits;
pub mod gameplay_settings;
pub mod load_game;
pub mod save_game;
pub mod loading;
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
            .add_systems(OnEnter(MenuState::PauseMenu), spawn_pause_menu)
            .add_plugins((
                    SettingsMenuUiPlugin,
                    ControllerSettingsMenuUiPlugin,
                    CreditsMenuUiPlugin,
                    GameplaySettingsMenuUiPlugin,
                    SoundSettingsMenuUiPlugin,
                    VideoSettingsMenuUiPlugin,
                    LoadingMenuUiPlugin,
                    LoadGameMenuUiPlugin,
            ));
    }
}

fn spawn_pause_menu(
    mut commands: Commands,
) {
    commands.spawn((
            widgets::ui_root("Pause Menu"),
            DespawnOnExit(MenuState::PauseMenu),
            GlobalZIndex(2),
            children![
                widgets::button("Settings", enter_settings_menu),
                widgets::button("Credits", enter_pause_menu),
            ]
    ));
}

fn enter_settings_menu(
    _: On<Pointer<Click>>,
    mut pause_menu_state: ResMut<NextState<MenuState>>,
) {
    pause_menu_state.set(MenuState::Settings);
}

fn enter_pause_menu(
    _: On<Pointer<Click>>,
    mut pause_menu_state: ResMut<NextState<MenuState>>,
) {
    pause_menu_state.set(MenuState::Credits);
}
