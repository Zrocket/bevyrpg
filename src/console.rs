use bevy::prelude::*;
use bevy_console::{ConsolePlugin, ConsoleConfiguration, reply, AddConsoleCommand, ConsoleCommand, PrintConsoleLine, ConsoleSet};
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

#[derive(Parser, ConsoleCommand)]
#[command(name = "example")]
struct KillCommand {
    msg: String,
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
    if let Some(Ok(ExampleCommand { msg })) = log.take() {
        // handle command
    }
}


fn log_command(mut log: ConsoleCommand<LogCommand>) {
    if let Some(Ok(LogCommand { msg, num })) = log.take() {
        let repeat_count = num.unwrap_or(1);

        for _ in 0..repeat_count {
            reply!(log, "{msg}");
        }

        log.ok();
    }
}

fn write_to_console(mut console_line: EventWriter<PrintConsoleLine>) {
    console_line.send(PrintConsoleLine::new("Hello".into()));
}
