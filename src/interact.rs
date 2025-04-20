use avian3d::prelude::{CollisionLayers, LayerMask};
use bevy::{ecs::system::QueryLens, prelude::*};
use avian_pickup::{prelude::*, prop::HeldProp};

//use crate::{trade::TradeEvent, DialogEvent, PickUpEvent};
use crate::{trade::TradeEvent, CollisionLayer, SitEvent};

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub enum Interactable {
    #[default]
    Talk,
    Misc,
    Trade,
    Chair,
    Door,
    Button,
    Container,
    Item,
    Read,
    Consume,
}

#[bevy_trait_query::queryable]
pub trait Interaction {
    fn interact(
        &self,
        entity: &Entity,
        query: QueryLens<&Interactable>
        ) -> Option<AvianPickupInput>;
}

#[derive(Event)]
pub struct InteractEvent {
    pub actor: Entity,
    pub target: Entity,
}

pub struct InteractPlugin;

impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        trace!("InteractPlugin build");
        app.register_type::<Interactable>()
            .add_plugins(AvianPickupPlugin::default())
            .add_event::<InteractEvent>()
//            .add_systems(Update, interact_event_handler)
            .register_type::<Interactable>();
    }
}

fn interact_event_handler(
    //characters: Query<(Entity, &Interactable)>,
    characters: Query<&Interactable>,
    mut interact_events: EventReader<InteractEvent>,
    //mut dialog_event_writer: EventWriter<DialogEvent>,
    mut sit_event_writer: EventWriter<SitEvent>,
    mut trade_event_writer: EventWriter<TradeEvent>,
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
    mut collision_layer_query: Query<&mut CollisionLayers>,
    held_prop_query: Query<&HeldProp>,
) {
    trace!("Event Handler: interact_event_handler");
    for event in interact_events.read() {
        if let Ok(_held_prop) = held_prop_query.get_single() {
            avian_pickup_input_writer.send( AvianPickupInput { actor: event.actor, action: AvianPickupAction::Drop } );
        }
        if let Ok(target_interact) = characters.get(event.target) {
            match target_interact {
                Interactable::Talk => {
                    info!("Talk Interact event");
                    //dialog_event_writer.send(DialogEvent { actor:  target_entity});
                }
                Interactable::Misc => {
                    info!("Misc Interact event");
                    avian_pickup_input_writer.send(AvianPickupInput { actor: event.actor, action: AvianPickupAction::Pull });
                    if let Ok(mut layer) = collision_layer_query.get_mut(event.target) {
                        layer.filters.remove(CollisionLayer::Player);
                    }
                }
                Interactable::Trade => {
                    info!("Trade Interact event");
                    trade_event_writer.send(TradeEvent {
                        actor: event.actor,
                        target: event.target,
                    });
                }
                Interactable::Chair => {
                    //sit_event_writer.send(SitEvent { actor: event.actor, target: event.target });
                }
                Interactable::Door => {
                }
                Interactable::Button => {
                }
                Interactable::Container => {
                }
                Interactable::Item => {
                }
                Interactable::Read => {
                }
                Interactable::Consume => {
                }
            }
        }
    }
}
