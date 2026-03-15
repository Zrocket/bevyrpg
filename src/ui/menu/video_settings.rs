use bevy::prelude::*;

use crate::{MenuState,  widgets::{self, ui_root}};

#[derive(Component, Reflect)]
pub struct UiVideoSettings;

pub struct VideoSettingsMenuUiPlugin;
impl Plugin for VideoSettingsMenuUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app
           .register_type::<UiVideoSettings>()
           .add_systems(OnEnter(MenuState::VideoSettings), spawn_video_settings_menu);
    }
}

fn spawn_video_settings_menu(
    mut commands: Commands,
) {
    commands.spawn((
            ui_root("Video Settings"),
            DespawnOnExit(MenuState::VideoSettings),
            GlobalZIndex(2),
            UiVideoSettings,
            children![
            (
                widgets::label("Camera Sensetivity"),
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
