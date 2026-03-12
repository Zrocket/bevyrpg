use bevy::{color::palettes::css::{DARK_GREEN, DARK_VIOLET}, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{UiState, widgets::floating_windows::floating_window_root};

#[derive(Component, Reflect)]
#[require(
    Node {
        flex_grow: 1.,
        ..default()
    }
)]
#[component(on_add = on_ui_quest_add)]
pub struct UiQuest;

fn on_ui_quest_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity);
}

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
    menu_state: Res<State<UiState>>,
    mut menu_state_setter: ResMut<NextState<UiState>>,
) {
    trace!("OBSERVER: display_quest_event_observer");

    if *menu_state == UiState::QuestLog {
        return;
    }

    menu_state_setter.set(UiState::QuestLog);

    commands.spawn((
        DespawnOnExit(UiState::QuestLog),
floating_window_root("Quest Log".into(),
(
        UiQuest,
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
    ))));
}
