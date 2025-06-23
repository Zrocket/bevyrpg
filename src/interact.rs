use bevy::{ecs::system::SystemId, prelude::*};
use avian_pickup::prelude::*;

#[bevy_trait_query::queryable]
pub trait Interaction {
    fn interact( &self,
        commands: &mut Commands,
        entity: Entity,
        prop: Entity,
    );
}

#[bevy_trait_query::queryable]
pub trait Inspectable {
    fn inspect( &self,
        commands: &mut Commands,
        entity: Entity,
        prop: Entity,
    );
}

#[derive(Event)]
pub struct InteractEvent {
    pub actor: Entity,
    pub target: Entity,
}

#[derive(Event)]
pub struct InspectEvent {
    pub actor: Entity,
    pub target: Entity,
}

pub struct InteractPlugin;

impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        trace!("InteractPlugin build");
        app.add_plugins(AvianPickupPlugin::default())
            .add_event::<InteractEvent>()
            .add_event::<InspectEvent>();
    }
}

