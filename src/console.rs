use bevy::prelude::*;
use bevy_console::{ConsolePlugin, ConsoleConfiguration, reply, AddConsoleCommand, ConsoleCommand, PrintConsoleLine};
//use bevy_simple_text_input::{TextInputSubmitEvent, TextInputSystem};
use clap::Parser;

/*#[derive(Component, Default)]
pub struct Console {
    pub history: Vec<String>,
}*/

/// Prints given arguments to the console
#[derive(Parser, ConsoleCommand)]
#[command(name = "log")]
struct LogCommand {
    /// Message to print
    msg: String,
    /// Number of times to print message
    num: Option<i64>,
}

/// Example command
#[derive(Parser, ConsoleCommand)]
#[command(name = "example")]
struct ExampleCommand { /// Some message
    msg: String,
}

/// Kills the given character
#[derive(Parser, ConsoleCommand)]
#[command(name = "kill")]
struct KillCommand {
    entity: String,
}

/// Kills the player
#[derive(Parser, ConsoleCommand)]
#[command(name = "suicide")]
struct SuicideCommand;

/// Toggles god mode
#[derive(Parser, ConsoleCommand)]
#[command(name = "god")]
struct GodCommand;

/// Toggles noclip mode
#[derive(Parser, ConsoleCommand)]
#[command(name = "noclip")]
struct NoclipCommand;

/// Spawns the given entity
#[derive(Parser, ConsoleCommand)]
#[command(name = "spawn")]
struct SpawnCommand {
    entity: String,
}

/// Toggles fps counter
#[derive(Parser, ConsoleCommand)]
#[command(name = "fps")]
struct FpsCommand;

/// Set player speed
#[derive(Parser, ConsoleCommand)]
#[command(name = "speed")]
struct SpeedCommand {
    value: u8,
}

/// Set player gravity
#[derive(Parser, ConsoleCommand)]
#[command(name = "gravity")]
struct GravityCommand {
    value: u8,
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "time")]
struct TimeCommand {
    value: String,
}


/// Set the weather
#[derive(Parser, ConsoleCommand)]
#[command(name = "weather")]
struct WeatherCommand {
    value: String,
}


/// Saves the game
#[derive(Parser, ConsoleCommand)]
#[command(name = "save")]
struct SaveCommand;

/// Reloads entities
#[derive(Parser, ConsoleCommand)]
#[command(name = "reload")]
struct ReloadCommand;

/// Set the field of view
#[derive(Parser, ConsoleCommand)]
#[command(name = "fov")]
struct FovCommand {
    value: u8,
}

/// Set the volume
#[derive(Parser, ConsoleCommand)]
#[command(name = "volume")]
struct VolumeCommand {
    value: u8,
}

/// Toggles HUD display
#[derive(Parser, ConsoleCommand)]
#[command(name = "hud")]
struct HudCommand;

/// Gives the specified item
#[derive(Parser, ConsoleCommand)]
#[command(name = "give")]
struct GiveCommand {
    item: String,
}

/// Activates specified event
#[derive(Parser, ConsoleCommand)]
#[command(name = "event")]
struct EventCommand {
    event: String,
}

pub struct MyConsolePlugin;

impl Plugin for MyConsolePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ConsolePlugin)
            .insert_resource(ConsoleConfiguration {
                // override config here
                ..Default::default()
            })
            .add_console_command::<ExampleCommand, _>(example_command)
            .add_console_command::<LogCommand, _>(log_command);
            //.add_systems(Update, write_to_console.after(ConsoleSet::ConsoleUI));
    }
}


fn example_command(mut log: ConsoleCommand<ExampleCommand>) {
    info!("Example command");
    if let Some(Ok(ExampleCommand { msg: _ })) = log.take() {
        // handle command
    }
}


fn log_command(mut log: ConsoleCommand<LogCommand>) {
    info!("Log command");
    if let Some(Ok(LogCommand { msg, num })) = log.take() {
        let repeat_count = num.unwrap_or(1);

        for _ in 0..repeat_count {
            reply!(log, "{msg}");
        }

        log.ok();
    }
}

fn _write_to_console(mut console_line: EventWriter<PrintConsoleLine>) {
    console_line.send(PrintConsoleLine::new("Hello".into()));
}
