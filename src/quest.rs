use bevy::prelude::*;

use crate::{Inventory, ItemDetails, MiscItem, Player};

#[derive(Component)]
#[require(
    QuestState,
)]
pub struct Quest {
    pub description: String,
}

#[derive(Resource, Reflect, Debug, Deref, Default, PartialEq)]
#[reflect(Resource)]
pub struct CurrentQuest(Option<Entity>);

#[derive(Component)]
pub struct CompleteddQuest;

#[derive(EntityEvent)]
pub struct AssignQuestEvent {
    entity: Entity,
}

#[derive(Component)]
#[relationship(relationship_target = SubQuests)]
pub struct SubQuestOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = SubQuestOf)]
pub struct SubQuests(Vec<Entity>);

#[derive(Component)]
pub struct FetchQuest {
    current: i32,
    pub ammount: i32,
    pub item: String,
}

impl FetchQuest {
    pub fn new(ammount: i32, item: String) -> Self {
        Self { current: 0, ammount, item }
    }
}

#[derive(Component)]
pub struct KillQuest {
    target: Entity,
}

pub struct FindQuest;

pub struct EscortQuest;

#[derive(Default, Component)]
pub enum QuestState {
    #[default]
    Incomplete,
    Completed,
    Botched,
}

#[derive(Component)]
#[relationship(relationship_target = Quests)]
pub struct QuestOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = QuestOf, linked_spawn)]
pub struct Quests(Vec<Entity>);

#[derive(Component)]
pub struct NextQuest(pub Entity);

#[derive(Component)]
pub struct LoggedQuest;

pub struct QuestPlugin;
impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(Update, check_fetch_quest);
    }
}

pub fn assign_quest_event_observer<T: Sync + Send + 'static>(
    trigger: On<AssignQuestEvent>,
) {
}

pub fn check_fetch_quest(
    mut quest_query: Query<(Entity, &mut FetchQuest, &mut QuestState), With<Quest>>,
    inventory_query: Query<&Inventory, With<Player>>,
    item_details_query: Query<&ItemDetails, With<MiscItem>>,
) {
    let mut count = 0;
    for mut quest in quest_query.iter_mut() {
        if let Ok(inventory) = inventory_query.single() {
            for item in inventory.iter() {
                if let Ok(details) = item_details_query.get(item)
                && details.name == quest.1.item {
                    count += 1;
                }
            }
            quest.1.current = count;
            if quest.1.ammount <= quest.1.current {
                *quest.2 = QuestState::Completed;
            } else {
                *quest.2 = QuestState::Incomplete;
            }
        }
    }
}
