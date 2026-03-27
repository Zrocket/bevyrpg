use avian3d::prelude::GravityScale;
use bevy::prelude::*;
use chill_bevy_console::{ChillConsole, CommandArgs, ConsoleAppExt};

use crate::{Player, level::ChangeLevelMessage};

pub struct MyConsolePlugin;
impl Plugin for MyConsolePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ChillConsole::default())
            .add_console_command("level", "level <file>", level_command)
            .add_console_command("gravity", "gravity <value>", gravity_command);
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
    In(args): CommandArgs,
) -> String {
    "ok".to_string()
}

fn noclip_command(
    In(args): CommandArgs,
) -> String {
    "ok".to_string()
}

fn god_command(
    In(args): CommandArgs,
) -> String {
    "ok".to_string()
}

fn navmesh_command(
    In(args): CommandArgs,
) -> String {
    "ok".to_string()
}
