use bevy::prelude::*;
use bevy_simple_text_input::{TextInputSubmitEvent, TextInputSystem};

#[derive(Component, Default)]
pub struct Console {
    pub history: Vec<String>,
}

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, listener.after(TextInputSystem));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Console {
        ..default()
    });
}

fn listener(
    mut events: EventReader<TextInputSubmitEvent>,
    mut console_query: Query<&mut Console>,
    ) {
    for event in events.read() {
        info!("{:?} submitted: {}", event.entity, event.value);
        if let Ok(mut console) = console_query.get_single_mut() {
            console.history.push(event.value.clone());
        }
    }
}
