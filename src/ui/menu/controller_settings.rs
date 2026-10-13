use bevy::prelude::*;

use crate::{MenuState, widgets::{self, ui_root}};

#[derive(Component, Reflect)]
pub struct UiControllerSettings;

#[derive(Component, Reflect)]
pub struct UiKeyBindSettings;

pub struct ControllerSettingsMenuUiPlugin;
impl Plugin for ControllerSettingsMenuUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app
           .register_type::<UiControllerSettings>()
           .add_systems(OnEnter(MenuState::ControllerSettings), spawn_controller_settings_menu)
           .add_systems(OnEnter(MenuState::KeyBindings), spawn_key_bindings_menu);
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

fn get_key_binding() {
}

fn spawn_key_bindings_menu(
    mut commands: Commands,
) {
    commands.spawn((
            ui_root("Key Bindings"),
            DespawnOnExit(MenuState::KeyBindings),
            GlobalZIndex(2),
            UiKeyBindSettings,
            children![(
                (
                    widgets::label("Forward: "),
                    Node {
                        justify_self: JustifySelf::End,
                        ..default()
                    },
                ),
                (
                    widgets::label("Backward: "),
                    Node {
                        justify_self: JustifySelf::End,
                        ..default()
                    },
                ),
                (
                    widgets::label("Left: "),
                    Node {
                        justify_self: JustifySelf::End,
                        ..default()
                    },
                ),
                (
                    widgets::label("Right: "),
                    Node {
                        justify_self: JustifySelf::End,
                        ..default()
                    },
                ),
                (
                    widgets::label("Dash: "),
                    Node {
                        justify_self: JustifySelf::End,
                        ..default()
                    },
                ),
                (
                    widgets::label("Jump: "),
                    Node {
                        justify_self: JustifySelf::End,
                        ..default()
                    },
                ),
                (
                    widgets::label("Flashlight: "),
                    Node {
                        justify_self: JustifySelf::End,
                        ..default()
                    },
                ),
                (
                    widgets::label("Interact: "),
                    Node {
                        justify_self: JustifySelf::End,
                        ..default()
                    },
                ),
                (
                    widgets::label("Inventory: "),
                    Node {
                        justify_self: JustifySelf::End,
                        ..default()
                    },
                ),
                widgets::button("Back", back_to_controller_settings_menu),
            )],
        ));
}

fn back_to_controller_settings_menu(
    _: On<Pointer<Click>>,
    mut pause_menu_state: ResMut<NextState<MenuState>>,
) {
    pause_menu_state.set(MenuState::ControllerSettings);
}
