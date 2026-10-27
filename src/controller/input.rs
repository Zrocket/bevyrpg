use std::{collections::HashMap, f32::consts};

use avian3d::prelude::{Collider, RigidBody};
use bevy::{app::Plugin, asset::{self, Assets}, ecs::{component::Component, lifecycle::HookContext, spawn::SpawnRelated, system::SystemParam, world::DeferredWorld}, input::{gamepad::GamepadButton, keyboard::{Key::Control, KeyCode}, mouse::MouseButton}, math::Vec2, prelude::*, reflect::TypePath, settings::{ReflectSettingsGroup, SaveSettings, SettingsGroup, SettingsPlugin}, time::Timer, utils::{default, define_atomic_id}};
use bevy_asset_loader::{
    prelude::*,
    asset_collection::AssetCollection,
    loading_state::{
        LoadingState,
        LoadingStateAppExt,
        config::{
            ConfigureLoadingState,
            LoadingStateConfig
        }
    }
};
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_enhanced_input::{action::Action, actions, binding::{Binding, mod_keys::ModKeys}, bindings, modifier::scale::Scale, prelude::{Axial, Bindings, Cardinal, DeadZone, EnhancedInputPlugin, Hold, InputAction, InputContextAppExt, SmoothNudge, Tap}};
use bevy_tnua::{TnuaConfig, TnuaController, builtins::{TnuaBuiltinClimbConfig, TnuaBuiltinCrouchConfig, TnuaBuiltinDashConfig, TnuaBuiltinJumpConfig, TnuaBuiltinWalkConfig, TnuaBuiltinWallSlideConfig}, control_helpers::TnuaSimpleAirActionsCounter};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
//use bevy_bae::{plan::Plan, prelude::{Operator, Sequence}, tasks};
use bevy_tnua_physics_integration_layer::math::float_consts;
use chill_bevy_console::BuiltinCommand::Bind;
use serde::{Deserialize, Serialize};

use crate::{IdleTimer, Player, PlayerControlScheme, PlayerControlSchemeConfig, Walk, /*idle, run_from_player, wander*/};

#[derive(Resource, SettingsGroup, Reflect, Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
#[reflect(Resource, SettingsGroup, Default)]
pub struct Keybinds {
    pub weapon1_key: Binding,
    pub weapon1_gamepad: Binding,
    pub weapon2_key: Binding,
    pub weapon2_gamepad: Binding,
    pub weapon3_key: Binding,
    pub weapon3_gamepad: Binding,
    pub weapon4_key: Binding,
    pub weapon4_gamepad: Binding,
    pub jump_key: Binding,
    pub jump_gamepad: Binding,
    pub run_key: Binding,
    pub run_gamepad: Binding,
    pub crouch_key: Binding,
    pub crouch_gamepad: Binding,
    pub up_key: Binding,
    pub up_gamepad: Binding,
    pub down_key: Binding,
    pub down_gamepad: Binding,
    pub interact_key: Binding,
    pub interact_gamepad: Binding,
    pub interact2_key: Binding,
    pub interact2_gamepad: Binding,
    pub inventory_key: Binding,
    pub inventory_gamepad: Binding,
    pub equip_key: Binding,
    pub equip_gamepad: Binding,
    pub stats_key: Binding,
    pub stats_gamepad: Binding,
    pub quest_key: Binding,
    pub quest_gamepad: Binding,
    pub console_key: Binding,
    pub console_gamepad: Binding,
    pub flashlight_key: Binding,
    pub flashlight_gamepad: Binding,
    pub forward_key: Binding,
    pub forward_gamepad: Binding,
    pub backward_key: Binding,
    pub backward_gamepad: Binding,
    pub left_key: Binding,
    pub left_gamepad: Binding,
    pub right_key: Binding,
    pub right_gamepad: Binding,
}

impl Default for Keybinds {
    fn default() -> Self {
        use KeyCode::*;
        use GamepadButton as G;
        let pad = Binding::GamepadButton;

        Self {
            weapon1_key: Digit1.into(),
            weapon1_gamepad: pad(G::DPadLeft),
            weapon2_key: Digit2.into(),
            weapon2_gamepad: pad(G::DPadUp),
            weapon3_key: Digit3.into(),
            weapon3_gamepad: pad(G::DPadRight),
            weapon4_key: Digit4.into(),
            weapon4_gamepad: pad(G::DPadDown),
            jump_key: Space.into(),
            jump_gamepad: pad(G::South),
            run_key: ShiftLeft.into(),
            run_gamepad: pad(G::LeftThumb),
            crouch_key: ControlLeft.into(),
            crouch_gamepad: pad(G::LeftThumb),
            up_key: KeyZ.into(),
            up_gamepad: Binding::None,
            down_key: KeyX.into(),
            down_gamepad: Binding::None,
            interact_key: KeyE.into(),
            interact_gamepad: pad(G::West),
            interact2_key: KeyE.into(),
            interact2_gamepad: pad(G::West),
            inventory_key: KeyI.into(),
            inventory_gamepad: pad(G::Select),
            equip_key: KeyK.into(),
            equip_gamepad: pad(G::Start),
            stats_key: KeyL.into(),
            stats_gamepad: Binding::None,
            quest_key: KeyH.into(),
            quest_gamepad: Binding::None,
            console_key: Backslash.into(),
            console_gamepad: Binding::None,
            flashlight_key: KeyF.into(),
            flashlight_gamepad: pad(G::East),
            backward_key: KeyS.into(),
            backward_gamepad: Binding::None,
            forward_key: KeyW.into(),
            forward_gamepad: Binding::None,
            left_key: KeyA.into(),
            left_gamepad: Binding::None,
            right_key: KeyD.into(),
            right_gamepad: Binding::None,
        }
    }
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_plugins(EnhancedInputPlugin)
            //.add_plugins(RonAssetPlugin::<KeybindDefinitions>::new(&["k.ron"]))
            .add_plugins(SettingsPlugin::new("com.github.zrocket.bevyrpg"))
            .add_input_context::<Player>();
            /*.configure_loading_state(
                LoadingStateConfig::new(crate::BootStrap::Preload)
                .load_collection::<KeybindDefinitionsHandle>()
                .finally_init_resource::<KeybindDatabase>()
            );*/
    }
}

#[derive(SystemParam)]
pub struct InputSystemParam<'w, 's> {
    commands: Commands<'w, 's>,
    //keybind_database: ResMut<'w, KeybindDatabase>,
    keybinds: ResMut<'w, Keybinds>,
    //bindings_query: Query<'w, 's, (&'static mut Bindings, &'static BindId)>,
    bindings_query: Query<'w, 's, (&'static mut Bindings, &'static BindId2)>,
}

impl <'w, 's> InputSystemParam<'w, 's> {
    pub fn rebind(&mut self, bindings_entity: Entity, new_bind: Binding, gamepad: bool) {
        let Ok((bindings, bind_id)) = self.bindings_query.get_mut(bindings_entity) else {
            error!("rebind: Failed to get Bindings and BindId for entity {}", bindings_entity);
            return;
        };

        match bind_id {
            BindId2::Weapon1 => {
                if gamepad {
                    self.keybinds.weapon1_gamepad = new_bind;
                } else {
                    self.keybinds.weapon1_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.weapon1_key, KeySlot)),
                                Spawn((self.keybinds.weapon1_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Weapon2 => {
                if gamepad {
                    self.keybinds.weapon2_gamepad = new_bind;
                } else {
                    self.keybinds.weapon2_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.weapon2_key, KeySlot)),
                                Spawn((self.keybinds.weapon2_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Weapon3 => {
                if gamepad {
                    self.keybinds.weapon3_gamepad = new_bind;
                } else {
                    self.keybinds.weapon3_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.weapon3_key, KeySlot)),
                                Spawn((self.keybinds.weapon3_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Weapon4 => {
                if gamepad {
                    self.keybinds.weapon4_gamepad = new_bind;
                } else {
                    self.keybinds.weapon4_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.weapon4_key, KeySlot)),
                                Spawn((self.keybinds.weapon4_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Jump => {
                if gamepad {
                    self.keybinds.jump_gamepad = new_bind;
                } else {
                    self.keybinds.jump_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.jump_key, KeySlot)),
                                Spawn((self.keybinds.jump_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Run => {
                if gamepad {
                    self.keybinds.run_gamepad = new_bind;
                } else {
                    self.keybinds.run_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.run_key, KeySlot)),
                                Spawn((self.keybinds.run_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Crouch => {
                if gamepad {
                    self.keybinds.crouch_gamepad = new_bind;
                } else {
                    self.keybinds.crouch_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.crouch_key, KeySlot)),
                                Spawn((self.keybinds.crouch_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Up => {
                if gamepad {
                    self.keybinds.up_gamepad = new_bind;
                } else {
                    self.keybinds.up_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.up_key, KeySlot)),
                                Spawn((self.keybinds.up_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Down => {
                if gamepad {
                    self.keybinds.down_gamepad = new_bind;
                } else {
                    self.keybinds.down_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.down_key, KeySlot)),
                                Spawn((self.keybinds.down_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Interact => {
                if gamepad {
                    self.keybinds.interact_gamepad = new_bind;
                } else {
                    self.keybinds.interact_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.interact_key, KeySlot)),
                                Spawn((self.keybinds.interact_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Interact2 => {
                if gamepad {
                    self.keybinds.interact2_gamepad = new_bind;
                } else {
                    self.keybinds.interact2_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.interact2_key, KeySlot)),
                                Spawn((self.keybinds.interact2_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Inventory => {
                if gamepad {
                    self.keybinds.inventory_gamepad = new_bind;
                } else {
                    self.keybinds.inventory_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.inventory_key, KeySlot)),
                                Spawn((self.keybinds.inventory_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Equip => {
                if gamepad {
                    self.keybinds.equip_gamepad = new_bind;
                } else {
                    self.keybinds.equip_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.equip_key, KeySlot)),
                                Spawn((self.keybinds.equip_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Stats => {
                if gamepad {
                    self.keybinds.stats_gamepad = new_bind;
                } else {
                    self.keybinds.stats_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.stats_key, KeySlot)),
                                Spawn((self.keybinds.stats_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Quest => {
                if gamepad {
                    self.keybinds.quest_gamepad = new_bind;
                } else {
                    self.keybinds.stats_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.quest_key, KeySlot)),
                                Spawn((self.keybinds.quest_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Console => {
                if gamepad {
                    self.keybinds.console_gamepad = new_bind;
                } else {
                    self.keybinds.console_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.console_key, KeySlot)),
                                Spawn((self.keybinds.console_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Flashlight => {
                if gamepad {
                    self.keybinds.flashlight_gamepad = new_bind;
                } else {
                    self.keybinds.flashlight_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.flashlight_key, KeySlot)),
                                Spawn((self.keybinds.console_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Forward => {
                if gamepad {
                    self.keybinds.forward_gamepad = new_bind;
                } else {
                    self.keybinds.forward_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.forward_key, KeySlot)),
                                Spawn((self.keybinds.forward_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Backward => {
                if gamepad {
                    self.keybinds.backward_gamepad = new_bind;
                } else {
                    self.keybinds.backward_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.backward_key, KeySlot)),
                                Spawn((self.keybinds.backward_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Left => {
                if gamepad {
                    self.keybinds.left_gamepad = new_bind;
                } else {
                    self.keybinds.left_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.left_key, KeySlot)),
                                Spawn((self.keybinds.left_gamepad, ControllerSlot)),
                        )),
                ));
            },
            BindId2::Right => {
                if gamepad {
                    self.keybinds.right_gamepad = new_bind;
                } else {
                    self.keybinds.right_key = new_bind;
                }
                self.commands.entity(bindings_entity).insert((
                        Bindings::spawn((
                                Spawn((self.keybinds.right_key, KeySlot)),
                                Spawn((self.keybinds.right_gamepad, ControllerSlot)),
                        )),
                ));
            },
        }

        self.commands.queue(SaveSettings::IfChanged);
    }
}

#[derive(Component)]
pub struct BindId(pub String);

#[derive(Component)]
pub enum BindId2 {
    Weapon1,
    Weapon2,
    Weapon3,
    Weapon4,
    Jump,
    Run,
    Crouch,
    Up,
    Down,
    Interact,
    Interact2,
    Inventory,
    Equip,
    Stats,
    Quest,
    Console,
    Flashlight,
    Forward,
    Backward,
    Left,
    Right,
}

#[derive(Component)]
pub struct KeySlot;

#[derive(Component)]
pub struct ControllerSlot;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct MovementAction;

#[derive(Component)]
pub struct MovementBinding;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct LookAction;

#[derive(Component)]
pub struct LookBinding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Weapon1Action;

#[derive(Component)]
pub struct Weapon1Binding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Weapon2Action;

#[derive(Component)]
pub struct Weapon2Binding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Weapon3Action;

#[derive(Component)]
pub struct Weapon3Binding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Weapon4Action;

#[derive(Component)]
pub struct Weapon4Binding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct RunAction;

#[derive(Component)]
pub struct RunBinding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct JumpAction;

#[derive(Component)]
pub struct JumpBinding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct CrouchAction;

#[derive(Component)]
pub struct CrouchBinding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct UpAction;

#[derive(Component)]
pub struct UpBinding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct DownAction;

#[derive(Component)]
pub struct DownBinding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct InteractAction;

#[derive(Component)]
pub struct InteractBinding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Interact2Action;

#[derive(Component)]
pub struct Interact2Binding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct OpenInventoryAction;

#[derive(Component)]
pub struct InventoryBinding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct OpenEquipAction;

#[derive(Component)]
pub struct EquipBinding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct OpenStatsAction;

#[derive(Component)]
pub struct StatsBinding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct OpenConsoleAction;

#[derive(Component)]
pub struct ConsoleBinding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct OpenQuestAction;

#[derive(Component)]
pub struct QuestBinding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct FlashlightAction;

#[derive(Component)]
pub struct FlashlightBinding;

#[derive(InputAction)]
#[action_output(bool)]
pub struct ShootAction;

#[derive(Component)]
pub struct ShootBinding;

#[derive(Component)]
#[component(on_add = on_tnua_npc_controller_add)]
#[require(
    TnuaController::<PlayerControlScheme>::default(),
    TnuaAvian3dSensorShape(Collider::cuboid(0.5, 0.5, 0.5)),
    TnuaSimpleAirActionsCounter::<PlayerControlScheme>::default(),
    RigidBody::Dynamic,
    Walk::default(),
    //Plan::new(),
)]
pub struct TnuaNpcController;

#[derive(Component, Default)]
#[component(on_add = on_tnua_enemy_controller_add)]
#[require(
    TnuaController::<PlayerControlScheme>::default(),
    TnuaAvian3dSensorShape(Collider::cuboid(0.5, 0.5, 0.5)),
    TnuaSimpleAirActionsCounter::<PlayerControlScheme>::default(),
    RigidBody::Dynamic,
    Walk::default(),
    //Plan::new(),
)]
pub struct TnuaEnemyController;

#[derive(Component, Default)]
#[require(
    TnuaController::<PlayerControlScheme>::default(),
    TnuaAvian3dSensorShape(Collider::cuboid(0.5, 0.5, 0.5)),
    TnuaSimpleAirActionsCounter::<PlayerControlScheme>::default(),
    RigidBody::Dynamic,
    Walk::default(),
)]
#[component(on_add = on_tnua_rover_controller_add)]
pub struct TnuaRoverController;

#[derive(Component, Default)]
#[require(
    TnuaController::<PlayerControlScheme>::default(),
    TnuaAvian3dSensorShape(Collider::cuboid(0.5, 0.5, 0.5)),
    TnuaSimpleAirActionsCounter::<PlayerControlScheme>::default(),
    RigidBody::Static,
    Walk::default(),
)]
#[component(on_add = on_tnua_rover_controller_add)]
pub struct TnuaCctvController;

#[derive(Component, Default)]
#[require(
)]
#[component(on_add = on_tnua_player_controller_add)]
pub struct TnuaPlayerController;

fn on_tnua_player_controller_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let keybinds = world.resource::<Keybinds>();

    let bei_input_map = actions!(Player[
        (
            Action::<MovementAction>::new(),
            DeadZone::default(),
            //SmoothNudge::new(30.0),
            Bindings::spawn((
                Cardinal::wasd_keys(),
                Axial::left_stick(),
            )),
        ),
        (
            Action::<LookAction>::new(),
            DeadZone::default(),
            //SmoothNudge::new(10.0),
            SmoothNudge::default(),
            Scale::splat(0.08),
            Bindings::spawn((
                    //Spawn(Binding::mouse_motion()),
                    Axial::right_stick(),
            )),
        ),
        (
            Action::<Weapon1Action>::new(),
            Bindings::spawn((
                    Spawn((keybinds.weapon1_key, Weapon1Binding, KeySlot)),
                    Spawn((keybinds.weapon1_gamepad, Weapon1Binding, ControllerSlot)),
            )),
            BindId2::Weapon1,
        ),
        (
            Action::<Weapon2Action>::new(),
            Bindings::spawn((
                    Spawn((keybinds.weapon2_key, Weapon2Binding, KeySlot)),
                    Spawn((keybinds.weapon2_gamepad, Weapon2Binding, ControllerSlot)),
            )),
            BindId2::Weapon2,
        ),
        (
            Action::<Weapon3Action>::new(),
            Bindings::spawn((
                    Spawn((keybinds.weapon3_key, Weapon3Binding, KeySlot)),
                    Spawn((keybinds.weapon3_gamepad, Weapon3Binding, ControllerSlot)),
            )),
            BindId2::Weapon3,
        ),
        (
            Action::<Weapon4Action>::new(),
            Bindings::spawn((
                    Spawn((keybinds.weapon4_key, Weapon4Binding, KeySlot)),
                    Spawn((keybinds.weapon4_gamepad, Weapon4Binding, ControllerSlot)),
            )),
            BindId2::Weapon4,
        ),
        (
            Action::<JumpAction>::new(),
            Bindings::spawn((
                    Spawn((keybinds.jump_key, JumpBinding, KeySlot)),
                    Spawn((keybinds.jump_gamepad, JumpBinding, ControllerSlot)),
            )),
            BindId2::Jump,
        ),
        (
            Action::<RunAction>::new(),
            Bindings::spawn((
                    Spawn((keybinds.run_key, RunBinding, KeySlot)),
                    Spawn((keybinds.run_gamepad, RunBinding, ControllerSlot)),
            )),
            BindId2::Run,
        ),
        (
            Action::<CrouchAction>::new(),
            Bindings::spawn((
                    Spawn((keybinds.crouch_key, CrouchBinding, KeySlot)),
                    Spawn((keybinds.crouch_gamepad, CrouchBinding, ControllerSlot)),
            )),
            BindId2::Crouch,
        ),
        (
            Action::<UpAction>::new(),
            Bindings::spawn((
                    Spawn((keybinds.up_key, UpBinding, KeySlot)),
                    Spawn((keybinds.up_gamepad, UpBinding, ControllerSlot)),
            )),
            BindId2::Up,
        ),
        (
            Action::<DownAction>::new(),
            Bindings::spawn((
                    Spawn((keybinds.down_key, DownBinding, KeySlot)),
                    Spawn((keybinds.down_gamepad, DownBinding, ControllerSlot)),
            )),
            BindId2::Down,
        ),
        (
            Action::<InteractAction>::new(),
            Tap::new(0.3),
            Bindings::spawn((
                    Spawn((keybinds.interact_key, InteractBinding, KeySlot)),
                    Spawn((keybinds.interact_gamepad, InteractBinding, ControllerSlot)),
            )),
            BindId2::Interact,
        ),
        (
            Action::<Interact2Action>::new(),
            Hold::new(0.5).one_shot(true),
            Bindings::spawn((
                    Spawn((keybinds.interact2_key, Interact2Binding, KeySlot)),
                    Spawn((keybinds.interact2_gamepad, Interact2Binding, ControllerSlot)),
            )),
            BindId2::Interact2,
        ),
        (
            Action::<OpenInventoryAction>::new(),
            Bindings::spawn((
                    Spawn((keybinds.inventory_key, InventoryBinding, KeySlot)),
                    Spawn((keybinds.inventory_gamepad, InventoryBinding, ControllerSlot)),
            )),
            BindId2::Inventory,
        ),
        (
            Action::<OpenEquipAction>::new(),
            Bindings::spawn((
                    Spawn((keybinds.equip_key, EquipBinding, KeySlot)),
                    Spawn((keybinds.equip_gamepad, EquipBinding, ControllerSlot)),
            )),
            BindId2::Equip,
        ),
        (
            Action::<OpenStatsAction>::new(),
            Bindings::spawn((
                    Spawn((keybinds.stats_key, StatsBinding, KeySlot)),
                    Spawn((keybinds.stats_gamepad, StatsBinding, ControllerSlot)),
            )),
            BindId2::Stats,
        ),
        (
            Action::<OpenQuestAction>::new(),
            Bindings::spawn((
                    Spawn((keybinds.quest_key, QuestBinding, KeySlot)),
                    Spawn((keybinds.quest_gamepad, QuestBinding, ControllerSlot)),
            )),
            BindId2::Quest,
        ),
        (
            Action::<OpenConsoleAction>::new(),
            Bindings::spawn((
                    Spawn((keybinds.console_key, ConsoleBinding, KeySlot)),
                    Spawn((keybinds.console_gamepad, ConsoleBinding, ControllerSlot)),
            )),
            BindId2::Console,
        ),
        (
            Action::<FlashlightAction>::new(),
            Bindings::spawn((
                    Spawn((keybinds.flashlight_key, FlashlightBinding, KeySlot)),
                    Spawn((keybinds.flashlight_gamepad, FlashlightBinding, ControllerSlot)),
            )),
            BindId2::Flashlight,
        ),
        (
            Action::<ShootAction>::new(),
            Bindings::spawn((
                    Spawn(Binding::MouseButton {
                        button: MouseButton::Left,
                        mod_keys: ModKeys::default(),
                    }),
                    Spawn(Binding::GamepadButton(GamepadButton::RightTrigger2)),
            )),
        ),
    ]);

    let mut control_scheme_configs = world.resource_mut::<Assets<PlayerControlSchemeConfig>>();
    let handle = control_scheme_configs.add(PlayerControlSchemeConfig {
        basis: TnuaBuiltinWalkConfig {
            speed: 10.0,
            float_height: 1.5,
            max_slope: float_consts::FRAC_PI_8,
            ..default()
        },
        jump: TnuaBuiltinJumpConfig {
            height: 2.0,
            ..default()
        },
        crouch: TnuaBuiltinCrouchConfig {
            float_offset: -0.7,
            ..default()
        },
        dash: TnuaBuiltinDashConfig {
            horizontal_distance: 5.0,
            ..default()
        },
        knockback: Default::default(),
        wall_slide: TnuaBuiltinWallSlideConfig {
            maintain_distance: Some(0.7),
            ..default()
        },
        wall_jump: TnuaBuiltinJumpConfig {
            height: 4.0,
            takeoff_extra_gravity: 90.0, // 3 times the default
            takeoff_above_velocity: 0.0,
            horizontal_distance: 2.0,
            ..default()
        },
        climb: TnuaBuiltinClimbConfig {
            climb_speed: 10.0,
            ..default()
        },
    });
    //let handle = control_scheme_configs.add(PlayerControlSchemeConfig::default());

    world.commands()
        .entity(context.entity)
        .insert((
                TnuaConfig::<PlayerControlScheme>(handle),
                bei_input_map,
        ));
}

fn on_tnua_npc_controller_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let mut control_scheme_configs = world.resource_mut::<Assets<PlayerControlSchemeConfig>>();
    let handle = control_scheme_configs.add(PlayerControlSchemeConfig {
        basis: TnuaBuiltinWalkConfig {
            speed: 10.0,
            float_height: 1.5,
            ..default()
        },
        jump: TnuaBuiltinJumpConfig {
            height: 2.0,
            ..default()
        },
        crouch: TnuaBuiltinCrouchConfig {
            float_offset: -0.7,
            ..default()
        },
        dash: TnuaBuiltinDashConfig {
            horizontal_distance: 5.0,
            ..default()
        },
        knockback: Default::default(),
        wall_slide: TnuaBuiltinWallSlideConfig {
            maintain_distance: Some(0.7),
            ..default()
        },
        wall_jump: TnuaBuiltinJumpConfig {
            height: 4.0,
            takeoff_extra_gravity: 90.0, // 3 times the default
            takeoff_above_velocity: 0.0,
            horizontal_distance: 2.0,
            ..default()
        },
        climb: TnuaBuiltinClimbConfig {
            climb_speed: 10.0,
            ..default()
        },
    });


    /*world.commands()
        .entity(context.entity)
        .insert((
                TnuaConfig::<PlayerControlScheme>(handle),
        ))
        .insert((
            Sequence,
            tasks![
                Operator::new(wander),
                Operator::new(idle),
            ],
            IdleTimer(Timer::from_seconds(5.0, bevy::time::TimerMode::Once))
        ));*/
}

fn on_tnua_enemy_controller_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let mut control_scheme_configs = world.resource_mut::<Assets<PlayerControlSchemeConfig>>();
    let handle = control_scheme_configs.add(PlayerControlSchemeConfig {
        basis: TnuaBuiltinWalkConfig {
            speed: 10.0,
            float_height: 1.5,
            ..default()
        },
        jump: TnuaBuiltinJumpConfig {
            height: 2.0,
            ..default()
        },
        crouch: TnuaBuiltinCrouchConfig {
            float_offset: -0.7,
            ..default()
        },
        dash: TnuaBuiltinDashConfig {
            horizontal_distance: 5.0,
            ..default()
        },
        knockback: Default::default(),
        wall_slide: TnuaBuiltinWallSlideConfig {
            maintain_distance: Some(0.7),
            ..default()
        },
        wall_jump: TnuaBuiltinJumpConfig {
            height: 4.0,
            takeoff_extra_gravity: 90.0, // 3 times the default
            takeoff_above_velocity: 0.0,
            horizontal_distance: 2.0,
            ..default()
        },
        climb: TnuaBuiltinClimbConfig {
            climb_speed: 10.0,
            ..default()
        },
    });

    /*world.commands()
        .entity(context.entity)
        .insert((
                TnuaConfig::<PlayerControlScheme>(handle),
        ))
        .insert((
            Sequence,
            tasks![
                //Operator::new(wander),
                Operator::new(idle),
                Operator::new(run_from_player),
            ],
            IdleTimer(Timer::from_seconds(5.0, bevy::time::TimerMode::Once))
        ));*/
}

fn on_tnua_rover_controller_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let mut control_scheme_configs = world.resource_mut::<Assets<PlayerControlSchemeConfig>>();
    let handle = control_scheme_configs.add(PlayerControlSchemeConfig {
        basis: TnuaBuiltinWalkConfig {
            speed: 10.0,
            float_height: 0.5,
            //tilt_offset_angvel: 1.0,
            //tilt_offset_angacl: 1.0,
            ..default()
        },
        jump: TnuaBuiltinJumpConfig {
            height: 2.0,
            ..default()
        },
        crouch: TnuaBuiltinCrouchConfig {
            float_offset: -0.7,
            ..default()
        },
        dash: TnuaBuiltinDashConfig {
            horizontal_distance: 5.0,
            ..default()
        },
        knockback: Default::default(),
        wall_slide: TnuaBuiltinWallSlideConfig {
            maintain_distance: Some(0.7),
            ..default()
        },
        wall_jump: TnuaBuiltinJumpConfig {
            height: 4.0,
            takeoff_extra_gravity: 90.0, // 3 times the default
            takeoff_above_velocity: 0.0,
            horizontal_distance: 2.0,
            ..default()
        },
        climb: TnuaBuiltinClimbConfig {
            climb_speed: 10.0,
            ..default()
        },
    });

    world.commands()
        .entity(context.entity)
        .insert((
                TnuaConfig::<PlayerControlScheme>(handle),
        ));
}
