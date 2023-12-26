use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct Interactable;

#[derive(Event)]
pub struct InteractEvent {
    actor: Entity,
    target: Entity,
}

pub struct InteractPlugin;

impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Interactable>()
            .add_event::<InteractEvent>();
    }
}
