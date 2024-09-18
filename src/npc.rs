use bevy::prelude::*;
use super::Dialog;


#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Npc {
    name: String,
    dialog: Dialog,
}

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Npc>();
    }
}

fn npc_interact(
    npcs: Query<&mut Npc>,
    interacted: EventReader<InteractEvent>,
) {
    trace!("Event Handler: npc_interact");
    for event in interacted.iter() {
    }
}
