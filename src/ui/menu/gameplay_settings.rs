use bevy::prelude::*;

use crate::{MenuState, widgets::{self, ui_root}};

#[derive(Component, Reflect)]
pub struct UiGameplaySettings;

pub struct GameplaySettingsMenuUiPlugin;
impl Plugin for GameplaySettingsMenuUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app
           .register_type::<UiGameplaySettings>()
           .add_systems(OnEnter(MenuState::GameplaySettings), spawn_gameplay_settings_menu);
    }
}

fn spawn_gameplay_settings_menu(
    mut commands: Commands,
) {
    commands.spawn((
            ui_root("Gameplay Settings"),
            DespawnOnExit(MenuState::GameplaySettings),
            GlobalZIndex(2),
            UiGameplaySettings,
            children![
                widgets::button("Back", back_to_settings_menu),
            ],
    ));
}

fn back_to_settings_menu(
    _: On<Pointer<Click>>,
    mut pause_menu_state: ResMut<NextState<MenuState>>,
) {
    pause_menu_state.set(MenuState::Settings);
}
