use bevy::{input, prelude::*};
use bevy_enhanced_input::{action::{Action, InputAction}, binding::{Binding, mod_keys::ModKeys, relationship::Bindings}};
use itertools::Itertools;

use crate::{BindId, ConsoleBinding, CrouchAction, FlashlightAction, InputSystemParam, InteractAction, JumpAction, KeySlot, Keybinds, MenuState, OpenConsoleAction, OpenEquipAction, OpenInventoryAction, OpenQuestAction, OpenStatsAction, RunAction, widgets::{self, ui_root}};

#[derive(Component, Reflect)]
pub struct UiControllerSettings;

#[derive(Component, Reflect)]
pub struct UiKeyBindSettings;

pub struct ControllerSettingsMenuUiPlugin;
impl Plugin for ControllerSettingsMenuUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app
           .init_resource::<Rebinding>()
           .register_type::<UiControllerSettings>()
           .add_systems(OnEnter(MenuState::ControllerSettings), spawn_controller_settings_menu)
           .add_systems(OnEnter(MenuState::KeyBindings), spawn_key_bindings_menu)
           .add_systems(Update, capture_rebind.run_if(in_state(MenuState::KeyBindings)))
           .add_systems(OnExit(MenuState::KeyBindings), |mut r: ResMut<Rebinding>| r.0 = None);
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
                widgets::button("Key Bindings", enter_keybindings_settings_menu),
                widgets::button("Back", back_to_settings_menu),
            ],
    ));
}

fn enter_keybindings_settings_menu(
    _: On<Pointer<Click>>,
    mut pause_menu_state: ResMut<NextState<MenuState>>,
) {
    pause_menu_state.set(MenuState::KeyBindings);
}

fn back_to_settings_menu(
    _: On<Pointer<Click>>,
    mut pause_menu_state: ResMut<NextState<MenuState>>,
) {
    pause_menu_state.set(MenuState::Settings);
}

#[derive(Resource, Default)]
pub struct Rebinding(pub Option<(Entity, bool)>);

fn rebind_key<A: InputAction, const PAD: bool>(
    _: On<Pointer<Click>>,
    mut input_commands: InputSystemParam,
    mut actions_query: Query<Entity, With<Action<A>>>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    gamepads: Query<&Gamepad>,
    mut rebinding: ResMut<Rebinding>,
) {
    let Ok(entity) = actions_query.single() else { return };
    rebinding.0 = Some((entity, PAD));
}

fn capture_rebind(
    mut rebinding: ResMut<Rebinding>,
    mut input_commands: InputSystemParam,
    mut next_menu: ResMut<NextState<MenuState>>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    gamepads: Query<&Gamepad>,
) {
    let Some((action, slot)) = rebinding.0 else { return };

    if keys.just_pressed(KeyCode::Escape) {
        rebinding.0 = None;
        return;
    }

    let new_bind: Option<Binding> = match slot {
        false => keys.get_just_pressed().next().map(|k| Binding::from(*k))
            .or_else(|| mouse.get_just_pressed().next().map(|m| Binding::from((*m)))),
        true => gamepads.iter()
            .find_map(|g| g.get_just_pressed().next().copied())
            .map(Binding::from),
    };
    let Some(new_bind) = new_bind else { return; };

    input_commands.rebind(action, new_bind, slot);
    rebinding.0 = None;
    next_menu.set(MenuState::KeyBindings);
}

fn spawn_key_bindings_menu(
    mut commands: Commands,
    //keybind_database: Res<KeybindDatabase>,
    keybinds: Res<Keybinds>
) {
    /*let Some(weapon1_bind) = keybind_database.0.get("weapon1") else {
        error!("spawn_key_bindings_menu: Could not get weapon1 from KeybindDatabase");
        return;
    };
    let Some(weapon2_bind) = keybind_database.0.get("weapon2") else {
        error!("spawn_key_bindings_menu: Could not get weapon2 from KeybindDatabase");
        return;
    };
    let Some(weapon3_bind) = keybind_database.0.get("weapon3") else {
        error!("spawn_key_bindings_menu: Could not get weapon3 from KeybindDatabase");
        return;
    };
    let Some(weapon4_bind) = keybind_database.0.get("weapon4") else {
        error!("spawn_key_bindings_menu: Could not get weapon4 from KeybindDatabase");
        return;
    };
    let Some(jump_bind) = keybind_database.0.get("jump") else {
        error!("spawn_key_bindings_menu: Could not get jump from KeybindDatabase");
        return;
    };
    let Some(run_bind) = keybind_database.0.get("run") else {
        error!("spawn_key_bindings_menu: Could not get run from KeybindDatabase");
        return;
    };
    let Some(crouch_bind) = keybind_database.0.get("crouch") else {
        error!("spawn_key_bindings_menu: Could not get crouch from KeybindDatabase");
        return;
    };
    let Some(up_bind) = keybind_database.0.get("up") else {
        error!("spawn_key_bindings_menu: Could not get up from KeybindDatabase");
        return;
    };
    let Some(down_bind) = keybind_database.0.get("down") else {
        error!("spawn_key_bindings_menu: Could not get down from KeybindDatabase");
        return;
    };
    let Some(interact_bind) = keybind_database.0.get("interact") else {
        error!("spawn_key_bindings_menu: Could not get interact from KeybindDatabase");
        return;
    };
    let Some(interact2_bind) = keybind_database.0.get("interact2") else {
        error!("spawn_key_bindings_menu: Could not get interact2 from KeybindDatabase");
        return;
    };
    let Some(inventory_bind) = keybind_database.0.get("inventory") else {
        error!("spawn_key_bindings_menu: Could not get inventory from KeybindDatabase");
        return;
    };
    let Some(equip_bind) = keybind_database.0.get("equip") else {
        error!("spawn_key_bindings_menu: Could not get equip from KeybindDatabase");
        return;
    };
    let Some(stats_bind) = keybind_database.0.get("stats") else {
        error!("spawn_key_bindings_menu: Could not get stats from KeybindDatabase");
        return;
    };
    let Some(quest_bind) = keybind_database.0.get("quest") else {
        error!("spawn_key_bindings_menu: Could not get quest from KeybindDatabase");
        return;
    };
    let Some(console_bind) = keybind_database.0.get("console") else {
        error!("spawn_key_bindings_menu: Could not get console from KeybindDatabase");
        return;
    };
    let Some(flashlight_bind) = keybind_database.0.get("flashlight") else {
        error!("spawn_key_bindings_menu: Could not get flashlight from KeybindDatabase");
        return;
    };*/

    commands.spawn((
            ui_root("Key Bindings"),
            DespawnOnExit(MenuState::KeyBindings),
            GlobalZIndex(2),
            UiKeyBindSettings,
            children![
                /*(
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
                ),*/
                //widgets::button(format!("Run (key): {}", run_bind.key), rebind_key::<RunAction, false>),
                widgets::button(format!("Run (key): {}", keybinds.run_key), rebind_key::<RunAction, false>),
                //widgets::button(format!("Run (gamepad): {}", run_bind.gamepad), rebind_key::<RunAction, true>),
                widgets::button(format!("Run (gamepad): {}", keybinds.run_gamepad), rebind_key::<RunAction, true>),

                //widgets::button(format!("Jump (key): {}", jump_bind.key), rebind_key::<JumpAction, false>),
                widgets::button(format!("Jump (key): {}", keybinds.jump_key), rebind_key::<JumpAction, false>),
                //widgets::button(format!("Jump (gamepad): {}", jump_bind.gamepad), rebind_key::<JumpAction, true>),
                widgets::button(format!("Jump (gamepad): {}", keybinds.jump_gamepad), rebind_key::<JumpAction, true>),

                //widgets::button(format!("Crouch (key): {}", crouch_bind.key), rebind_key::<CrouchAction, false>),
                widgets::button(format!("Crouch (key): {}", keybinds.crouch_key), rebind_key::<CrouchAction, false>),
                //widgets::button(format!("Crouch (gamepad): {}", crouch_bind.gamepad), rebind_key::<CrouchAction, true>),
                widgets::button(format!("Crouch (gamepad): {}", keybinds.crouch_gamepad), rebind_key::<CrouchAction, true>),

                //widgets::button(format!("Flashlight (key): {}", flashlight_bind.key), rebind_key::<FlashlightAction, false>),
                widgets::button(format!("Flashlight (key): {}", keybinds.flashlight_key), rebind_key::<FlashlightAction, false>),
                //widgets::button(format!("Flashlight (gamepad): {}", flashlight_bind.gamepad), rebind_key::<FlashlightAction, true>),
                widgets::button(format!("Flashlight (gamepad): {}", keybinds.flashlight_gamepad), rebind_key::<FlashlightAction, true>),

                //widgets::button(format!("Interact (key): {}", interact_bind.key), rebind_key::<InteractAction, false>),
                widgets::button(format!("Interact (key): {}", keybinds.interact_key), rebind_key::<InteractAction, false>),
                //widgets::button(format!("Interact (gamepad): {}", interact_bind.gamepad), rebind_key::<InteractAction, true>),
                widgets::button(format!("Interact (gamepad): {}", keybinds.interact_gamepad), rebind_key::<InteractAction, true>),

                //widgets::button(format!("Inventory (key): {}", inventory_bind.key), rebind_key::<OpenInventoryAction, false>),
                widgets::button(format!("Inventory (key): {}", keybinds.inventory_key), rebind_key::<OpenInventoryAction, false>),
                //widgets::button(format!("Inventory (gamepad): {}", inventory_bind.gamepad), rebind_key::<OpenInventoryAction, true>),
                widgets::button(format!("Inventory (gamepad): {}", keybinds.inventory_gamepad), rebind_key::<OpenInventoryAction, true>),

                //widgets::button(format!("Equip (key): {}", equip_bind.key), rebind_key::<OpenEquipAction, false>),
                widgets::button(format!("Equip (key): {}", keybinds.equip_key), rebind_key::<OpenEquipAction, false>),
                //widgets::button(format!("Equip (gamepad): {}", equip_bind.gamepad), rebind_key::<OpenEquipAction, true>),
                widgets::button(format!("Equip (gamepad): {}", keybinds.equip_gamepad), rebind_key::<OpenEquipAction, true>),

                //widgets::button(format!("Stats (key): {}", stats_bind.key), rebind_key::<OpenStatsAction, false>),
                widgets::button(format!("Stats (key): {}", keybinds.stats_key), rebind_key::<OpenStatsAction, false>),
                //widgets::button(format!("Stats (gamepad): {}", stats_bind.gamepad), rebind_key::<OpenStatsAction, true>),
                widgets::button(format!("Stats (gamepad): {}", keybinds.stats_gamepad), rebind_key::<OpenStatsAction, true>),

                //widgets::button(format!("Quest (key): {}", quest_bind.key), rebind_key::<OpenQuestAction, false>),
                widgets::button(format!("Quest (key): {}", keybinds.quest_key), rebind_key::<OpenQuestAction, false>),
                //widgets::button(format!("Quest (gamepad): {}", quest_bind.gamepad), rebind_key::<OpenQuestAction, true>),
                widgets::button(format!("Quest (gamepad): {}", keybinds.quest_gamepad), rebind_key::<OpenQuestAction, true>),

                //widgets::button(format!("Console (key): {}", console_bind.key), rebind_key::<OpenConsoleAction, false>),
                widgets::button(format!("Console (key): {}", keybinds.console_key), rebind_key::<OpenConsoleAction, false>),

                widgets::button("Back", back_to_controller_settings_menu),
            ],
        ));
}

fn back_to_controller_settings_menu(
    _: On<Pointer<Click>>,
    mut pause_menu_state: ResMut<NextState<MenuState>>,
) {
    pause_menu_state.set(MenuState::ControllerSettings);
}
