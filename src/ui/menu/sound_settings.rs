use bevy::prelude::*;

use crate::{MenuState, widgets::{self, ui_root}};

#[derive(Component, Reflect)]
pub struct UiSoundSettings;

pub struct SoundSettingsMenuUiPlugin;
impl Plugin for SoundSettingsMenuUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app
           .register_type::<UiSoundSettings>()
           .add_systems(OnEnter(MenuState::SoundSettings), spawn_sound_settings_menu);
    }
}

fn spawn_sound_settings_menu(
    mut commands: Commands,
) {
    commands.spawn((
            ui_root("Sound Settings"),
            DespawnOnExit(MenuState::SoundSettings),
            GlobalZIndex(2),
            UiSoundSettings,
            children![
            (
                widgets::label("Master Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                },
            ),
            (
                widgets::label("Music Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                },
            ),
            (
                widgets::label("Sound Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                },
            ),
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
