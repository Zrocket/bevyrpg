use bevy::prelude::*;

use crate::{MenuState, widgets::{self, ui_root}};

#[derive(Component, Reflect)]
pub struct UiControllerSettings;

pub struct ControllerSettingsMenuUiPlugin;
impl Plugin for ControllerSettingsMenuUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app
           .register_type::<UiControllerSettings>()
           .add_systems(OnEnter(MenuState::ControllerSettings), spawn_controller_settings_menu);
    }
}

fn spawn_controller_settings_menu(
    mut commands: Commands,
) {
    commands.spawn((
            ui_root("Controller Settings"),
            DespawnOnExit(MenuState::ControllerSettings),
            GlobalZIndex(2),
            UiControllerSettings,
            children![
            (
                widgets::label("Mouse Sensetivity"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                },
            ),
            (
                widgets::label("Key Bindings"),
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
