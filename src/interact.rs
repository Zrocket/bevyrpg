use bevy::prelude::*;

#[derive(Component, Default)]
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

#[derive(EntityEvent)]
pub struct PickupEvent {
    pub entity: Entity,
    pub actor: Entity,
}

pub fn drop_event_observer<T: Component>(
    trigger: On<DropEvent>,
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    trace!("OBSERVER: drop_event_observer");
    if let Ok(entity) = query.get(trigger.entity) {
        commands.entity(entity);
    }
}

#[derive(Message)]
pub struct UnInspectMessage;

pub struct InteractPlugin;
impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        trace!("InteractPlugin build");
        app
            .add_message::<UnInspectMessage>();
    }
}
