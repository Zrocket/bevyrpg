use bevy::{color::palettes::css::DARK_GREEN, prelude::*};

use crate::widgets::floating_windows::floating_window_root;

#[derive(Component, Reflect)]
pub struct UiQuest;

#[derive(EntityEvent)]
pub struct DisplayQuestEvent {
    pub entity: Entity,
}

pub struct QuestUiPlugin;
impl Plugin for QuestUiPlugin {
    fn build(&self, app: &mut App) {
       app; 
    }
}

pub fn display_quest_event_observer(
    trigger: On<DisplayQuestEvent>,
    mut commands: Commands,
) {
    trace!("OBSERVER: display_quest_event_observer");

    commands.spawn((
            floating_window_root("Quest Log".into(),
                (
                    Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                        parent.spawn((
                                Node {
                                    ..default()
                                },
                                BackgroundColor::from(DARK_GREEN),
                        ));
                    }))
            ))
    ));
}
