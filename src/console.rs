use avian3d::prelude::{Collider, GravityScale};
use bevy::prelude::*;
use bevy_flycam::FlyCam;
use bevy_landmass::debug::EnableLandmassDebug;
use chill_bevy_console::{ChillConsole, CommandArgs, ConsoleAppExt};

use crate::{AddToInventoryEvent, DamageEvent, DeathEvent, Equiptable, GodMode, Health, ItemDetails, Player, PlayerCamera, PlayerState, level::ChangeLevelMessage};

pub struct MyConsolePlugin;
impl Plugin for MyConsolePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ChillConsole::default());
            //.add_console_command("level", "level <file>", level_command)
            //.add_console_command("gravity", "gravity <value>", gravity_command)
            //.add_console_command("suicide", "suicide", suicide_command)
            //.add_console_command("god", "god", god_command)
            //.add_console_command("noclip", "noclip", noclip_command)
            //.add_console_command("navmesh", "navmesh", navmesh_command)
            //.add_console_command("damage", "damage", damage_command)
            //.add_console_command("inventory", "inventory", inventory_command)
            //.add_console_command("mana", "mana", mana_command);
    }
}

fn gravity_command(
    In(args): CommandArgs,
    mut player_gravity_query: Query<&mut GravityScale, With<Player>>,
) -> String {
    if let Ok(mut player_gravity) = player_gravity_query.single_mut()
    && let Ok(value) = args[0].parse::<f32>() {
        player_gravity.0 = value;
        return "ok".to_string();
    }
    "fail".to_string()
}

fn level_command(
    In(args): CommandArgs,
    mut change_level_message_writer: MessageWriter<ChangeLevelMessage>,
) -> String {
    change_level_message_writer.write(ChangeLevelMessage(args[0].clone()));
    "ok".to_string()
}

fn suicide_command(
    In(_args): CommandArgs,
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Health), With<Player>>,
) -> String {
    if let Ok((player_entity, mut health)) = player_query.single_mut() {
        health.0 = 0;
        commands.entity(player_entity).trigger(|entity| DeathEvent { entity });
    }
    "ok".to_string()
}

fn noclip_command(
    In(_args): CommandArgs,
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut PlayerState), With<Player>>,
    player_camera_query: Query<Entity, With<PlayerCamera>>,
) -> String {
    if let Ok((player_entity, mut player_state)) = player_query.single_mut()
    && let Ok(player_camera) = player_camera_query.single() {
        if *player_state == PlayerState::NoClip {
            commands.entity(player_camera).remove::<FlyCam>();
            commands.entity(player_entity).insert(Collider::capsule(0.1, 0.5));
            *player_state = PlayerState::Grounded;
        } else {
            commands.entity(player_entity).remove::<Collider>();
            commands.entity(player_camera).insert(FlyCam);
            *player_state = PlayerState::NoClip;
        }
    }
    "ok".to_string()
}

fn god_command(
    In(_args): CommandArgs,
    mut commands: Commands,
    player_query: Query<(Entity, Option<&GodMode>), With<Player>>,
) -> String {
    if let Ok((player_entity, player_godmode)) = player_query.single() {
        if player_godmode.is_some() {
            commands.entity(player_entity).remove::<GodMode>();
        } else {
            commands.entity(player_entity).insert(GodMode);
        }
    }
    "ok".to_string()
}

fn navmesh_command(
    In(_args): CommandArgs,
    mut navmesh_debug: ResMut<EnableLandmassDebug>
) -> String {
    **navmesh_debug = !**navmesh_debug;
    "ok".to_string()
}

fn kill_command(
    In(args): CommandArgs,
) -> String {
    "ok".to_string()
}

fn spawn_command(
    In(args): CommandArgs,
) -> String {
    "ok".to_string()
}

fn damage_command(
    In(_args): CommandArgs,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>
) -> String {
    if let Ok(player_entity) = player_query.single() {
        commands.entity(player_entity).trigger(|entity| DamageEvent { entity, ammount: 5 });
    }
    "ok".to_string()
}

fn inventory_command(
    In(_args): CommandArgs,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>
) -> String {
    if let Ok(player_entity) = player_query.single() {
        let item = commands.spawn((
                ItemDetails {
                    name: "Test".to_string(),
                    description: crate::Description("Test".to_string()),
                    weight: crate::Weight(0),
                },
                Name::new(format!("Test {}", rand::random::<u8>() as char)),
                Equiptable {
                    slot: crate::EquipSlot::Arm,
                    defense: 1,
                }
        )).id();
        println!("{:?}", item);
        commands.entity(player_entity).trigger(|entity| AddToInventoryEvent { entity, item });
    }
    "ok".to_string()
}

fn mana_command(
    In(_args): CommandArgs,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) -> String {
    if let Ok(player_entity) = player_query.single() {
        commands.entity(player_entity).trigger(|entity| crate::ManaEvent { entity, ammount: 5 });
    }
    "ok".to_string()
}
