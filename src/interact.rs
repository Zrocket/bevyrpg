use bevy::prelude::*;

use crate::{trade::TradeEvent, DialogEvent, PickUpEvent};

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub enum Interactable {
    #[default]
    Talk,
    Misc,
    Trade,
}

#[derive(Event)]
pub struct InteractEvent {
    pub actor: Entity,
    pub target: Entity,
}

pub struct InteractPlugin;

impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Interactable>()
            .add_event::<InteractEvent>()
            .add_systems(Update, interact_event_handler)
            .register_type::<Interactable>();
    }
}

fn interact_event_handler(
    characters: Query<(Entity, &Interactable)>,
    mut interact_events: EventReader<InteractEvent>,
    mut dialog_event_writer: EventWriter<DialogEvent>,
    mut pick_up_event_writer: EventWriter<PickUpEvent>,
    mut trade_event_writer: EventWriter<TradeEvent>,
    ) {
    trace!("Event Handler: interact_event_handler");
    for event in interact_events.read() {
        if let Ok((target_entity, target_interact)) = characters.get(event.target) {
            match target_interact {
                Interactable::Talk => {
                    info!("Talk Interact event");
                    dialog_event_writer.send(DialogEvent { actor:  target_entity});
                },
                Interactable::Misc => {
                    info!("Misc Interact event");
                    pick_up_event_writer.send(PickUpEvent {
                        actor: event.actor,
                        target: event.target,
                    });
                },
                Interactable::Trade => {
                    trade_event_writer.send(TradeEvent {
                        actor: event.actor,
                        target: event.target
                    });
                }
            }
        }
    }
}
