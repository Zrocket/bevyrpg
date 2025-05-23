use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

use crate::GameState;

#[derive(Debug, Component, Reflect, Eq, PartialEq, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct YarnNode(pub String);

impl Default for YarnNode {
    fn default() -> Self {
        YarnNode("Start".to_string())
    }
}

#[derive(Event)]
pub struct DialogEvent {
    pub actor: Entity,
}

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                    YarnSpinnerPlugin::new(),
                    ExampleYarnSpinnerDialogueViewPlugin::new(),
                    ))
            .register_type::<YarnNode>()
            .add_event::<DialogEvent>()
            .add_systems(Update, spawn_dialog_runner.run_if(resource_added::<YarnProject>))
            .add_systems(Update, read_dialog.run_if(in_state(GameState::Gameplay)));
    }
}

fn spawn_dialog_runner(
    mut commands: Commands,
    project: Res<YarnProject>
    ) {
    trace!("System: spawn_dialog_runner");
    let dialog_runner = project.create_dialogue_runner(&mut commands);
    commands.spawn(dialog_runner);
}

fn read_dialog(
    mut dialog_events: EventReader<DialogEvent>,
    mut dialog_runner: Query<&mut DialogueRunner>,
    dialog_caller_query: Query<(Entity, &YarnNode)>,
) {
    trace!("SYSTEM: read_dialog");

    for event in dialog_events.read() {
        info!("Dialog Event");
        if let Ok((_caller, caller_node)) = dialog_caller_query.get(event.actor) {
            if let Ok(mut dialog_runner) = dialog_runner.single_mut() {
                dialog_runner.stop();
                dialog_runner.start_node(&caller_node.0);
            }
        }
    }
}
