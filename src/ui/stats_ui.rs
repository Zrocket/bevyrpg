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
    player_query: Query<(&Level, &Experience, &Speed, &Corruption, &Matter, &Mind, &Luck, &Maneuver), With<Player>>
) {
    if let Ok((level, experience, speed, corruption, matter, mind, luck, maneuver)) = player_query.single() {
        let level_node = commands.spawn((
            Node {
                ..default()
            },
            Text::new(format!("Level: {}", level.0)),
        )).id();
        let experience_node = commands.spawn((
            Node {
                ..default()
            },
            Text::new(format!("Experience: {}", experience.0)),
        )).id();
        let speed_node = commands.spawn((
            Node {
                ..default()
            },
            Text::new(format!("Speed: {}", speed.0)),
        )).id();
        let corruption_node = commands.spawn((
            Node {
                ..default()
            },
            Text::new(format!("Corruption: {}", corruption.0)),
        )).id();
        let matter_node = commands.spawn((
            Node {
                ..default()
            },
            Text::new(format!("Matter: {}", matter.0)),
        )).id();
        let mind_node = commands.spawn((
            Node {
                ..default()
            },
            Text::new(format!("Mind: {}", mind.0)),
        )).id();
        let luck_node = commands.spawn((
            Node {
                ..default()
            },
            Text::new(format!("Luck: {}", luck.0)),
        )).id();
        let maneuver_node = commands.spawn((
            Node {
                ..default()
            },
            Text::new(format!("Maneuver: {}", maneuver.0)),
        )).id();
        commands.spawn((
                floating_window_root("Stats".to_string(),
                    (
                        Node {
                            flex_grow: 1.,
                            flex_direction: bevy::ui::FlexDirection::Column,
                            overflow: Overflow { x: bevy::ui::OverflowAxis::Hidden, y: bevy::ui::OverflowAxis::Hidden },
                            ..default()
                        },
                    ),
                    ),
        ))
        .add_child(level_node)
        .add_child(experience_node)
        .add_child(speed_node)
        .add_child(corruption_node)
        .add_child(matter_node)
        .add_child(mind_node)
        .add_child(luck_node)
        .add_child(maneuver_node);
    }
}
