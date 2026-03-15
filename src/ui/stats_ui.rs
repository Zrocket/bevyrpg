use bevy::{ecs::{entity::Entity, event::EntityEvent, observer::On, query::With, system::{Commands, Query, Res, ResMut}}, prelude::{Node, Plugin}, state::{state::{NextState, State}, state_scoped::DespawnOnExit}, ui::{Overflow, widget::Text}, utils::default};

use crate::{Experience, Level, Player, UiState, widgets::floating_windows::floating_window_root};

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
    _trigger: On<DisplayStatsEvent>,
    mut commands: Commands,
    player_query: Query<(&Level, &Experience), With<Player>>,
    menu_state: Res<State<UiState>>,
    mut menu_state_setter: ResMut<NextState<UiState>>,
) {
    if *menu_state == UiState::Stats {
        return;
    }

    menu_state_setter.set(UiState::Stats);

    if let Ok((level, experience)) = player_query.single() {
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

        commands.spawn((
                DespawnOnExit(UiState::Stats),
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
        .add_child(experience_node);
    }
}
