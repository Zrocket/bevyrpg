use bevy::prelude::*;

#[derive(Component)]
#[relationship(relationship_target = Quests)]
pub struct QuestOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = QuestOf, linked_spawn)]
pub struct Quests(Vec<Entity>);

pub struct QuestPlugin;
impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
       app; 
    }
}
