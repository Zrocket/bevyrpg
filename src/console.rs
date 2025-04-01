use bevy::prelude::*;

#[derive(Event)]
pub struct ConsoleInputEvent {
    pub input: String,
}

#[derive(Component)]
pub struct Console {
    pub history: Vec<String>,
}

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {}
}
