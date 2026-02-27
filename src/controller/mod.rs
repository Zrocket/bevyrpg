mod basis;
mod equip_controller;
mod input;
mod interact_controller;
mod inventory_controller;
mod player_controller;
mod quest_controller;
mod stats_controller;

use bevy_enhanced_input::{action::Action, prelude::{ActionEvents, Fire}};
use equip_controller::*;
pub use input::*;
use avian_pickup::prelude::*;
use bevy::prelude::*;
use bevy_tnua::TnuaUserControlsSystems;
pub use interact_controller::*;
use inventory_controller::*;
pub use player_controller::*;
pub use quest_controller::*;
pub use stats_controller::*;

use bevy::window::{CursorGrabMode, CursorOptions};

use crate::{ActiveWeapon, ItemDetails, PlayerCamera, ShootEvent, level::DAGunAssets, shoot, widgets::floating_windows::FloatingWindow};

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
            .add_plugins(QuestControllerPlugin)
            .add_observer(weapon_1)
            .add_observer(weapon_2)
            .add_observer(weapon_3)
            .add_observer(weapon_4)
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
    key: Res<ButtonInput<KeyCode>>,
    shoot_action: Single<&ActionEvents, With<Action<ShootAction>>>,
    mut controllers: Query<&mut PlayerController>,
    mut shoot_event_writer: MessageWriter<shoot::ShootEvent>,
    avian_pickup_actor: Single<Entity, With<AvianPickupActor>>,
    mut avian_pickup_input_writer: MessageWriter<AvianPickupInput>,
    active_windoow: Query<Entity, With<FloatingWindow>>,
) {
    if let Ok(mut window) = windows.single_mut() {
        if window.grab_mode != CursorGrabMode::Locked {
                if shoot_action.contains(ActionEvents::FIRE) {
                if !active_windoow.is_empty() {
                    return;
                }
                window.grab_mode = CursorGrabMode::Locked;
                window.visible = false;
                for mut controller in &mut controllers {
                    controller.enable_input = true;
                }
            }
        } else if shoot_action.contains(ActionEvents::START) {
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

fn weapon_1(
    _trigger: On<Fire<Weapon1Action>>,
    mut commands: Commands,
    player_query: Query<Entity, With<PlayerCamera>>,
    gun_assets: Res<DAGunAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    asset_server: Res<AssetServer>,
    active_weapon_query: Query<Entity, With<ActiveWeapon>>,
) {
    if let Ok(player_entity) = player_query.single() {
        if let Ok(active_weapon) = active_weapon_query.single() {
            commands.entity(active_weapon).despawn();
        }
        let path = gltf_assets.get(&gun_assets.uzi).unwrap().scenes[0].path().unwrap();
        let gun = commands.spawn((
                Transform::from_translation(vec3(0.1, -0.2, -0.5)),
                SceneRoot(asset_server.load(path)),
                ItemDetails {
                    name: "Uzi".to_string(),
                    description: crate::Description("gun".to_string()),
                    weight: crate::Weight(0),
                },
                Name::new("uzi"),
                ActiveWeapon,
        ))
        .id();

        commands.entity(player_entity)
            .add_child(gun);
    }
}

fn weapon_2(
    _trigger: On<Fire<Weapon2Action>>,
    mut commands: Commands,
    player_query: Query<Entity, With<PlayerCamera>>,
    gun_assets: Res<DAGunAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    asset_server: Res<AssetServer>,
    active_weapon_query: Query<Entity, With<ActiveWeapon>>,
) {
    if let Ok(player_entity) = player_query.single() {
        if let Ok(active_weapon) = active_weapon_query.single() {
            commands.entity(active_weapon).despawn();
        }
        let path = gltf_assets.get(&gun_assets.shotgun).unwrap().scenes[0].path().unwrap();
        let gun = commands.spawn((
                Transform::from_translation(vec3(0.1, -0.2, -0.5)),
                SceneRoot(asset_server.load(path)),
                ItemDetails {
                    name: "Shotgun".to_string(),
                    description: crate::Description("gun".to_string()),
                    weight: crate::Weight(0),
                },
                Name::new("shotgun"),
                ActiveWeapon,
        ))
        .id();

        commands.entity(player_entity)
            .add_child(gun);
    }
}

fn weapon_3(
    _trigger: On<Fire<Weapon3Action>>,
    mut commands: Commands,
    player_query: Query<Entity, With<PlayerCamera>>,
    gun_assets: Res<DAGunAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    asset_server: Res<AssetServer>,
    active_weapon_query: Query<Entity, With<ActiveWeapon>>,
) {
    if let Ok(player_entity) = player_query.single() {
        if let Ok(active_weapon) = active_weapon_query.single() {
            commands.entity(active_weapon).despawn();
        }
        let path = gltf_assets.get(&gun_assets.sniper).unwrap().scenes[0].path().unwrap();
        let gun = commands.spawn((
                Transform::from_translation(vec3(0.1, -0.2, -0.5)),
                SceneRoot(asset_server.load(path)),
                ItemDetails {
                    name: "Sniper".to_string(),
                    description: crate::Description("gun".to_string()),
                    weight: crate::Weight(0),
                },
                Name::new("sniper"),
                ActiveWeapon,
        ))
        .id();

        commands.entity(player_entity)
            .add_child(gun);
    }
}

fn weapon_4(
    _trigger: On<Fire<Weapon4Action>>,
    mut commands: Commands,
    player_query: Query<Entity, With<PlayerCamera>>,
    gun_assets: Res<DAGunAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    asset_server: Res<AssetServer>,
    active_weapon_query: Query<Entity, With<ActiveWeapon>>,
) {
    if let Ok(player_entity) = player_query.single() {
        if let Ok(active_weapon) = active_weapon_query.single() {
            commands.entity(active_weapon).despawn();
        }
        let path = gltf_assets.get(&gun_assets.mp5).unwrap().scenes[0].path().unwrap();
        let gun = commands.spawn((
                Transform::from_translation(vec3(0.1, -0.2, -0.5)),
                SceneRoot(asset_server.load(path)),
                ItemDetails {
                    name: "MP5".to_string(),
                    description: crate::Description("gun".to_string()),
                    weight: crate::Weight(0),
                },
                Name::new("mp5"),
                ActiveWeapon,
        ))
        .id();

        commands.entity(player_entity)
            .add_child(gun);
    }
}
