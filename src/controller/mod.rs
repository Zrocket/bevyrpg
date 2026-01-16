mod equip_controller;
mod input;
mod interact_controller;
mod inventory_controller;
mod player_controller;
mod stats_controller;

use equip_controller::*;
pub use input::*;
use avian_pickup::prelude::*;
use bevy::prelude::*;
use bevy_tnua::TnuaUserControlsSystems;
pub use interact_controller::*;
use inventory_controller::*;
pub use player_controller::*;
pub use stats_controller::*;

use bevy::window::{CursorGrabMode, CursorOptions};

use crate::{ShootEvent, shoot, widgets::floating_windows::FloatingWindow};

pub struct ControllerPlugin;
impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_plugins(InputManagerPlugin::<LeafwingAction>::default())
            .add_plugins(InputPlugin)
            .add_plugins(InteractControllerPlugin)
            .add_plugins(PlayerControllerPlugin)
            .add_plugins(InventoryControllerPlugin)
            .add_plugins(EquipControllerPlugin)
            .add_plugins(StatsControllerPlugin)
            .register_type::<RayHit>()
            .add_systems(
                Update,
                (
                    manage_cursor,
                    //manage_interact.run_if(in_state(GameState::Gameplay)).run_if(input_just_pressed(KeyCode::KeyE)),
                    //manage_interact.run_if(in_state(GameState::Gameplay)).run_if(input_just_pressed(KeyCode::KeyE)),
                    //manage_inspect.run_if(in_state(GameState::Gameplay)),
                    //player_raycast.run_if(in_state(GameState::Gameplay)),
                    inventory_navigation.in_set(TnuaUserControlsSystems),
                )
            );

    }
}

#[allow(clippy::too_many_arguments)]
fn manage_cursor(
    mut windows: Query<&mut CursorOptions>,
    mut commands: Commands,
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut controllers: Query<&mut PlayerController>,
    mut shoot_event_writer: MessageWriter<shoot::ShootEvent>,
    avian_pickup_actor: Single<Entity, With<AvianPickupActor>>,
    mut avian_pickup_input_writer: MessageWriter<AvianPickupInput>,
    active_windoow: Query<Entity, With<FloatingWindow>>,
) {
    if let Ok(mut window) = windows.single_mut() {
        if window.grab_mode != CursorGrabMode::Locked {
            if btn.just_pressed(MouseButton::Left) {
                if !active_windoow.is_empty() {
                    return;
                }
                window.grab_mode = CursorGrabMode::Locked;
                window.visible = false;
                for mut controller in &mut controllers {
                    controller.enable_input = true;
                }
            }
        } else if btn.just_pressed(MouseButton::Left) {
            avian_pickup_input_writer.write(AvianPickupInput { action: AvianPickupAction::Throw, actor: *avian_pickup_actor });
            shoot_event_writer.write(shoot::ShootEvent);
            commands.trigger(ShootEvent);
        }

        if key.just_pressed(KeyCode::Escape) {
            //window.grab_mode = CursorGrabMode::None;
            window.visible = true;
            for mut controller in &mut controllers {
                controller.enable_input = false;
            }
        }
    }
}

/*fn _weapon_select(
    action_query: Query<&ActionState<LeafwingAction>, With<Player>>,
) {
    if let Ok(action) = action_query.single() {
        if action.just_pressed(&LeafwingAction::Weapon1) {
            todo!();
        } else if action.just_pressed(&LeafwingAction::Weapon2) {
            todo!();
        } else if action.just_pressed(&LeafwingAction::Weapon3) {
            todo!();
        } else if action.just_pressed(&LeafwingAction::Weapon4) {
            todo!();
        }
    }
}*/
