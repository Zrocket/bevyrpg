use bevy::{color::palettes::css::{DARK_GREEN, DARK_VIOLET}, prelude::*};

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
    _trigger: On<DisplayQuestEvent>,
    mut commands: Commands,
) {
    trace!("OBSERVER: display_quest_event_observer");

    commands.spawn(
floating_window_root("Quest Log".into(),
(
        Node {
            flex_grow: 1.,
            ..default()
        },
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn((
                    Node {
                        flex_grow: 1.,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor::from(DARK_GREEN),
                    Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                        parent.spawn((
                                Node {
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                Text("Active".into())
                        ));
                        parent.spawn((
                                Node {
                                    ..default()
                                },
                                Text("quest 1".into())
                        ));
                        parent.spawn((
                                Node {
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                Text("Inactive".into())
                        ));
                        parent.spawn((
                                Node {
                                    ..default()
                                },
                                Text("quest 2".into())
                        ));
                    })),
            ));
            parent.spawn((
                    Node {
                        flex_grow: 1.,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    //Text("Quest".into()),
                    BackgroundColor::from(DARK_VIOLET),
                    Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                        parent.spawn((
                            Node {
                                align_self: AlignSelf::Center,
                                ..default()
                            },
                            Text("Quest".into()),
                        ));
                        parent.spawn((
                            Node {
                                ..default()
                            },
                            Text("ZZZZZZZZZZZz".into()),
                        ));
                    })),
            ));
        }))
    )));
}
