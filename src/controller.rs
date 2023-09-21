use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;
use bevy_fps_controller::controller::*;
use super::GameState;

use crate::PlayerUi;
use crate::shoot;
use crate::interact::Interactable;
use crate::interact::InteractEvent;
use crate::player::Player;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractEvent>()
            .add_systems(Update, manage_cursor.run_if(in_state(GameState::Gameplay)))
            .add_systems(Update, manage_interact.run_if(in_state(GameState::Gameplay)))
            .add_systems(Update, manage_console.run_if(in_state(GameState::Gameplay)))
            .add_systems(Update, manage_inventory.run_if(in_state(GameState::Gameplay)));
    }
}

fn manage_cursor(
    mut windows: Query<&mut Window>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut controllers: Query<&mut FpsController>,
    mut shoot_event_writer: EventWriter<shoot::ShootEvent>,
) {
    let mut window = windows.get_single_mut().unwrap();

    if window.cursor.grab_mode != CursorGrabMode::Locked {
        if btn.just_pressed(MouseButton::Left) {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
            for mut controller in &mut controllers {
                controller.enable_input = true;
            }
        }
    } else {
        if btn.just_pressed(MouseButton::Left) {
            shoot_event_writer.send(shoot::ShootEvent)
        }
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        for mut controller in &mut controllers {
            controller.enable_input = false;
        }
    }
}

fn manage_interact(
    key: Res<Input<KeyCode>>,
    mut interact_event_writer: EventWriter<InteractEvent>,
) {
    if key.just_pressed(KeyCode::E) {
        interact_event_writer.send(InteractEvent)
    }
}

fn manage_console(
    key: Res<Input<KeyCode>>,
    mut player_ui: Query<&mut PlayerUi>,
) {
    if key.just_pressed(KeyCode::Grave) {
        println!("GRAVE");
        let mut player_ui = player_ui.get_single_mut().unwrap();
        if player_ui.console {
            player_ui.console = false;
        } else {
            player_ui.console = true;
        }
    }
}

fn manage_inventory(
    key: Res<Input<KeyCode>>,
    mut player_ui: Query<&mut PlayerUi>,
) {
    if key.just_pressed(KeyCode::I) {
        println!("INV");
        let mut player_ui = player_ui.get_single_mut().unwrap();
        if player_ui.inventory {
            player_ui.inventory = false;
        } else {
            player_ui.inventory = true;
        }
    }
}

fn interact(
    mut interact_events: EventReader<InteractEvent>,
    query: Query<(&Camera, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
    player: Query<Entity, With<Player>>,
    interactables: Query<Entity, With<Interactable>>,
) {
    let player = player.get_single().unwrap();
    for _event in interact_events.iter() {
        for (_camera, global_transform) in query.iter() {
            let camera_position = global_transform.translation();
            let direction = global_transform.forward();

            if let Some((entity, toi)) = rapier_context.cast_ray(
                camera_position, direction, Real::MAX, false, QueryFilter {exclude_collider: Some(player), ..default()}
                ) {
                let hit_point = camera_position + direction * toi;
                println!("INTERACT Entity {:?} hit at point {}", entity, hit_point);
            }
        }
    }
}
