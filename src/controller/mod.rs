mod console_controller;
mod interact_controller;
mod inventory_controller;
mod menu_controller;

use console_controller::*;
use interact_controller::*;
use inventory_controller::*;
use menu_controller::*;

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_fps_controller::controller::*;
use super::GameState;

use crate::shoot;
use crate::interact::InteractEvent;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractEvent>()
            .add_systems(Update, manage_cursor)//.run_if(in_state(GameState::Gameplay)))
            .add_systems(Update, manage_interact.run_if(in_state(GameState::Gameplay)))
            .add_systems(Update, manage_console.run_if(in_state(GameState::Gameplay)))
            .add_systems(Update, manage_inventory.run_if(in_state(GameState::Gameplay)))
            .add_systems(Update, manage_menu.run_if(in_state(GameState::Gameplay)))
            .add_systems(Update, inventory_navigation.run_if(in_state(GameState::Gameplay)));
    }
}

fn manage_cursor(
    mut windows: Query<&mut Window>,
    btn: Res<ButtonInput<MouseButton>>,
    _key: Res<ButtonInput<KeyCode>>,
    mut controllers: Query<&mut FpsController>,
    mut shoot_event_writer: EventWriter<shoot::ShootEvent>,
) {
    if let Ok(mut window) = windows.get_single_mut() {
        if window.cursor.grab_mode != CursorGrabMode::Locked {
            if btn.just_pressed(MouseButton::Left) {
                window.cursor.grab_mode = CursorGrabMode::Locked;
                window.cursor.visible = false;
                for mut controller in &mut controllers {
                    controller.enable_input = true;
                }
            }
        } else if btn.just_pressed(MouseButton::Left) {
            shoot_event_writer.send(shoot::ShootEvent);
        }

        /*if key.just_pressed(KeyCode::Escape) {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
            for mut controller in &mut controllers {
                controller.enable_input = false;
            }
        }*/
    }
}

fn weapon_select(
    key: Res<ButtonInput<KeyCode>>,
    ) {
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
