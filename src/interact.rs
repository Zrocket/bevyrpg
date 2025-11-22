use bevy::prelude::*;
use avian_pickup::prelude::*;

#[derive(Component)]
pub struct Interactable;

#[derive(EntityEvent)]
pub struct InteractionEvent {
    pub entity: Entity,
    pub actor: Entity,
}

#[derive(EntityEvent)]
pub struct InspectEvent {
    pub entity: Entity,
    pub actor: Entity,
}

pub struct InteractPlugin;
impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        trace!("InteractPlugin build");
        app.add_plugins(AvianPickupPlugin::default());
    }
}
