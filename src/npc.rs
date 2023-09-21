
use bevy::prelude::*;


#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Npc {
    name: String,
    dialog: String,
}

pub struct NpcPlugin {
}

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Npc>();
    }
}

fn npc_interact(
    npcs: Query<&mut Npc>,
) {
    for npc in npcs.iter
}
