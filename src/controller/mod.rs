mod interact_controller;
mod inventory_controller;
mod menu_controller;
mod player_controller;

use bevy::prelude::*;
use bevy_tnua::TnuaUserControlsSystemSet;
use interact_controller::*;
use inventory_controller::*;
use menu_controller::*;
pub use player_controller::*;

use super::GameState;
use bevy::window::CursorGrabMode;
use leafwing_input_manager::prelude::*;

use crate::interact::InteractEvent;
use crate::shoot;

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
#[actionlike(DualAxis)]
pub enum CameraMovement {
    Pan,
}

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractEvent>()
            .add_plugins(InputManagerPlugin::<Action>::default())
            //.add_systems(Update, manage_cursor) //.run_if(in_state(GameState::Gameplay)))
            .add_systems(
                Update,
                (
                    manage_cursor,
                    manage_interact.run_if(in_state(GameState::Gameplay)),
                    manage_inventory.run_if(in_state(GameState::Gameplay)),
                    inventory_navigation.in_set(TnuaUserControlsSystemSet),
                    manage_menu.run_if(in_state(GameState::Gameplay)),
                )
            );

        app.add_systems(
            Update,
            (
                player_controller_input,
                player_controller_look,
                //fps_controller_move,
                tnua_player_input,
            )
                .chain(), //.after(bevy::input::mouse::mouse_button_input_system)
                          //.after(bevy::input::keyboard::keyboard_input_system)
                          //.after(bevy::input::gamepad::gamepad_axis_event_system)
                          //.after(bevy::input::gamepad::gamepad_button_event_system)
                          //.after(bevy::input::gamepad::gamepad_connection_system)
                          //.after(bevy::input::gamepad::gamepad_event_system)
                          //.after(bevy::input::touch::touch_screen_input_system),
        );
    }
}

fn manage_cursor(
    mut windows: Query<&mut Window>,
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut controllers: Query<&mut PlayerController>,
    mut shoot_event_writer: EventWriter<shoot::ShootEvent>,
) {
    if let Ok(mut window) = windows.get_single_mut() {
        if window.cursor_options.grab_mode != CursorGrabMode::Locked {
            if btn.just_pressed(MouseButton::Left) {
                window.cursor_options.grab_mode = CursorGrabMode::Locked;
                window.cursor_options.visible = false;
                for mut controller in &mut controllers {
                    controller.enable_input = true;
                }
            }
        } else if btn.just_pressed(MouseButton::Left) {
            shoot_event_writer.send(shoot::ShootEvent);
        }

        if key.just_pressed(KeyCode::Escape) {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
            for mut controller in &mut controllers {
                controller.enable_input = false;
            }
        }
    }
}

fn weapon_select(key: Res<ButtonInput<KeyCode>>) {
    if key.just_pressed(KeyCode::Digit1) {
    } else if key.just_pressed(KeyCode::Digit2) {
        todo!();
    } else if key.just_pressed(KeyCode::Digit3) {
        todo!();
    } else if key.just_pressed(KeyCode::Digit4) {
        todo!();
    } else if key.just_pressed(KeyCode::Digit5) {
        todo!();
    }
}
