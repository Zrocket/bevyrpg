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

#[derive(EntityEvent)]
pub struct EquiptEvent {
    pub entity: Entity,
    pub actor: Entity,
}

#[derive(EntityEvent)]
pub struct UseEvent {
    pub entity: Entity,
    pub actor: Entity,
}

#[derive(EntityEvent)]
pub struct DropEvent {
    pub entity: Entity,
    pub actor: Entity,
}

#[derive(Message)]
pub struct UnInspectMessage;

pub struct InteractPlugin;
impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        trace!("InteractPlugin build");
        app
            .add_plugins(AvianPickupPlugin::default())
            .add_message::<UnInspectMessage>();
    }
}
