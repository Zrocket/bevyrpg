use std::marker::PhantomData;

use bevy::{color::palettes::css::DARK_GRAY, ecs::{component::Component, entity::Entity, event::EntityEvent, hierarchy::{ChildSpawner, Children}, observer::On, query::With, spawn::{SpawnRelated, SpawnWith}, system::{Commands, Query}}, prelude::{Node, Plugin}, reflect::enum_debug, ui::{BackgroundColor, Overflow, widget::Text}, utils::default};

use crate::{Corruption, Experience, Health, Hunger, Level, Luck, Maneuver, Matter, Mind, Player, Speed, widgets::floating_windows::floating_window_root};


#[derive(EntityEvent)]
pub struct DisplayStatsEvent {
    pub entity: Entity,
}

pub struct StatsUiPlugin;
impl Plugin for StatsUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app; 
    }
}

pub fn spawn_stats_ui(
    mut commands: Commands,
    entity: Query<Entity, With<Player>>,
) {
    if let Ok(entity) = entity.single() {
        commands.entity(entity).trigger(|entity| DisplayStatsEvent { entity });
    }
}

pub fn display_stats_event_observer(
    trigger: On<DisplayStatsEvent>,
    mut commands: Commands,
    player_query: Query<(&Level, &Experience, &Speed, &Corruption, &Matter, &Mind, &Luck, &Maneuver, &Hunger), With<Player>>
) {
    if let Ok((level, experience, speed, corruption, matter, mind, luck, maneuver, hunger)) = player_query.single() {
        commands.spawn((
                floating_window_root("EQUIPT TEST".to_string(),
                    (
                        Node {
                            flex_grow: 1.,
                            flex_direction: bevy::ui::FlexDirection::Column,
                            overflow: Overflow { x: bevy::ui::OverflowAxis::Hidden, y: bevy::ui::OverflowAxis::Hidden },
                            ..default()
                        },
                        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                            parent.spawn((
                                    Node {
                                        ..default()
                                    },
                                    Text::new(format!("Level")),
                            ));
                            parent.spawn((
                                    Node {
                                        ..default()
                                    },
                                    Text::new("Experience"),
                            ));
                            parent.spawn((
                                    Node {
                                        ..default()
                                    },
                                    Text::new("Speed"),
                            ));
                            parent.spawn((
                                    Node {
                                        ..default()
                                    },
                                    Text::new("Corruption"),
                            ));
                            parent.spawn((
                                    Node {
                                        ..default()
                                    },
                                    Text::new("Matter"),
                            ));
                            parent.spawn((
                                    Node {
                                        ..default()
                                    },
                                    Text::new("Mind"),
                            ));
                            parent.spawn((
                                    Node {
                                        ..default()
                                    },
                                    Text::new("Luck"),
                            ));
                            parent.spawn((
                                    Node {
                                        ..default()
                                    },
                                    Text::new("Maneuver"),
                            ));
                            parent.spawn((
                                    Node {
                                        ..default()
                                    },
                                    Text::new("Hunger"),
                            ));
                        })),
                    ),
                    ),
        ));
    }
}
